use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HookError {
    #[error("Git repository not found in current directory")]
    GitNotFound,

    #[error("Invalid hook name '{0}'. Valid hooks are: pre-commit, commit-msg, etc.")]
    InvalidHook(String),

    // #[error("Hook execution failed: {hook_name} - {reason}")]
    // HookExecutionError { hook_name: String, reason: String },
    #[error("Script execution failed: {script_name} - {reason}")]
    ScriptExecutionError { script_name: String, reason: String },

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Failed to read/write file at {path}: {source}")]
    FileError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Commit message validation failed: {kind}")]
    LintError { kind: LintErrorKind },
}

#[derive(Debug)]
pub enum LintErrorKind {
    InvalidFormat {
        input: String,
        expected: String,
    },
    InvalidType {
        type_value: String,
        allowed_types: Vec<String>,
    },
    InvalidScope {
        scope: String,
        allowed_scopes: Vec<String>,
    },
    SubjectTooShort {
        subject: String,
        length: usize,
        min: usize,
    },
    SubjectTooLong {
        subject: String,
        length: usize,
        max: usize,
    },
    BodyLineTooLong {
        line: usize,
        content: String,
        length: usize,
        max: usize,
    },
}

impl std::fmt::Display for LintErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat { input, expected } => {
                writeln!(f, "Invalid commit message format")?;
                writeln!(f, "Input:    {}", input)?;
                writeln!(f, "Expected: {}", expected)?;
                writeln!(f, "Example:  feat(api): add user authentication")
            }
            Self::InvalidType {
                type_value,
                allowed_types,
            } => {
                writeln!(f, "Invalid commit type '{}'", type_value)?;
                writeln!(f, "Allowed types:")?;
                for t in allowed_types {
                    writeln!(f, "  - {}", t)?;
                }
                Ok(())
            }
            Self::InvalidScope {
                scope,
                allowed_scopes,
            } => {
                writeln!(f, "Invalid commit scope '{}'", scope)?;
                writeln!(f, "Allowed scopes:")?;
                for s in allowed_scopes {
                    writeln!(f, "  - {}", s)?;
                }
                Ok(())
            }
            Self::SubjectTooShort {
                subject,
                length,
                min,
            } => {
                writeln!(
                    f,
                    "Commit subject too short ({} chars, minimum {})",
                    length, min
                )?;
                writeln!(f, "Subject: {}", subject)?;
                writeln!(f, "Please provide a more descriptive commit message")
            }
            Self::SubjectTooLong {
                subject,
                length,
                max,
            } => {
                writeln!(
                    f,
                    "Commit subject too long ({} chars, maximum {})",
                    length, max
                )?;
                writeln!(f, "Subject: {}", subject)?;
                writeln!(f, "Please make your commit message more concise")
            }
            Self::BodyLineTooLong {
                line,
                content,
                length,
                max,
            } => {
                writeln!(
                    f,
                    "Line {} is too long ({} chars, maximum {})",
                    line, length, max
                )?;
                writeln!(f, "Content: {}", content)?;
                writeln!(f, "Please break this line into multiple lines")
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, HookError>;
