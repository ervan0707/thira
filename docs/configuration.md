# Configuration Guide

## Overview

Thira uses a YAML-based configuration file called `hooks.yaml` to define Git hooks, custom scripts, linting rules, and global options. This guide provides a comprehensive reference for all available configuration options.

## Configuration File Structure

The `hooks.yaml` file is organized into four main sections:

```yaml
hooks: # Git hook definitions
scripts: # Custom script configurations
options: # Global settings
lint: # Commit message validation rules
```

## Hooks Section

The `hooks` section defines Git hooks that will be automatically installed and executed during Git operations.

### Basic Hook Configuration

```yaml
hooks:
  pre-commit:
    - command: cargo
      args:
        - test
    - command: cargo
      args:
        - clippy
```

### Hook Properties

Each hook can contain multiple commands with the following properties:

| Property      | Type   | Required | Description                       |
| ------------- | ------ | -------- | --------------------------------- |
| `command`     | String | Yes      | The command to execute            |
| `args`        | Array  | No       | Command line arguments            |
| `working_dir` | String | No       | Working directory for the command |

### Supported Hook Types

thira supports all standard Git hooks:

- `pre-commit` - Before commit is created
- `commit-msg` - After commit message is entered
- `post-commit` - After commit is completed
- `pre-push` - Before push to remote
- `pre-receive` - Before receive on remote (server-side)
- `post-receive` - After receive on remote (server-side)
- `update` - Before each ref is updated (server-side)
- `post-update` - After refs are updated (server-side)

### Template Variables

Hooks support template variable substitution:

```yaml
hooks:
  commit-msg:
    - command: ${thira} # Resolves to thira binary path
      args:
        - commit
        - validate
        - $1 # Git passes commit message file as $1
```

Available template variables:

- `${thira}` - Path to the thira binary
- `${script_name}` - Reference to a configured script
- `$1`, `$2`, etc. - Git hook parameters

## Scripts Section

The `scripts` section defines reusable automation scripts that can be run independently or referenced in hooks. All scripts must be defined using the command array format.

### Script Configuration Format

```yaml
scripts:
  lint-all:
    parallel: false
    max_threads: 1
    commands:
      - command: "cargo clippy --workspace"
        description: "Run clippy on all workspace crates"

  test-integration:
    parallel: false
    max_threads: 1
    commands:
      - command: "cargo test --test integration"
        description: "Run integration tests"

  build-release:
    parallel: false
    max_threads: 1
    commands:
      - command: "cargo build --release"
        description: "Build release binary"

  test-all:
    parallel: true
    max_threads: 4
    commands:
      - command: "cargo test --bin thira"
        description: "Run binary tests"
        working_dir: "."
        env:
          RUST_LOG: "debug"
          TEST_ENV: "ci"
      - command: "cargo test --lib"
        description: "Run library tests"
        working_dir: "tests"
        env:
          RUST_LOG: "info"
```

### Script Properties

| Property      | Type    | Default | Description                 |
| ------------- | ------- | ------- | --------------------------- |
| `parallel`    | Boolean | `false` | Enable parallel execution   |
| `max_threads` | Number  | `4`     | Maximum concurrent threads  |
| `commands`    | Array   | -       | List of commands to execute |

### Command Properties

Each command in a script can have:

| Property      | Type   | Required | Description                            |
| ------------- | ------ | -------- | -------------------------------------- |
| `command`     | String | Yes      | The command to execute                 |
| `description` | String | No       | Human-readable description             |
| `working_dir` | String | No       | Working directory for this command     |
| `env`         | Object | No       | Environment variables for this command |

### Using Scripts in Hooks

Reference scripts in hooks using template variables:

```yaml
hooks:
  pre-commit:
    - command: ${lint-all} # References the 'lint-all' script
    - command: ${test-integration} # References the 'test-integration' script

scripts:
  lint-all:
    parallel: false
    max_threads: 1
    commands:
      - command: "cargo clippy --workspace"
        description: "Run clippy on workspace"

  test-integration:
    parallel: false
    max_threads: 1
    commands:
      - command: "cargo test --test integration"
        description: "Run integration tests"
```

## Options Section

The `options` section contains global configuration settings.

```yaml
options:
  auto_install: true
  hooks_dir: .thira
```

### Available Options

| Option         | Type    | Default  | Description                                       |
| -------------- | ------- | -------- | ------------------------------------------------- |
| `auto_install` | Boolean | `true`   | Automatically reinstall hooks when config changes |
| `hooks_dir`    | String  | `.thira` | Directory where hook files are stored             |

### Auto Install Behavior

When `auto_install` is `true`:

- Hooks are automatically reinstalled when `hooks.yaml` is saved
- Scripts added/removed trigger hook reinstallation
- Hook modifications are immediately applied

When `auto_install` is `false`:

- Manual installation required: `thira hooks install`
- Useful for controlled deployment environments

### Hooks Directory Options

| Value        | Description                            |
| ------------ | -------------------------------------- |
| `.thira`     | Default custom directory (recommended) |
| `.git/hooks` | Traditional Git hooks directory        |
| Custom path  | Any valid directory path               |

⚠️ **Note**: Using `.git` directly is not allowed for safety reasons.

## Lint Section

The `lint` section configures commit message validation rules based on conventional commit standards.

```yaml
lint:
  types:
    - feat
    - fix
    - docs
    - style
    - refactor
    - perf
    - test
    - build
    - ci
    - chore
    - revert
  scopes:
    - api
    - ui
    - db
    - core
    - cli
    - config
    - deps
    - tests
  min_subject_length: 3
  max_subject_length: 72
  max_body_line_length: 100
```

### Lint Properties

| Property               | Type   | Default            | Description                 |
| ---------------------- | ------ | ------------------ | --------------------------- |
| `types`                | Array  | See default config | Allowed commit types        |
| `scopes`               | Array  | See default config | Allowed commit scopes       |
| `min_subject_length`   | Number | `3`                | Minimum subject line length |
| `max_subject_length`   | Number | `72`               | Maximum subject line length |
| `max_body_line_length` | Number | `100`              | Maximum body line length    |

### Commit Message Format

The linter validates commits against this format:

```
<type>(<scope>): <subject>

<body>
```

Examples:

- `feat(api): add user authentication`
- `fix(ui): resolve button alignment issue`
- `docs: update installation guide`

### Default Types

- `feat` - New features
- `fix` - Bug fixes
- `docs` - Documentation changes
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `perf` - Performance improvements
- `test` - Test additions or modifications
- `build` - Build system changes
- `ci` - CI/CD changes
- `chore` - Maintenance tasks
- `revert` - Revert previous commits

### Default Scopes

- `api` - API related changes
- `ui` - User interface changes
- `db` - Database related changes
- `core` - Core functionality
- `cli` - Command line interface
- `config` - Configuration changes
- `deps` - Dependency updates
- `tests` - Test-related changes

## Complete Configuration Example

```yaml
# Complete hooks.yaml configuration example
hooks:
  pre-commit:
    - command: cargo
      args:
        - fmt
        - --check
    - command: ${lint-all}
    - command: ${test-all}

  commit-msg:
    - command: ${thira}
      args:
        - commit
        - validate
        - $1

  pre-push:
    - command: ${build-release}

scripts:
  # Simple single-command scripts
  lint-all:
    parallel: false
    max_threads: 1
    commands:
      - command: "cargo clippy --workspace -- -D warnings"
        description: "Run linting checks"
        env:
          RUSTFLAGS: "-D warnings"

  test-all:
    parallel: false
    max_threads: 1
    commands:
      - command: "cargo test --all"
        description: "Run all tests"

  build-release:
    parallel: false
    max_threads: 1
    commands:
      - command: "cargo build --release"
        description: "Build release binary"

  # Advanced parallel script
  ci-check:
    parallel: true
    max_threads: 3
    commands:
      - command: "cargo fmt --check"
        description: "Check code formatting"
      - command: "cargo clippy --workspace -- -D warnings"
        description: "Run linting checks"
        env:
          RUSTFLAGS: "-D warnings"
      - command: "cargo test --all"
        description: "Run all tests"
        env:
          RUST_BACKTRACE: "1"

options:
  auto_install: true
  hooks_dir: .thira

lint:
  types:
    - feat
    - fix
    - docs
    - style
    - refactor
    - perf
    - test
    - build
    - ci
    - chore
    - revert
  scopes:
    - api
    - ui
    - db
    - core
    - cli
    - config
    - deps
    - tests
  min_subject_length: 3
  max_subject_length: 72
  max_body_line_length: 100
```

## Configuration Validation

thira validates your configuration when:

- Loading the configuration file
- Installing hooks
- Running scripts

### Common Validation Errors

1. **Empty commands**: All commands must have non-empty command strings
2. **Invalid hook names**: Only standard Git hook names are allowed
3. **Invalid directories**: `hooks_dir` cannot be `.git` directly
4. **Invalid lint settings**: Length constraints must be positive and logical

### Configuration Testing

Test your configuration with:

```sh
# Validate configuration
thira hooks install

# Test script configuration
thira scripts list

# Test commit message rules
echo "feat(api): test message" | thira commit validate -
```

## Best Practices

### 1. Organization

- Group related hooks together
- Use descriptive script names
- Document complex configurations with comments

### 2. Performance

- Use parallel scripts for independent tasks
- Limit `max_threads` based on system resources
- Keep hooks fast to avoid blocking Git operations

### 3. Maintainability

- Use scripts to avoid duplicating commands
- Keep hooks.yaml in version control
- Test configuration changes in development first

### 4. Security

- Avoid storing sensitive data in configuration
- Use environment variables for secrets
- Validate external script sources

## Troubleshooting

### Configuration Issues

1. **YAML Syntax Errors**

   ```sh
   # Validate YAML syntax
   python -c "import yaml; yaml.safe_load(open('hooks.yaml'))"
   ```

2. **Hook Installation Failures**

   ```sh
   # Check hook installation status
   thira hooks show-path
   thira hooks list
   ```

3. **Script Execution Problems**

   ```sh
   # Test scripts individually
   thira scripts run <script-name>

   # Enable debug logging
   RUST_LOG=debug thira scripts run <script-name>
   ```

### Getting Help

- Use `--help` with any command for detailed usage
- Check the [troubleshooting section](script-management.md#troubleshooting) in other guides
- Validate configuration after making changes

## Related Documentation

- [Basic Usage Guide](basic-usage.md) - Getting started with thira
- [Hook Management](hook-management.md) - Managing Git hooks in detail
- [Script Management](script-management.md) - Advanced script configuration
- [Commit Guidelines](commit-guidelines.md) - Writing valid commit messages
