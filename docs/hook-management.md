# Hook Management Guide

## Understanding Git Hooks

Git hooks are scripts that run automatically before or after Git events like commit, push, or merge. Thira helps you manage these hooks effectively.

## Supported Hook Types

Thira supports all standard Git hooks:

- `pre-commit`: Run before commit creation
- `prepare-commit-msg`: Modify default commit message
- `commit-msg`: Validate commit messages
- `post-commit`: Run after commit creation
- `pre-push`: Run before pushing commits
- `post-checkout`: Run after switching branches
- `pre-rebase`: Run before rebasing
- `post-merge`: Run after merging
- `pre-receive`: Run before receiving pushes
- `update`: Run before updating refs
- `post-receive`: Run after receiving pushes
- `post-update`: Run after updating refs

## Managing Hooks

### Adding New Hooks

```sh
thira hooks add <hook-name> <command> [args...]
```

Example:

```sh
thira hooks add pre-commit "cargo test" -- --all-features --quiet
```

### Listing Hooks

View all configured hooks:

```sh
thira hooks list
```

### Installing Hooks

After configuring hooks, install them:

```sh
thira hooks install
```

### Uninstalling Hooks

Remove all installed hooks:

```sh
thira hooks uninstall
```

### Managing Hook Paths

Show current hooks directory:

```sh
thira hooks show-path
```

Reset to default `.git/hooks`:

```sh
thira hooks reset-path
```

## Hook Configuration

### Configuration File Structure

Hooks are configured in `hooks.yaml`:

```yaml
hooks:
  pre-commit:
    - command: cargo
      args:
        - test
        - --all-features
    - command: cargo
      args:
        - clippy
  commit-msg:
    - command: thira
      args:
        - commit
        - validate
        - $1

options:
  auto_install: true # Automatically reinstall hooks when config changes
  hooks_dir: .thira
```

### Auto-Install Option

The `auto_install` option in your configuration determines whether hooks should be automatically reinstalled when the configuration changes:

- When `true`: Hooks are automatically reinstalled whenever:
  - The configuration file is modified and saved
  - Scripts are added or removed
  - Hooks are added or modified
- When `false`: You must manually run `thira hooks install` after making changes

This is useful for:

- Ensuring hooks are always up-to-date with your configuration
- Avoiding manual reinstallation steps
- Preventing accidental use of outdated hooks

### Multiple Commands per Hook

You can configure multiple commands for each hook type:

```yaml
hooks:
  pre-commit:
    - command: cargo test
    - command: cargo clippy
    - command: cargo fmt --check
```

### Working Directory

Specify a custom working directory for hooks:

```yaml
hooks:
  pre-commit:
    - command: npm
      args:
        - test
      working_dir: frontend
```

## Best Practices

1. **Keep Hooks Fast**

   - Optimize commands for quick execution
   - Use focused tests for pre-commit hooks

2. **Handle Failures Gracefully**

   - Add clear error messages
   - Provide guidance for fixing issues

3. **Use Version Control**

   - Commit your `hooks.yaml`
   - Document hook requirements

4. **Organize Hooks**
   - Group related commands
   - Use descriptive names for custom scripts

## Troubleshooting

### Common Issues

1. **Hooks Not Executing**

   - Verify installation: `thira hooks list`
   - Check hook permissions
   - Validate hook path: `thira hooks show-path`

2. **Permission Errors**

   ```sh
   # Fix permissions
   chmod +x .thira/*
   # Reinstall hooks
   thira hooks install
   ```

3. **Configuration Issues**
   ```sh
   # Reset to default configuration
   thira hooks clean
   thira hooks init
   ```

For more help, check the error messages or use `--help` with any command.
