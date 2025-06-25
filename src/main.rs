mod config;
mod error;
mod git;
mod hooks;
mod linter;
mod scripts;

use clap::{Command, CommandFactory, Parser, Subcommand};
use colored::*;
use error::{HookError, Result};
use std::process;

#[derive(Parser)]
#[command(
    author = "Ervan <ervanroot@gmail.com>",
    version,
    about = "Git hooks manager and commit message linter",
    long_about = "A tool for managing git hooks and validating commit messages.\nProvides hooks management, scripts automation, and commit message linting."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Git hooks management commands
    #[command(subcommand)]
    Hooks(HooksCommands),

    /// Script management commands
    #[command(subcommand)]
    Scripts(ScriptsCommands),

    /// Commit message validation commands
    #[command(subcommand)]
    Commit(CommitCommands),
}

#[derive(Subcommand)]
enum HooksCommands {
    #[command(about = "Initialize hooks configuration")]
    Init,

    #[command(about = "Install git hooks")]
    Install,

    #[command(about = "Uninstall git hooks")]
    Uninstall,

    #[command(about = "Add a new hook")]
    Add {
        name: String,
        command: String,
        #[arg(last = true)]
        args: Vec<String>,
    },

    #[command(about = "Show current hooks path configuration")]
    ShowPath,

    #[command(about = "Reset hooks path to default (.git/hooks)")]
    ResetPath,

    #[command(about = "Clean up all hooks and configuration")]
    Clean,

    #[command(about = "List all configured hooks")]
    List,
}

#[derive(Subcommand)]
enum ScriptsCommands {
    #[command(about = "Add a new script")]
    Add { name: String, command: String },

    #[command(about = "Remove a script")]
    Remove { name: String },

    #[command(about = "List all scripts")]
    List,

    #[command(about = "Run a script")]
    Run { name: String },
}

#[derive(Subcommand)]
enum CommitCommands {
    #[command(about = "Validate a commit message")]
    Validate {
        #[arg(help = "Path to commit message file")]
        message_file: String,
    },
}

fn print_error(error: &HookError) {
    println!("\n{}", "ERROR:".red().bold());
    match error {
        HookError::GitNotFound => {
            println!("Git repository not found in current directory");
            println!("\n{}", "SUGGESTIONS:".yellow());
            println!("1. Make sure you're in a git repository");
            println!("2. Run 'git init' if this is a new project");
        }
        HookError::FileError { path, source } => {
            println!("Failed to access: {}", path.display());
            println!("Cause: {}", source);
            println!("\n{}", "SUGGESTIONS:".yellow());
            println!("1. Check file permissions");
            println!("2. Make sure the path exists");
        }
        HookError::LintError { kind } => {
            println!("{}", kind);
        }
        _ => println!("{}", error),
    }
}

fn print_command_tree() {
    println!("{}", "Available Commands:".blue().bold());
    print_subcommands(&Cli::command(), 0, true);
    println!(
        "\nRun {} for more details",
        "'thira <command> --help'".cyan()
    );
}

fn print_subcommands(cmd: &Command, depth: usize, is_last: bool) {
    let prefix = if depth == 0 {
        String::new()
    } else {
        format!(
            "{}{}",
            "│  ".repeat(depth - 1),
            if is_last { "└─ " } else { "├─ " }
        )
    };

    // Skip printing the root command
    if depth > 0 {
        let name = cmd.get_name();
        let about = cmd.get_about().unwrap_or_default();
        if depth == 1 {
            println!("{}{}", prefix, name.yellow().bold());
        } else {
            println!("{}{:<12} {}", prefix, name, about);
        }
    }

    let subcommands: Vec<_> = cmd.get_subcommands().collect();
    let count = subcommands.len();

    for (idx, subcmd) in subcommands.iter().enumerate() {
        print_subcommands(subcmd, depth + 1, idx == count - 1);
    }
}

fn main() {
    // If no arguments provided, show the command tree
    if std::env::args().len() == 1 {
        print_command_tree();
        process::exit(0);
    }
    match run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            print_error(&e);
            process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let mut hook_manager = hooks::HookManager::new().inspect_err(|e| match e {
        HookError::GitNotFound => {
            println!(
                "{}",
                "Not a git repository. Please run 'git init' first.".red()
            );
        }
        _ => print_error(e),
    })?;

    let mut script_manager = scripts::ScriptManager::new()?;

    match cli.command {
        Commands::Hooks(cmd) => match cmd {
            HooksCommands::Init => {
                let output = std::process::Command::new("git")
                    .args(["rev-parse", "--git-dir"])
                    .output()?;

                if !output.status.success() {
                    println!(
                        "{}",
                        "Not a git repository. Please run 'git init' first.".red()
                    );
                    return Ok(());
                }

                let config = config::Config::default();
                config.save()?;
                println!("{}", "✓ Hooks configuration initialized.".green());
                println!("\n{}", "Next steps:".blue().bold());
                println!("1. Review the generated hooks.yaml configuration");
                println!("2. Run 'thira hooks install' to install the hooks");
            }
            HooksCommands::Install => {
                hook_manager.install_hooks()?;
                println!("{}", "✓ Hooks installed successfully.".green());
                println!("\nHooks will now run automatically on git operations.");
            }
            HooksCommands::Uninstall => {
                hook_manager.uninstall_hooks()?;
                println!("{}", "✓ Hooks uninstalled successfully.".green());
            }
            HooksCommands::Add {
                name,
                command,
                args,
            } => {
                hook_manager.add_hook(name.clone(), command, args)?;
                println!(
                    "{}",
                    format!("✓ Hook '{}' added successfully.", name).green()
                );
            }
            HooksCommands::ShowPath => {
                let path = hook_manager.get_hooks_path()?;
                println!("Current hooks path: {}", path);
            }
            HooksCommands::ResetPath => {
                hook_manager.unset_hooks_path()?;
                println!("{}", "✓ Reset hooks path to default (.git/hooks)".green());
            }
            HooksCommands::Clean => {
                hook_manager.uninstall_hooks()?;
                println!("{}", "✓ Cleaned up all hooks and configuration.".green());
            }
            HooksCommands::List => {
                println!("{}", "Configured Hooks:".blue().bold());
                let hooks = hook_manager.get_hooks();
                if hooks.is_empty() {
                    println!("  No hooks configured");
                } else {
                    for (name, hooks) in hooks {
                        println!("  {}:", name.yellow());
                        for hook in hooks {
                            let args_str = if hook.args.is_empty() {
                                String::new()
                            } else {
                                format!(" {}", hook.args.join(" "))
                            };
                            println!("    - {}{}", hook.command, args_str);
                        }
                    }
                }
            }
        },

        Commands::Scripts(cmd) => match cmd {
            ScriptsCommands::Add { name, command } => {
                script_manager.add_script(name.clone(), command)?;
                println!(
                    "{}",
                    format!("✓ Script '{}' added successfully.", name).green()
                );
            }
            ScriptsCommands::Remove { name } => {
                script_manager.remove_script(&name)?;
                println!(
                    "{}",
                    format!("✓ Script '{}' removed successfully.", name).green()
                );
            }
            // In the Scripts List command handler
            ScriptsCommands::List => {
                println!("{}", "Configured Scripts:".blue().bold());
                let scripts = script_manager.get_scripts();
                if scripts.is_empty() {
                    println!("  No scripts configured");
                } else {
                    for (name, script_config) in scripts {
                        // For each command in the script
                        println!("  {}:", name.yellow());
                        for (i, cmd) in script_config.commands.iter().enumerate() {
                            let desc = cmd.description.as_deref().unwrap_or("");
                            let env_info = if !cmd.env.is_empty() {
                                format!(" [{}]", cmd.env.len())
                            } else {
                                String::new()
                            };
                            let dir_info = cmd
                                .working_dir
                                .as_ref()
                                .map(|d| format!(" (in {})", d.display()))
                                .unwrap_or_default();

                            if script_config.commands.len() > 1 {
                                println!(
                                    "    {}: {}{}{} {}",
                                    (i + 1).to_string().cyan(),
                                    cmd.command,
                                    dir_info,
                                    env_info,
                                    desc
                                );
                            } else {
                                // Single command script - simpler output
                                println!("    {}{}{} {}", cmd.command, dir_info, env_info, desc);
                            }
                        }
                        // Show parallel info if relevant
                        if script_config.parallel && script_config.commands.len() > 1 {
                            println!(
                                "    → Parallel execution ({} max threads)",
                                script_config.max_threads
                            );
                        }
                    }
                }
            }

            ScriptsCommands::Run { name } => {
                script_manager.run_script(&name)?;
                println!(
                    "{}",
                    format!("✓ Script '{}' completed successfully.", name).green()
                );
            }
        },

        Commands::Commit(cmd) => match cmd {
            CommitCommands::Validate { message_file } => {
                match hook_manager.validate_commit_message(&message_file) {
                    Ok(_) => {
                        println!("{}", "✓ Commit message is valid.".green());
                    }
                    Err(HookError::LintError { kind }) => {
                        println!("\n{}", "Commit Validation Error:".red().bold());
                        println!("{}", kind);
                        process::exit(1);
                    }
                    Err(e) => return Err(e),
                }
            }
        },
    }

    Ok(())
}
