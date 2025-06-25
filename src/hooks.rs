use crate::config::{CommandConfig, Config};
use crate::error::HookError;
use crate::error::Result;
use crate::git::GitRepo;
use crate::linter::CommitLinter;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct HookManager {
    config: Config,
    repo: GitRepo,
}

impl HookManager {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let repo = GitRepo::new(&config.options.hooks_dir)?;

        Ok(Self { config, repo })
    }

    pub fn validate_commit_message(&self, message_file: &str) -> Result<()> {
        let message = std::fs::read_to_string(message_file).map_err(|e| HookError::FileError {
            path: PathBuf::from(message_file),
            source: e,
        })?;

        let linter = CommitLinter::new(self.config.lint.clone());
        linter.validate(&message)
    }

    pub fn get_hooks(&self) -> &HashMap<String, Vec<crate::config::Hook>> {
        &self.config.hooks
    }

    pub fn install_hooks(&self) -> Result<()> {
        self.config.validate()?;

        // Get list of existing hook files
        let existing_hooks = std::fs::read_dir(&self.repo.hooks_dir)
            .map_err(|e| HookError::FileError {
                path: self.repo.hooks_dir.clone(),
                source: e,
            })?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name())
            .filter_map(|name| name.into_string().ok())
            .collect::<Vec<_>>();

        // Remove hooks that are no longer in config
        for hook_name in existing_hooks {
            if !self.config.hooks.contains_key(&hook_name) {
                self.repo.uninstall_hook(&hook_name)?;
            }
        }

        // Clean up old hooks in .git/hooks before installing new ones
        self.repo
            .clean_git_hooks(&self.config.hooks.keys().cloned().collect::<Vec<_>>())?;

        // Install new hooks
        for (name, hooks) in &self.config.hooks {
            self.repo.validate_hook_name(name)?;
            let script = self.generate_hook_script(hooks);
            self.repo.install_hook(name, &script)?;
        }

        // Configure Git to use our hooks directory
        if !self.config.options.hooks_dir.contains(".git/hooks") {
            self.repo.set_hooks_path()?;
        }

        Ok(())
    }

    fn generate_hook_script(&self, hooks: &[crate::config::Hook]) -> String {
        let mut script = String::from("#!/bin/sh\n\n");

        for hook in hooks {
            let mut command = self.substitute_variables(&hook.command);

            // Special handling for the commit-msg hook
            if command.contains("thira") && hook.args.contains(&"commit".to_string()) {
                // Use the binary path for commit message validation
                script.push_str(&format!(
                    "{} commit validate \"$1\"\n",
                    std::env::current_exe().unwrap().display()
                ));
                script.push_str("if [ $? -ne 0 ]; then\n");
                script.push_str("  exit 1\n");
                script.push_str("fi\n\n");
                continue;
            }

            if !hook.args.is_empty() {
                let args = hook
                    .args
                    .iter()
                    .map(|arg| self.substitute_variables(arg))
                    .collect::<Vec<_>>()
                    .join(" ");
                command.push_str(&format!(" {}", args));
            }

            script.push_str(&format!("{}\n", command));
            script.push_str("if [ $? -ne 0 ]; then\n");
            script.push_str("  exit 1\n");
            script.push_str("fi\n\n");
        }

        script
    }

    fn substitute_variables(&self, input: &str) -> String {
        use regex::Regex;
        let mut result = input.to_string();

        // Get the current binary path
        let binary_path = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("thira"))
            .display()
            .to_string();

        // Create regex for variable substitution
        let re = Regex::new(r"\$\{([^}]+)\}").unwrap();

        // Collect all replacements first to avoid recursive substitution
        let mut replacements = Vec::new();

        for cap in re.captures_iter(&result) {
            let var = &cap[0]; // The full match including ${...}
            let var_name = &cap[1]; // Just the name inside

            let replacement = match var_name {
                // Special case for the binary path
                "thira" => binary_path.clone(),

                // Handle script references
                s if s.starts_with("scripts.") => {
                    let script_name = &s["scripts.".len()..];
                    format!("{} scripts run {}", binary_path, script_name)
                }
                s if self.config.scripts.contains_key(s) => {
                    format!("{} scripts run {}", binary_path, s)
                }

                // Handle command references (name.N)
                s if s.contains('.') => {
                    let parts: Vec<&str> = s.split('.').collect();
                    match parts[..] {
                        [script_name, index_str] => {
                            if let Some(script) = self.config.scripts.get(script_name) {
                                if let Ok(idx) = index_str.parse::<usize>() {
                                    if let Some(cmd) = script.commands.get(idx - 1) {
                                        self.build_command_string(cmd)
                                    } else {
                                        var.to_string()
                                    }
                                } else {
                                    var.to_string()
                                }
                            } else {
                                var.to_string()
                            }
                        }
                        [prefix @ "scripts", script_name, index_str] => {
                            if let Some(script) = self.config.scripts.get(script_name) {
                                if let Ok(idx) = index_str.parse::<usize>() {
                                    if script.commands.get(idx - 1).is_some() {
                                        format!("{} {} run {}", binary_path, prefix, script_name)
                                    } else {
                                        var.to_string()
                                    }
                                } else {
                                    var.to_string()
                                }
                            } else {
                                var.to_string()
                            }
                        }
                        _ => var.to_string(),
                    }
                }

                // Keep unknown variables as-is
                _ => var.to_string(),
            };

            replacements.push((var.to_string(), replacement));
        }

        // Apply all replacements
        for (var, replacement) in replacements {
            result = result.replace(&var, &replacement);
        }

        result
    }

    // Helper method to build a command string
    fn build_command_string(&self, cmd: &CommandConfig) -> String {
        let mut parts = Vec::new();

        // Add working directory if specified
        if let Some(dir) = &cmd.working_dir {
            parts.push(format!("cd {} &&", dir.display()));
        }

        // Add environment variables if any
        for (key, value) in &cmd.env {
            parts.push(format!("{}=\"{}\"", key, value));
        }

        // Add the main command
        parts.push(cmd.command.clone());

        parts.join(" ")
    }

    pub fn get_hooks_path(&self) -> Result<String> {
        // First check Git config
        let output = std::process::Command::new("git")
            .args(["config", "--get", "core.hooksPath"])
            .output()?;

        // If Git config exists and is valid, use it
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path_buf = std::path::PathBuf::from(&path);

            if path_buf.exists() {
                return Ok(path);
            }
        }

        // Otherwise return the path from our config
        Ok(self.config.options.hooks_dir.clone())
    }

    pub fn uninstall_hooks(&mut self) -> Result<()> {
        // First uninstall all hooks from current location
        for name in self.config.hooks.keys() {
            self.repo.uninstall_hook(name)?;
        }

        // Clean up any hooks in .git/hooks
        self.repo
            .clean_git_hooks(&self.config.hooks.keys().cloned().collect::<Vec<_>>())?;

        // Unset Git's core.hooksPath
        self.repo.unset_hooks_path()?;

        // Remove the config file
        if std::path::Path::new("hooks.yaml").exists() {
            std::fs::remove_file("hooks.yaml")?;
        }

        Ok(())
    }

    pub fn unset_hooks_path(&mut self) -> Result<()> {
        // Clean up hooks from old location
        self.repo
            .clean_git_hooks(&self.config.hooks.keys().cloned().collect::<Vec<_>>())?;

        // Unset Git's core.hooksPath
        self.repo.unset_hooks_path()?;

        // Update config to use default .git/hooks directory
        self.config.options.hooks_dir = ".git/hooks".to_string();

        // Save config without auto-installing
        let auto_install = self.config.options.auto_install;
        self.config.options.auto_install = false;
        self.config.save()?;
        self.config.options.auto_install = auto_install;

        // Update repo reference
        self.repo = GitRepo::new(&self.config.options.hooks_dir)?;

        Ok(())
    }

    pub fn add_hook(&mut self, name: String, command: String, args: Vec<String>) -> Result<()> {
        let hook = crate::config::Hook {
            command,
            args,
            working_dir: None,
        };

        self.repo.validate_hook_name(&name)?;

        self.config
            .hooks
            .entry(name.clone())
            .or_default()
            .push(hook);

        self.config.save()?;

        Ok(())
    }
}
