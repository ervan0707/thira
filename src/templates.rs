use crate::config::{CommandConfig, Config, Hook, LinterConfig, Options, ScriptConfig};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ProjectTemplate {
    Rust,
    NodeJs,
    Python,
    Go,
    Java,
    Generic,
}

impl ProjectTemplate {
    pub fn all() -> Vec<(ProjectTemplate, &'static str, &'static str)> {
        vec![
            (
                ProjectTemplate::Rust,
                "Rust Project",
                "Cargo-based Rust project with clippy, tests, and formatting",
            ),
            (
                ProjectTemplate::NodeJs,
                "Node.js Project",
                "NPM/Yarn project with linting, testing, and formatting",
            ),
            (
                ProjectTemplate::Python,
                "Python Project",
                "Python project with black formatting, flake8 linting, and pytest",
            ),
            (
                ProjectTemplate::Go,
                "Go Project",
                "Go project with formatting, linting, and testing",
            ),
            (
                ProjectTemplate::Java,
                "Java Project",
                "Maven/Gradle Java project with formatting and testing",
            ),
            (
                ProjectTemplate::Generic,
                "Generic Project",
                "Basic template with minimal configuration",
            ),
        ]
    }

    pub fn to_config(&self) -> Config {
        match self {
            ProjectTemplate::Rust => self.rust_config(),
            ProjectTemplate::NodeJs => self.nodejs_config(),
            ProjectTemplate::Python => self.python_config(),
            ProjectTemplate::Go => self.go_config(),
            ProjectTemplate::Java => self.java_config(),
            ProjectTemplate::Generic => self.generic_config(),
        }
    }

    fn rust_config(&self) -> Config {
        let mut hooks = HashMap::new();
        let mut scripts = HashMap::new();

        // Pre-commit hooks for Rust
        hooks.insert(
            "pre-commit".to_string(),
            vec![
                Hook {
                    command: "cargo".to_string(),
                    args: vec!["fmt".to_string(), "--check".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "cargo".to_string(),
                    args: vec![
                        "clippy".to_string(),
                        "--".to_string(),
                        "-D".to_string(),
                        "warnings".to_string(),
                    ],
                    working_dir: None,
                },
                Hook {
                    command: "cargo".to_string(),
                    args: vec!["test".to_string()],
                    working_dir: None,
                },
            ],
        );

        // Commit message validation
        hooks.insert(
            "commit-msg".to_string(),
            vec![Hook {
                command: "${thira}".to_string(),
                args: vec![
                    "commit".to_string(),
                    "validate".to_string(),
                    "$1".to_string(),
                ],
                working_dir: None,
            }],
        );

        // Scripts for Rust development
        scripts.insert(
            "format".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "cargo fmt".to_string(),
                    description: Some("Format Rust code".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "lint".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "cargo clippy -- -D warnings".to_string(),
                    description: Some("Run Clippy linter".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "test".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "cargo test".to_string(),
                    description: Some("Run tests".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        Config {
            hooks,
            scripts,
            options: Options {
                auto_install: true,
                hooks_dir: ".thira".to_string(),
            },
            lint: self.default_linter_config(),
        }
    }

    fn nodejs_config(&self) -> Config {
        let mut hooks = HashMap::new();
        let mut scripts = HashMap::new();

        // Pre-commit hooks for Node.js
        hooks.insert(
            "pre-commit".to_string(),
            vec![
                Hook {
                    command: "npm".to_string(),
                    args: vec!["run".to_string(), "lint".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "npm".to_string(),
                    args: vec!["run".to_string(), "format:check".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "npm".to_string(),
                    args: vec!["test".to_string()],
                    working_dir: None,
                },
            ],
        );

        // Commit message validation
        hooks.insert(
            "commit-msg".to_string(),
            vec![Hook {
                command: "${thira}".to_string(),
                args: vec![
                    "commit".to_string(),
                    "validate".to_string(),
                    "$1".to_string(),
                ],
                working_dir: None,
            }],
        );

        // Scripts for Node.js development
        scripts.insert(
            "install".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "npm install".to_string(),
                    description: Some("Install dependencies".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "format".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "npm run format".to_string(),
                    description: Some("Format code with Prettier".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "lint".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "npm run lint".to_string(),
                    description: Some("Run ESLint".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        Config {
            hooks,
            scripts,
            options: Options {
                auto_install: true,
                hooks_dir: ".thira".to_string(),
            },
            lint: self.default_linter_config(),
        }
    }

    fn python_config(&self) -> Config {
        let mut hooks = HashMap::new();
        let mut scripts = HashMap::new();

        // Pre-commit hooks for Python
        hooks.insert(
            "pre-commit".to_string(),
            vec![
                Hook {
                    command: "black".to_string(),
                    args: vec!["--check".to_string(), ".".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "flake8".to_string(),
                    args: vec![".".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "pytest".to_string(),
                    args: vec![],
                    working_dir: None,
                },
            ],
        );

        // Commit message validation
        hooks.insert(
            "commit-msg".to_string(),
            vec![Hook {
                command: "${thira}".to_string(),
                args: vec![
                    "commit".to_string(),
                    "validate".to_string(),
                    "$1".to_string(),
                ],
                working_dir: None,
            }],
        );

        // Scripts for Python development
        scripts.insert(
            "format".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "black .".to_string(),
                    description: Some("Format Python code with Black".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "lint".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "flake8 .".to_string(),
                    description: Some("Run flake8 linter".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "test".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "pytest".to_string(),
                    description: Some("Run tests with pytest".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        Config {
            hooks,
            scripts,
            options: Options {
                auto_install: true,
                hooks_dir: ".thira".to_string(),
            },
            lint: self.default_linter_config(),
        }
    }

    fn go_config(&self) -> Config {
        let mut hooks = HashMap::new();
        let mut scripts = HashMap::new();

        // Pre-commit hooks for Go
        hooks.insert(
            "pre-commit".to_string(),
            vec![
                Hook {
                    command: "gofmt".to_string(),
                    args: vec!["-l".to_string(), ".".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "go".to_string(),
                    args: vec!["vet".to_string(), "./...".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "go".to_string(),
                    args: vec!["test".to_string(), "./...".to_string()],
                    working_dir: None,
                },
            ],
        );

        // Commit message validation
        hooks.insert(
            "commit-msg".to_string(),
            vec![Hook {
                command: "${thira}".to_string(),
                args: vec![
                    "commit".to_string(),
                    "validate".to_string(),
                    "$1".to_string(),
                ],
                working_dir: None,
            }],
        );

        // Scripts for Go development
        scripts.insert(
            "format".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "gofmt -w .".to_string(),
                    description: Some("Format Go code".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "lint".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "go vet ./...".to_string(),
                    description: Some("Run Go vet".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "test".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "go test ./...".to_string(),
                    description: Some("Run tests".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        Config {
            hooks,
            scripts,
            options: Options {
                auto_install: true,
                hooks_dir: ".thira".to_string(),
            },
            lint: self.default_linter_config(),
        }
    }

    fn java_config(&self) -> Config {
        let mut hooks = HashMap::new();
        let mut scripts = HashMap::new();

        // Pre-commit hooks for Java (Maven)
        hooks.insert(
            "pre-commit".to_string(),
            vec![
                Hook {
                    command: "mvn".to_string(),
                    args: vec!["spotless:check".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "mvn".to_string(),
                    args: vec!["compile".to_string()],
                    working_dir: None,
                },
                Hook {
                    command: "mvn".to_string(),
                    args: vec!["test".to_string()],
                    working_dir: None,
                },
            ],
        );

        // Commit message validation
        hooks.insert(
            "commit-msg".to_string(),
            vec![Hook {
                command: "${thira}".to_string(),
                args: vec![
                    "commit".to_string(),
                    "validate".to_string(),
                    "$1".to_string(),
                ],
                working_dir: None,
            }],
        );

        // Scripts for Java development
        scripts.insert(
            "format".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "mvn spotless:apply".to_string(),
                    description: Some("Format Java code".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "compile".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "mvn compile".to_string(),
                    description: Some("Compile Java code".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        scripts.insert(
            "test".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "mvn test".to_string(),
                    description: Some("Run tests".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        Config {
            hooks,
            scripts,
            options: Options {
                auto_install: true,
                hooks_dir: ".thira".to_string(),
            },
            lint: self.default_linter_config(),
        }
    }

    fn generic_config(&self) -> Config {
        let mut hooks = HashMap::new();
        let mut scripts = HashMap::new();

        // Basic pre-commit hook
        hooks.insert(
            "pre-commit".to_string(),
            vec![Hook {
                command: "echo".to_string(),
                args: vec!["Running pre-commit checks...".to_string()],
                working_dir: None,
            }],
        );

        // Commit message validation
        hooks.insert(
            "commit-msg".to_string(),
            vec![Hook {
                command: "${thira}".to_string(),
                args: vec![
                    "commit".to_string(),
                    "validate".to_string(),
                    "$1".to_string(),
                ],
                working_dir: None,
            }],
        );

        // Basic script
        scripts.insert(
            "hello".to_string(),
            ScriptConfig {
                parallel: false,
                max_threads: 1,
                commands: vec![CommandConfig {
                    command: "echo 'Hello from Thira!'".to_string(),
                    description: Some("A simple hello script".to_string()),
                    working_dir: Some(PathBuf::from(".")),
                    env: HashMap::new(),
                }],
            },
        );

        Config {
            hooks,
            scripts,
            options: Options {
                auto_install: true,
                hooks_dir: ".thira".to_string(),
            },
            lint: self.default_linter_config(),
        }
    }

    fn default_linter_config(&self) -> LinterConfig {
        LinterConfig {
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
            min_subject_length: 3,
            max_subject_length: 72,
            max_body_line_length: 100,
        }
    }
}
