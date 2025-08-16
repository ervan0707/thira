use crate::config::{Config, ScriptConfig};
use crate::error::HookError;
use crate::error::Result;

use std::collections::HashMap;

pub struct ScriptManager {
    config: Config,
}

impl ScriptManager {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;

        Ok(Self { config })
    }

    pub fn add_script(&mut self, name: String, command: String) -> Result<()> {
        self.config.add_script(name, command)
    }

    pub fn run_script(&self, name: &str) -> Result<()> {
        use crossterm::{
            cursor, execute,
            style::{Color, Print, SetForegroundColor},
            terminal::{self, Clear, ClearType},
            event::{self, Event, KeyCode, KeyEvent},
        };
        use parking_lot::Mutex;
        use std::io::stdout;
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        use std::sync::Arc;
        use std::thread::{self, JoinHandle};
        use std::time::{Duration, Instant};

        // Create a mutex-wrapped stdout for thread-safe access
        let stdout = Arc::new(Mutex::new(stdout()));
        
        // Create cancellation flag for Ctrl+C handling
        let cancelled = Arc::new(AtomicBool::new(false));

        // Initialize terminal
        {
            let mut stdout = stdout.lock();
            execute!(stdout, terminal::EnterAlternateScreen)?;
        }
        terminal::enable_raw_mode()?;

        // Start timing
        let start_time = Instant::now();

        // Get the script configuration
        let script_config =
            self.config
                .scripts
                .get(name)
                .ok_or_else(|| HookError::ScriptExecutionError {
                    script_name: name.to_string(),
                    reason: "Script not found".to_string(),
                })?;

        if script_config.commands.is_empty() {
            return Ok(());
        }

        let commands = Arc::new(script_config.commands.clone());
        let mut handles: Vec<JoinHandle<Result<()>>> = vec![]; 

        // Calculate screen layout
        let (term_width, term_height) = terminal::size()?;
        let section_height = term_height / commands.len() as u16;

        // Clear screen and show initial layout
        {
            let mut stdout = stdout.lock();
            execute!(stdout, Clear(ClearType::All))?;

            for (idx, cmd) in commands.iter().enumerate() {
                let y_pos = idx as u16 * section_height;
                execute!(
                    stdout,
                    cursor::MoveTo(0, y_pos),
                    SetForegroundColor(Color::Cyan),
                    Print("┌".to_string() + &"─".repeat(term_width as usize - 2) + "┐"),
                    cursor::MoveTo(2, y_pos),
                    Print(format!(
                        " Command {}: {} {}",
                        idx + 1,
                        cmd.command,
                        cmd.description
                            .as_ref()
                            .map(|d| format!("({})", d))
                            .unwrap_or_default()
                    )),
                )?;
            }
        }

        // For parallel execution with max threads control
        let active_threads = Arc::new(AtomicUsize::new(0));

        // Start Ctrl+C detection thread
        let cancelled_clone = Arc::clone(&cancelled);
        let stdout_clone = Arc::clone(&stdout);
        let ctrl_c_handle = thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                    if let Ok(Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers, .. })) = event::read() {
                        if modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                            cancelled_clone.store(true, Ordering::SeqCst);
                            let mut stdout = stdout_clone.lock();
                            let _ = execute!(
                                stdout,
                                cursor::MoveTo(0, 0),
                                SetForegroundColor(Color::Red),
                                Print("⚠️  Cancelling execution... (Ctrl+C detected)")
                            );
                            break;
                        }
                    }
                }
                if cancelled_clone.load(Ordering::SeqCst) {
                    break;
                }
            }
        });

        // Execute commands
        for (idx, cmd) in commands.iter().enumerate() {
            // Check for cancellation before starting new commands
            if cancelled.load(Ordering::SeqCst) {
                break;
            }

            let cmd = cmd.clone();
            let name = name.to_string();
            let stdout = Arc::clone(&stdout);
            let y_pos = idx as u16 * section_height;
            // let term_width = term_width;
            let active_threads = Arc::clone(&active_threads);
            let cancelled_flag = Arc::clone(&cancelled);

            // If parallel execution is disabled, wait for previous command to complete
            if !script_config.parallel {
                // Take ownership and join the last handle if it exists
                if let Some(last_handle) = handles.pop() {
                    match last_handle
                        .join()
                        .map_err(|_| HookError::ScriptExecutionError {
                            script_name: name.clone(),
                            reason: "Thread panicked".to_string(),
                        })? {
                        Ok(_) => (),
                        Err(e) => {
                            // If sequential execution fails, return immediately
                            let mut stdout = stdout.lock();
                            execute!(
                                stdout,
                                cursor::MoveTo(0, term_height - 2),
                                SetForegroundColor(Color::Red),
                                Print(format!("Error: {}", e))
                            )?;
                            return Err(HookError::ScriptExecutionError {
                                script_name: name.clone(),
                                reason: "Previous command failed".to_string(),
                            });
                        }
                    }
                }
            } else {
                // Wait if we've reached max threads
                while script_config.parallel
                    && active_threads.load(Ordering::SeqCst) >= script_config.max_threads
                {
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            }

            active_threads.fetch_add(1, Ordering::SeqCst);

            let handle = thread::spawn(move || -> Result<()> {
                let command_start = Instant::now();

                // Check for cancellation before starting
                if cancelled_flag.load(Ordering::SeqCst) {
                    active_threads.fetch_sub(1, Ordering::SeqCst);
                    return Err(HookError::ScriptExecutionError {
                        script_name: name,
                        reason: "Execution cancelled".to_string(),
                    });
                }

                // Build command with working directory and environment variables
                let mut child = std::process::Command::new("sh");
                child.arg("-c").arg(&cmd.command);

                // Set working directory if specified
                if let Some(dir) = &cmd.working_dir {
                    child.current_dir(dir);
                }

                // Set environment variables
                for (key, value) in &cmd.env {
                    child.env(key, value);
                }

                // Configure stdio
                let mut child = child
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()
                    .map_err(|e| HookError::ScriptExecutionError {
                        script_name: name.clone(),
                        reason: e.to_string(),
                    })?;

                let mut current_line = y_pos + 1;

                // Process output in real-time
                if let Some(child_stdout) = child.stdout.take() {
                    let reader = std::io::BufReader::new(child_stdout);
                    for line in std::io::BufRead::lines(reader).flatten() {
                        // Check for cancellation during output processing
                        if cancelled_flag.load(Ordering::SeqCst) {
                            let _ = child.kill();
                            active_threads.fetch_sub(1, Ordering::SeqCst);
                            return Err(HookError::ScriptExecutionError {
                                script_name: name,
                                reason: "Execution cancelled".to_string(),
                            });
                        }

                        let mut stdout = stdout.lock();
                        execute!(
                            stdout,
                            cursor::MoveTo(1, current_line),
                            Clear(ClearType::CurrentLine),
                            Print(&line)
                        )?;
                        current_line = (current_line + 1).min(y_pos + section_height - 1);
                    }
                }

                // Check for cancellation before waiting for process completion
                if cancelled_flag.load(Ordering::SeqCst) {
                    let _ = child.kill();
                    active_threads.fetch_sub(1, Ordering::SeqCst);
                    return Err(HookError::ScriptExecutionError {
                        script_name: name,
                        reason: "Execution cancelled".to_string(),
                    });
                }

                let status = child.wait().map_err(|e| HookError::ScriptExecutionError {
                    script_name: name.clone(),
                    reason: e.to_string(),
                })?;

                if !status.success() {
                    active_threads.fetch_sub(1, Ordering::SeqCst);
                    return Err(HookError::ScriptExecutionError {
                        script_name: name,
                        reason: format!("Command '{}' failed with status {}", cmd.command, status),
                    });
                }

                let duration = command_start.elapsed();
                let mut stdout = stdout.lock();
                execute!(
                    stdout,
                    cursor::MoveTo(term_width - 20, y_pos),
                    SetForegroundColor(Color::Green),
                    Print(format!("[{:.2?}]", duration))
                )?;

                active_threads.fetch_sub(1, Ordering::SeqCst);
                Ok(())
            });

            handles.push(handle);
        }

        // Wait for all remaining threads to complete
        let mut all_successful = true;
        let was_cancelled = cancelled.load(Ordering::SeqCst);
        
        while let Some(handle) = handles.pop() {
            match handle.join().map_err(|_| HookError::ScriptExecutionError {
                script_name: name.to_string(),
                reason: "Thread panicked".to_string(),
            })? {
                Ok(_) => (),
                Err(e) => {
                    all_successful = false;
                    let mut stdout = stdout.lock();
                    execute!(
                        stdout,
                        cursor::MoveTo(0, term_height - 2),
                        SetForegroundColor(Color::Red),
                        Print(format!("Error: {}", e))
                    )?;
                    if !script_config.parallel {
                        break; // Exit early for sequential execution
                    }
                }
            }
        }

        // Signal the Ctrl+C thread to stop and wait for it
        cancelled.store(true, Ordering::SeqCst);
        let _ = ctrl_c_handle.join();

        // Show final status
        let total_duration = start_time.elapsed();
        {
            let mut stdout = stdout.lock();
            execute!(
                stdout,
                cursor::MoveTo(0, term_height - 1),
                SetForegroundColor(if was_cancelled {
                    Color::Yellow
                } else if all_successful {
                    Color::Green
                } else {
                    Color::Red
                }),
                Print(format!(
                    "Execution {} in {:.2?} ({})",
                    if was_cancelled {
                        "cancelled"
                    } else if all_successful {
                        "completed"
                    } else {
                        "failed"
                    },
                    total_duration,
                    if script_config.parallel {
                        format!("parallel, max {} threads", script_config.max_threads)
                    } else {
                        "sequential".to_string()
                    }
                ))
            )?;

            // Wait for user input before closing
            execute!(stdout, cursor::MoveTo(0, term_height))?;
        }

        terminal::disable_raw_mode()?;
        {
            let mut stdout = stdout.lock();
            execute!(stdout, terminal::LeaveAlternateScreen)?;
        }

        if was_cancelled || !all_successful {
            return Err(HookError::ScriptExecutionError {
                script_name: name.to_string(),
                reason: if was_cancelled {
                    "Execution was cancelled by user".to_string()
                } else {
                    "One or more commands failed".to_string()
                },
            });
        }

        Ok(())
    }

    pub fn remove_script(&mut self, name: &str) -> Result<()> {
        self.config.remove_script(name)
    }

    pub fn get_scripts(&self) -> &HashMap<String, ScriptConfig> {
        &self.config.scripts
    }
}
