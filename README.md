# Thira - Git Hooks Manager

[![Crates.io](https://img.shields.io/crates/v/thira.svg)](https://crates.io/crates/thira)
[![NPM Version](https://img.shields.io/npm/v/thira.svg)](https://www.npmjs.com/package/thira)
[![PyPI](https://img.shields.io/pypi/v/thira.svg)](https://pypi.org/project/thira/)

Thira is a Git hooks manager and commit message linter that helps you maintain consistent commit messages and automate your Git workflows.

## Features

- **Easy Git hooks management**: Install, update, and remove Git hooks with a single command. Supports all standard Git hook events and custom hooks directories.
- **Conventional commit message validation**: Enforces customizable commit message rules based on the Conventional Commits standard, including type, scope, and subject/body length.
- **Custom script automation**: Define and run custom scripts for any workflow, with support for sequential or parallel execution, environment variables, and working directories.
- **Flexible YAML configuration**: Manage hooks, scripts, and linting rules in a single, human-readable `hooks.yaml` file.
- **Cross-platform support**: Works seamlessly on macOS, Linux, and Windows.
- **Colorful and informative CLI**: User-friendly command-line interface with colored output, helpful error messages, and command trees.
- **Auto-installation and validation**: Automatically installs hooks and validates configuration integrity for a smooth developer experience.
- **Real-time terminal UI**: Displays live output and progress for long-running scripts and hook executions.
- **Integrates with Cargo, npm, and pip**: Installable via multiple package managers for easy adoption in any workflow.

## Documentation

Our documentation is split into several sections for easier navigation:

- [Installation Guide](https://github.com/ervan0707/thira/blob/main/docs/installation.md) - How to install and set up Thira
- [Basic Usage](https://github.com/ervan0707/thira/blob/main/docs/basic-usage.md) - Getting started with Thira
- [Configuration Guide](https://github.com/ervan0707/thira/blob/main/docs/configuration.md) - Complete reference for hooks.yaml configuration
- [Hook Management](https://github.com/ervan0707/thira/blob/main/docs/hook-management.md) - Managing Git hooks
- [Script Management](https://github.com/ervan0707/thira/blob/main/docs/script-management.md) - Creating and managing custom scripts
- [Commit Message Guidelines](https://github.com/ervan0707/thira/blob/main/docs/commit-guidelines.md) - How to write valid commit messages

## Quick Start

```sh
# Install with installer
curl -sSL https://raw.githubusercontent.com/ervan0707/thira/main/install.sh | bash

# Install with Cargo
cargo install thira

# Install with NPM
npm install -g thira
# The `thira` NPM package is currently marked as a security holding package and is not available for installation. Please use the Cargo or pip installation methods for now.

# Install with pip
pip install thira

# Initialize configuration
thira hooks init

# Install Git hooks
thira hooks install
```

Your Git hooks are now managed by Thira! Check out the [Basic Usage](https://github.com/ervan0707/thira/blob/main/docs/basic-usage.md) guide for more details.

## Project Status

This project is under active development. Feel free to open issues and submit pull requests.

## License

This project is licensed under the MIT License – see the [LICENSE](https://github.com/ervan0707/thira/blob/main/LICENSE) file for details.
