# Script Management Guide

## Overview

Thira allows you to define and manage reusable scripts that can be integrated into your Git hooks or run independently. This guide explains how to effectively use script management features, including experimental parallel execution capabilities.

## Managing Scripts

### Adding Scripts

Add a new script using:

```sh
thira scripts add <name> <command>
```

Example:

```sh
thira scripts add lint "cargo clippy --all-features"
thira scripts add test "cargo test --all"
```

### Listing Scripts

View all configured scripts:

```sh
thira scripts list
```

### Running Scripts

Execute a configured script:

```sh
thira scripts run <name>
```

Example:

```sh
thira scripts run lint
```

### Removing Scripts

Delete a configured script:

```sh
thira scripts remove <name>
```

## Script Configuration

Scripts are stored in the `hooks.yaml` file under the `scripts` section. All scripts must be defined using the command array format:

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
      - command: "cargo test --lib"
        description: "Run library tests"
        working_dir: "."
        env:
          RUST_LOG: "info"
      - command: "cargo clippy --all-targets"
        description: "Run clippy on all targets"
```

### Required Script Properties

Each script must include:

- `parallel`: Enable parallel execution (true/false)
- `max_threads`: Maximum number of concurrent threads
- `commands`: Array of command configurations

### Command Configuration

Each command in the commands array can have:

- `command`: The command to execute (required)
- `description`: Human-readable description (optional)
- `working_dir`: Working directory for the command (optional)
- `env`: Environment variables specific to this command (optional)

## Parallel Execution (Experimental)

⚠️ **Note**: Parallel script execution is an experimental feature and may have limitations or unexpected behavior.

### Configuration Options

- `parallel`: Enable parallel execution of commands (default: `false`)
- `max_threads`: Maximum number of concurrent threads (default: `4`)
- `commands`: Array of command configurations

### Command Configuration

Each command in a parallel script can have:

- `command`: The command to execute (required)
- `description`: Human-readable description (optional)
- `working_dir`: Working directory for the command (optional)
- `env`: Environment variables specific to this command (optional)

### Real-time Output

When running parallel scripts, thira provides:

- Real-time terminal output display
- Per-command execution status
- Execution timing information
- Color-coded status indicators

### Sequential vs Parallel

- **Sequential** (`parallel: false`): Commands run one after another, stopping on first failure
- **Parallel** (`parallel: true`): Commands run concurrently with thread limiting

Example sequential script:

```yaml
scripts:
  ci-check:
    parallel: false
    commands:
      - command: "cargo fmt --check"
        description: "Check formatting"
      - command: "cargo clippy -- -D warnings"
        description: "Check linting"
      - command: "cargo test"
        description: "Run tests"
```

## Using Scripts in Hooks

You can reference scripts in your hook configurations using `${script_name}`:

```yaml
hooks:
  pre-commit:
    - command: ${lint}
    - command: ${test}
```

This will execute the defined scripts during the pre-commit hook.

## Best Practices

### 1. Naming Conventions

- Use descriptive names
- Keep names short but meaningful
- Use kebab-case for multi-word names

Good examples:

```yaml
scripts:
  lint-all: "cargo clippy --workspace"
  test-integration: "cargo test --test integration"
  build-release: "cargo build --release"
```

### 2. Script Organization

- Group related scripts
- Use consistent naming patterns
- Document complex scripts with descriptions

### 3. Error Handling

- Include error checks
- Provide clear error messages
- Return appropriate exit codes

### 4. Performance

- Keep scripts efficient
- Avoid unnecessary operations
- Use parallel execution for independent tasks
- Be cautious with resource-intensive parallel operations

### 5. Parallel Script Guidelines

⚠️ **Experimental Feature Considerations:**

- Test parallel scripts thoroughly before production use
- Monitor resource usage with multiple concurrent commands
- Be aware that parallel execution may produce interleaved output
- Consider dependencies between commands when enabling parallel execution
- Use `max_threads` to limit resource consumption

## Advanced Usage

### 1. Combining Scripts

You can combine multiple commands in a single script:

```yaml
scripts:
  check-all: "cargo fmt --check && cargo clippy && cargo test"
```

### 2. Environment Variables

Scripts can use environment variables:

```yaml
scripts:
  build: "RUSTFLAGS='-C target-cpu=native' cargo build --release"
```

Or with advanced configuration:

```yaml
scripts:
  build-optimized:
    commands:
      - command: "cargo build --release"
        env:
          RUSTFLAGS: "-C target-cpu=native"
          CARGO_PROFILE_RELEASE_LTO: "true"
```

### 3. Working Directory

When running scripts through hooks, you can specify a working directory:

```yaml
hooks:
  pre-commit:
    - command: ${test}
      working_dir: tests
```

## Troubleshooting

### Common Issues

1. **Script Not Found**

   - Verify script name: `thira scripts list`
   - Check `hooks.yaml` configuration
   - Ensure correct spelling

2. **Permission Issues**

   - Check file permissions
   - Verify executable paths
   - Use absolute paths when necessary

3. **Environment Problems**

   - Verify environment variables
   - Check PATH configuration
   - Validate dependencies

4. **Parallel Execution Issues** (Experimental)
   - Check system resource limits
   - Verify thread configuration
   - Monitor for race conditions between commands
   - Review interleaved output for errors

### Debugging Scripts

1. Run scripts manually first:

   ```sh
   thira scripts run <name>
   ```

2. Check script output:

   ```sh
   RUST_LOG=debug thira scripts run <name>
   ```

3. Verify script configuration:

   ```sh
   cat hooks.yaml
   ```

4. For parallel scripts, test with `parallel: false` first to isolate issues

## Next Steps

- Learn about [Hook Management](hook-management.md)
- Read [Commit Guidelines](commit-guidelines.md)
- Explore advanced configuration options

For more detailed information about any command, use:

```sh
thira scripts --help
```
