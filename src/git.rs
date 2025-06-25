use crate::error::{HookError, Result};
use std::path::PathBuf;

pub struct GitRepo {
    pub hooks_dir: PathBuf,
}

impl GitRepo {
    pub fn set_hooks_path(&self) -> Result<()> {
        let status = std::process::Command::new("git")
            .args([
                "config",
                "core.hooksPath",
                &self.hooks_dir.display().to_string(),
            ])
            .status()?;

        if !status.success() {
            return Err(HookError::ConfigError(
                "Failed to set core.hooksPath".into(),
            ));
        }

        Ok(())
    }

    pub fn new(hooks_dir: &str) -> Result<Self> {
        let root = std::env::current_dir().map_err(|e| HookError::FileError {
            path: PathBuf::from("."),
            source: e,
        })?;

        // Verify git repository exists
        let output = std::process::Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .output()
            .map_err(HookError::IoError)?;

        if !output.status.success() {
            return Err(HookError::GitNotFound);
        }

        // Construct hooks directory path
        let hooks_dir = if hooks_dir == ".git/hooks" {
            root.join(".git").join("hooks")
        } else {
            root.join(hooks_dir)
        };

        std::fs::create_dir_all(&hooks_dir).map_err(|e| HookError::FileError {
            path: hooks_dir.clone(),
            source: e,
        })?;

        Ok(Self { hooks_dir })
    }

    pub fn install_hook(&self, name: &str, content: &str) -> Result<()> {
        let hook_path = self.hooks_dir.join(name);

        // Use map_err because we're adding path context
        std::fs::write(&hook_path, content).map_err(|e| HookError::FileError {
            path: hook_path.clone(),
            source: e,
        })?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&hook_path)
                .map_err(|e| HookError::FileError {
                    path: hook_path.clone(),
                    source: e,
                })?
                .permissions();

            perms.set_mode(0o755);
            std::fs::set_permissions(&hook_path, perms).map_err(|e| HookError::FileError {
                path: hook_path,
                source: e,
            })?;
        }

        Ok(())
    }

    pub fn unset_hooks_path(&self) -> Result<()> {
        // Add safety check before cleanup
        if self.hooks_dir.ends_with(".git") {
            return Err(HookError::ConfigError(
                "Cannot remove .git directory. Use '.git/hooks' for Git hooks.".to_string(),
            ));
        }

        // First unset the Git config
        let status = std::process::Command::new("git")
            .args(["config", "--unset", "core.hooksPath"])
            .status()?;

        if !status.success() && !status.code().unwrap_or(0) == 5 {
            // Error code 5 means key was not found
            return Err(HookError::ConfigError(
                "Failed to unset core.hooksPath".into(),
            ));
        }

        // Only clean up custom hooks directory if it's not .git/hooks and not .git
        if !self.hooks_dir.ends_with(".git/hooks") && !self.hooks_dir.ends_with(".git") && self.hooks_dir.exists() {
             std::fs::remove_dir_all(&self.hooks_dir)?;
        }

        Ok(())
    }

    pub fn uninstall_hook(&self, name: &str) -> Result<()> {
        let hook_path = self.hooks_dir.join(name);
        if hook_path.exists() {
            std::fs::remove_file(hook_path)?;
        }
        Ok(())
    }

    pub fn validate_hook_name(&self, name: &str) -> Result<()> {
        let valid_hooks = [
            "pre-commit",
            "prepare-commit-msg",
            "commit-msg",
            "post-commit",
            "pre-push",
            "post-checkout",
            "pre-rebase",
            "post-merge",
            "pre-receive",
            "update",
            "post-receive",
            "post-update",
        ];

        if !valid_hooks.contains(&name) {
            return Err(HookError::InvalidHook(name.to_string()));
        }

        Ok(())
    }

    pub fn get_git_hooks_dir() -> Result<PathBuf> {
        let root = std::env::current_dir().map_err(|e| HookError::FileError {
            path: PathBuf::from("."),
            source: e,
        })?;
        Ok(root.join(".git").join("hooks"))
    }

    // Add a method to clean old hooks
    pub fn clean_git_hooks(&self, hook_names: &[String]) -> Result<()> {
        let git_hooks_dir = Self::get_git_hooks_dir()?;

        // Only clean if the current hooks_dir is different from git hooks dir
        if self.hooks_dir != git_hooks_dir {
            for name in hook_names {
                let hook_path = git_hooks_dir.join(name);
                if hook_path.exists() {
                    std::fs::remove_file(&hook_path).map_err(|e| HookError::FileError {
                        path: hook_path,
                        source: e,
                    })?;
                }
            }
        }
        Ok(())
    }
}
