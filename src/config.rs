use crate::error::HookError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScriptConfig {
    #[serde(default)]
    pub parallel: bool,
    #[serde(default = "default_max_threads")]
    pub max_threads: usize,
    #[serde(default)]
    pub commands: Vec<CommandConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandConfig {
    pub command: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub working_dir: Option<PathBuf>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LinterConfig {
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default = "default_min_subject_length")]
    pub min_subject_length: usize,
    #[serde(default = "default_max_subject_length")]
    pub max_subject_length: usize,
    #[serde(default = "default_max_body_line_length")]
    pub max_body_line_length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hooks: HashMap<String, Vec<Hook>>,
    pub scripts: HashMap<String, ScriptConfig>,
    #[serde(default)]
    pub options: Options,
    #[serde(default)]
    pub lint: LinterConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hook {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub working_dir: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Options {
    #[serde(default)]
    pub auto_install: bool,
    #[serde(default = "default_hooks_dir")]
    pub hooks_dir: String,
}

// Default values
fn default_min_subject_length() -> usize {
    3
}

fn default_max_subject_length() -> usize {
    72
}

fn default_max_body_line_length() -> usize {
    100
}

fn default_hooks_dir() -> String {
    ".thira".to_string()
}

fn default_max_threads() -> usize {
    4
}

use std::fmt;

// Add Display implementation for ScriptConfig
impl fmt::Display for ScriptConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.commands.len() == 1 {
            // For backward compatibility with simple scripts, just show the command
            write!(f, "{}", self.commands[0].command)
        } else {
            write!(
                f,
                "{} commands, parallel: {}",
                self.commands.len(),
                if self.parallel { "yes" } else { "no" }
            )
        }
    }
}



impl Config {
    pub fn load() -> crate::error::Result<Self> {
        let config_path = PathBuf::from("hooks.yaml");
        if !config_path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(config_path)?;
        Ok(serde_yaml::from_str(&content)?)
    }

    pub fn save(&self) -> crate::error::Result<()> {
        let mut content = String::new();

        // Hooks section
        content.push_str("hooks:\n");
        for (name, hooks) in &self.hooks {
            content.push_str(&format!("  {}:\n", name));
            for hook in hooks {
                content.push_str("    - command: ");
                content.push_str(&hook.command);
                content.push('\n');

                if !hook.args.is_empty() {
                    content.push_str("      args:\n");
                    for arg in &hook.args {
                        content.push_str(&format!("        - {}\n", arg));
                    }
                }

                if let Some(working_dir) = &hook.working_dir {
                    content.push_str(&format!("      working_dir: {}\n", working_dir.display()));
                }
            }
        }

        // Scripts section 
        content.push_str("\nscripts:\n");
        for (name, script) in &self.scripts {
            content.push_str(&format!("  {}:\n", name));

            // Write parallel and max_threads if parallel is true or there are multiple commands
            if script.parallel || script.commands.len() > 1 {
                content.push_str(&format!("    parallel: {}\n", script.parallel));
                content.push_str(&format!("    max_threads: {}\n", script.max_threads));
            }

            // Always write commands section
            content.push_str("    commands:\n");
            for cmd in &script.commands {
                content.push_str(&format!("      - command: \"{}\"\n", cmd.command));

                if let Some(desc) = &cmd.description {
                    content.push_str(&format!("        description: \"{}\"\n", desc));
                }

                if let Some(dir) = &cmd.working_dir {
                    content.push_str(&format!("        working_dir: \"{}\"\n", dir.display()));
                }

                if !cmd.env.is_empty() {
                    content.push_str("        env:\n");
                    for (key, value) in &cmd.env {
                        content.push_str(&format!("          {}: \"{}\"\n", key, value));
                    }
                }
            }
        }

        // Options section
        content.push_str("\noptions:\n");
        content.push_str(&format!("  auto_install: {}\n", self.options.auto_install));
        content.push_str(&format!("  hooks_dir: {}\n", self.options.hooks_dir));

        // Linter section
        content.push_str("\nlint:\n");
        if !self.lint.types.is_empty() {
            content.push_str("  types:\n");
            for t in &self.lint.types {
                content.push_str(&format!("    - {}\n", t));
            }
        }
        if !self.lint.scopes.is_empty() {
            content.push_str("  scopes:\n");
            for s in &self.lint.scopes {
                content.push_str(&format!("    - {}\n", s));
            }
        }
        content.push_str(&format!(
            "  min_subject_length: {}\n",
            self.lint.min_subject_length
        ));
        content.push_str(&format!(
            "  max_subject_length: {}\n",
            self.lint.max_subject_length
        ));
        content.push_str(&format!(
            "  max_body_line_length: {}\n",
            self.lint.max_body_line_length
        ));

        // Write the content to file
        std::fs::write("hooks.yaml", content)?;

        // Auto-install hooks if enabled
        if self.options.auto_install {
            println!("Installing hooks auto_install...");
            let hook_manager = crate::hooks::HookManager::new()?;
            hook_manager.install_hooks()?;
        }
        Ok(())
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        self.validate_hooks()?;
        self.validate_lint_config()?;
        self.validate_hooks_dir()?;
        Ok(())
    }

    fn validate_hooks_dir(&self) -> crate::error::Result<()> {
        if self.options.hooks_dir == ".git" {
            return Err(HookError::ConfigError(
                "Invalid hooks directory: Cannot use '.git' directly. Use '.git/hooks' instead."
                    .to_string(),
            ));
        }
        Ok(())
    }

    fn validate_hooks(&self) -> crate::error::Result<()> {
        for (name, hooks) in &self.hooks {
            if hooks.is_empty() {
                return Err(HookError::ConfigError(format!(
                    "Hook '{}' has no commands",
                    name
                )));
            }

            for hook in hooks {
                if hook.command.is_empty() {
                    return Err(HookError::ConfigError(format!(
                        "Empty command in hook '{}'",
                        name
                    )));
                }
            }
        }
        Ok(())
    }

    fn validate_lint_config(&self) -> crate::error::Result<()> {
        if self.lint.min_subject_length == 0 {
            return Err(HookError::ConfigError(
                "min_subject_length must be greater than 0".into(),
            ));
        }
        if self.lint.max_subject_length < self.lint.min_subject_length {
            return Err(HookError::ConfigError(
                "max_subject_length must be greater than min_subject_length".into(),
            ));
        }
        if self.lint.max_body_line_length == 0 {
            return Err(HookError::ConfigError(
                "max_body_line_length must be greater than 0".into(),
            ));
        }
        Ok(())
    }

    pub fn add_script(&mut self, name: String, command: String) -> crate::error::Result<()> {
        self.scripts.insert(name, ScriptConfig {
            parallel: false,
            max_threads: default_max_threads(),
            commands: vec![
                CommandConfig {
                    command,
                    description: None,
                    working_dir: None,
                    env: HashMap::new(),
                }
            ],
        });
        self.save()?;
        Ok(())
    }

    pub fn remove_script(&mut self, name: &str) -> crate::error::Result<()> {
        self.scripts.remove(name);
        self.save()?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut hooks = HashMap::new();
        let mut scripts = HashMap::new();

        // Add default scripts
        // 1. test-all script
        scripts.insert(
            "test-all".to_string(),
            ScriptConfig {
                parallel: true,
                max_threads: 4,
                commands: vec![
                    CommandConfig {
                        command: "sh test1.sh".to_string(),
                        description: Some("Run test script 1".to_string()),
                        working_dir: Some(PathBuf::from(".")),
                        env: {
                            let mut env = HashMap::new();
                            env.insert("TEST_MODE".to_string(), "parallel-1".to_string());
                            env.insert("TEST_VALUE".to_string(), "123".to_string());
                            env
                        },
                    },
                    CommandConfig {
                        command: "sh test2.sh".to_string(),
                        description: Some("Run test script 2".to_string()),
                        working_dir: Some(PathBuf::from(".")),
                        env: {
                            let mut env = HashMap::new();
                            env.insert("TEST_MODE".to_string(), "parallel-2".to_string());
                            env.insert("TEST_VALUE".to_string(), "456".to_string());
                            env
                        },
                    },
                ],
            },
        );

        // Add default hooks
        // 1. pre-commit hook
        hooks.insert(
            "pre-commit".to_string(),
            vec![
                Hook {
                    command: "cargo".to_string(),
                    args: vec!["test".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "cargo".to_string(),
                    args: vec!["clippy".to_string()],
                    working_dir: None,
                },
            ],
        );

        // 2. commit-msg hook
        hooks.insert(
            "commit-msg".to_string(),
            vec![Hook {
                command: "${thira}".to_string(), // Use the template variable
                args: vec![
                    "commit".to_string(),
                    "validate".to_string(),
                    "$1".to_string(),
                ],
                working_dir: None,
            }],
        );

        // Default linter config
        let lint = LinterConfig {
            types: vec![
                "feat".into(),
                "fix".into(),
                "docs".into(),
                "style".into(),
                "refactor".into(),
                "perf".into(),
                "test".into(),
                "build".into(),
                "ci".into(),
                "chore".into(),
                "revert".into(),
            ],
            scopes: vec![
                "api".into(),
                "ui".into(),
                "db".into(),
                "core".into(),
                "cli".into(),
                "config".into(),
                "deps".into(),
                "tests".into(),
            ],
            min_subject_length: default_min_subject_length(),
            max_subject_length: default_max_subject_length(),
            max_body_line_length: default_max_body_line_length(),
        };

        Self {
            hooks,
            scripts,
            options: Options {
                auto_install: true,
                hooks_dir: ".thira".to_string(),
            },
            lint,
        }
    }
}
