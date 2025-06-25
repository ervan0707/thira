# Installation Guide

Thira can be installed through multiple package managers, via our install script, or using Nix.

## Installation Methods

### 1. Using Install Script (Recommended)

The easiest way to install Thira is through our install script:

```sh
curl -sSL https://raw.githubusercontent.com/ervan0707/thira/main/install.sh | bash
```

### 2. Using Package Managers

#### Cargo (Rust)

```sh
cargo install thira
```

#### NPM (Node.js)

```sh
npm install -g thira
# The `thira` NPM package is currently marked as a security holding package and is not available for installation. Please use the Cargo or pip installation methods for now.
```

#### PyPI (Python)

```sh
pip install thira
```

### 3. Using Nix

#### As a Flake (Recommended)

```sh
# Run directly
nix run github:ervan0707/thira

# Install into your profile
nix profile install github:ervan0707/thira

# Add to your NixOS configuration
{
  inputs.thira.url = "github:ervan0707/thira";

  # Add to your system packages
  environment.systemPackages = [ inputs.thira.packages.${system}.default ];
}
```

#### Development Shell

To enter a development environment with all dependencies:

```sh
# Using flakes
nix develop github:ervan0707/thira

# Or clone the repository and run
git clone https://github.com/ervan0707/thira.git
cd thira
nix develop
```

#### Building with Nix

You can build the package from source using Nix:

```sh
# Build the package
nix build github:ervan0707/thira

# Or after cloning the repository
git clone https://github.com/ervan0707/thira.git
cd thira
nix build

# The built binary will be available in ./result/bin/thira
```

### 4. Building from Source

If you want to build from source:

```sh
# Clone the repository
git clone https://github.com/ervan0707/thira.git
cd thira

# Build and install
cargo build --release
cargo install --path .
```

## Verifying Installation

After installation, verify that thira is properly installed:

```sh
thira --version
```

You should see the version number of thira displayed.

## Initial Setup

1. Initialize a new Git repository (if you haven't already):

   ```sh
   git init
   ```

2. Initialize thira's configuration:

   ```sh
   thira hooks init
   ```

   This will create a `hooks.yaml` file in your project root.

3. Install the Git hooks:
   ```sh
   thira hooks install
   ```

## Configuration Location

thira uses two main configuration locations:

- `hooks.yaml` - Project-specific hook configurations
- `.thira/` - Default directory for hook scripts (configurable)

## Troubleshooting

### Common Issues

1. **Command not found**

   - Ensure Cargo's bin directory is in your PATH
   - Try restarting your terminal

2. **Git hooks not running**

   - Check if hooks are installed: `thira hooks list`
   - Verify hooks path: `thira hooks show-path`
   - Ensure hook files are executable

3. **Permission denied**
   - Check file permissions in your `.thira` directory
   - Ensure you have write access to the Git hooks directory

### Getting Help

If you encounter any issues:

1. Run commands with more verbosity:

   ```sh
   RUST_LOG=debug thira <command>
   ```

2. Check your Git hooks path:

   ```sh
   thira hooks show-path
   ```

3. Reset to default configuration:
   ```sh
   thira hooks reset-path
   ```

For more help, visit our [GitHub repository](https://github.com/ervan0707/thira/issues).
