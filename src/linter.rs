use crate::config::LinterConfig;
use crate::error::{HookError, LintErrorKind, Result};
use regex::Regex;

pub struct CommitLinter {
    config: LinterConfig,
}

impl CommitLinter {
    pub fn new(config: LinterConfig) -> Self {
        Self { config }
    }

    pub fn validate(&self, message: &str) -> Result<()> {
        if message.is_empty() {
            return Err(HookError::LintError {
                kind: LintErrorKind::InvalidFormat {
                    input: String::new(),
                    expected: "<type>(<scope>): <subject>".to_string(),
                },
            });
        }

        let re = Regex::new(r"^(?P<type>[a-z]+)(?:\((?P<scope>[a-z-]+)\))?: (?P<subject>.+)")
            .expect("Invalid regex pattern");

        let lines: Vec<&str> = message.lines().collect();
        let first_line = lines.first().ok_or_else(|| HookError::LintError {
            kind: LintErrorKind::InvalidFormat {
                input: String::new(),
                expected: "<type>(<scope>): <subject>".to_string(),
            },
        })?;

        // Validate format
        let caps = re
            .captures(first_line)
            .ok_or_else(|| HookError::LintError {
                kind: LintErrorKind::InvalidFormat {
                    input: first_line.to_string(),
                    expected: "<type>(<scope>): <subject>".to_string(),
                },
            })?;

        // Validate type
        let commit_type = caps.name("type").unwrap().as_str();
        if !self.config.types.contains(&commit_type.to_string()) {
            return Err(HookError::LintError {
                kind: LintErrorKind::InvalidType {
                    type_value: commit_type.to_string(),
                    allowed_types: self.config.types.clone(),
                },
            });
        }

        // Validate scope if present and if scopes are configured
        if let Some(scope) = caps.name("scope") {
            if !self.config.scopes.is_empty()
                && !self.config.scopes.contains(&scope.as_str().to_string())
            {
                return Err(HookError::LintError {
                    kind: LintErrorKind::InvalidScope {
                        scope: scope.as_str().to_string(),
                        allowed_scopes: self.config.scopes.clone(),
                    },
                });
            }
        }

        // Validate subject
        let subject = caps.name("subject").unwrap().as_str();

        // Check minimum length
        if subject.len() < self.config.min_subject_length {
            return Err(HookError::LintError {
                kind: LintErrorKind::SubjectTooShort {
                    subject: subject.to_string(),
                    length: subject.len(),
                    min: self.config.min_subject_length,
                },
            });
        }

        // Check maximum length
        if subject.len() > self.config.max_subject_length {
            return Err(HookError::LintError {
                kind: LintErrorKind::SubjectTooLong {
                    subject: subject.to_string(),
                    length: subject.len(),
                    max: self.config.max_subject_length,
                },
            });
        }

        // Validate body lines
        for (i, line) in lines.iter().skip(1).enumerate() {
            // Skip empty lines in body
            if line.is_empty() {
                continue;
            }

            if line.len() > self.config.max_body_line_length {
                return Err(HookError::LintError {
                    kind: LintErrorKind::BodyLineTooLong {
                        line: i + 2, // +2 because we're skipping the first line and 0-based index
                        content: line.to_string(),
                        length: line.len(),
                        max: self.config.max_body_line_length,
                    },
                });
            }
        }

        Ok(())
    }
}
