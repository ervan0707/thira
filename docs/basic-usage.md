# Basic Usage Guide

## Overview

Thira provides a simple and intuitive CLI for managing Git hooks and scripts. This guide covers the basic commands you'll need to get started.

## Command Structure

thira commands follow this general structure:

```sh
thira <command-group> <action> [options]
```

Command groups:

- `hooks` - Git hooks management
- `scripts` - Custom script management
- `commit` - Commit message validation

## Common Commands

### Viewing Available Commands

To see all available commands:

```sh
thira
```

To get help for a specific command:

```sh
thira <command> --help
```

### Hook Management Basics

1. Initialize configuration:

   ```sh
   thira hooks init
   ```

2. Install Git hooks:

   ```sh
   thira hooks install
   ```

3. List configured hooks:
   ```sh
   thira hooks list
   ```

### Script Management Basics

1. Add a new script:

   ```sh
   thira scripts add <name> <command>
   ```

2. Run a script:

   ```sh
   thira scripts run <name>
   ```

3. List all scripts:
   ```sh
   thira scripts list
   ```

## Configuration File

After initialization, thira creates a `hooks.yaml` file in your project root. This file contains:

- Hook configurations
- Custom script definitions
- Commit message rules
- General options

Example configuration:

```yaml
hooks:
  pre-commit:
    - command: cargo
      args:
        - test
    - command: cargo
      args:
        - clippy

scripts:
  lint: cargo clippy
  test: cargo test

options:
  auto_install: true
  hooks_dir: .thira

lint:
  types:
    - feat
    - fix
    - docs
  scopes:
    - api
    - ui
    - db
  min_subject_length: 3
  max_subject_length: 72

options:
  # Automatically reinstall hooks when config changes
  auto_install: true
  # Directory where hook files are stored
  hooks_dir: .thira
```

### Configuration Options

#### auto_install

When `true`, thira will automatically reinstall hooks whenever:

- The configuration file is modified and saved
- Scripts are added or removed
- Hooks are added or modified

This ensures your Git hooks always reflect your latest configuration. If `false`, you'll need to manually run `thira hooks install` after making changes.

#### hooks_dir

Specifies where hook files are stored. Default is `.thira`. Can be set to `.git/hooks` to use the traditional Git hooks directory.

## Next Steps

- Learn more about [Hook Management](hook-management.md)
- Explore [Script Management](script-management.md)
- Understand [Commit Message Guidelines](commit-guidelines.md)

For detailed explanations of any command, use the `--help` flag:

```sh
thira <command-group> <action> --help
```
