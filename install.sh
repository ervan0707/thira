#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# GitHub repository information
REPO_OWNER="ervan0707"
REPO_NAME="thira"
GITHUB_API="https://api.github.com"

# Print error message and exit
error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

# Print info message
info() {
    echo -e "${GREEN}$1${NC}"
}

# Print warning message
warn() {
    echo -e "${YELLOW}Warning: $1${NC}"
}

# Print shell configuration help
print_shell_config() {
    local install_dir="$1"

    echo -e "\n${BLUE}Installation Directory: ${install_dir}${NC}"
    echo -e "${YELLOW}Note: This directory needs to be added to your PATH${NC}"
    echo -e "\nChoose one of these methods to add it:\n"

    # Fish Shell Configuration
    echo -e "${BLUE}For Fish shell:${NC}"
    echo "1. Immediate use:"
    echo -e "${GREEN}    fish_add_path $install_dir${NC}"
    echo "2. Add to ~/.config/fish/config.fish:"
    echo -e "${GREEN}    if test -d $install_dir
    fish_add_path $install_dir
end${NC}"
    echo "3. Using Home Manager:"
    echo -e "${GREEN}    programs.fish = {
      enable = true;
      interactiveShellInit = ''
        if test -d $install_dir
          fish_add_path $install_dir
        end
      '';
    };${NC}"

    # Bash Shell Configuration
    echo -e "\n${BLUE}For Bash shell:${NC}"
    echo "1. Immediate use:"
    echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
    echo "2. Add to ~/.bashrc (Linux) or ~/.bash_profile (macOS):"
    echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
    echo "3. Using Home Manager:"
    echo -e "${GREEN}    programs.bash = {
      enable = true;
      initExtra = ''
        export PATH=\"\$PATH:$install_dir\"
      '';
    };${NC}"

    # Zsh Shell Configuration
    echo -e "\n${BLUE}For Zsh shell:${NC}"
    echo "1. Immediate use:"
    echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
    echo "2. Add to ~/.zshrc:"
    echo -e "${GREEN}    export PATH=\"\$PATH:$install_dir\"${NC}"
    echo "3. Using Home Manager:"
    echo -e "${GREEN}    programs.zsh = {
      enable = true;
      initExtra = ''
        export PATH=\"\$PATH:$install_dir\"
      '';
    };${NC}"

    # Nix Home Manager (shell-agnostic approach)
    echo -e "\n${BLUE}Shell-agnostic Home Manager approach:${NC}"
    echo -e "${GREEN}    home = {
      sessionPath = [
        \"$install_dir\"
      ];
    };${NC}"

    echo -e "\n${YELLOW}After adding the PATH:${NC}"
    echo "• Start a new terminal session, or"
    echo "• Reload your shell configuration"

    echo -e "\n${BLUE}To verify the installation:${NC}"
    echo "• Run: thira --help"
    echo -e "• If it doesn't work, make sure the PATH is properly set: echo \$PATH\n"
}

# Detect OS and architecture
detect_platform() {
    # Detect OS
    case "$(uname -s)" in
        Linux*)     OS=linux;;
        Darwin*)    OS=darwin;;
        MINGW64*)   OS=windows;;
        MSYS*)      OS=windows;;
        *)          error "Unsupported operating system: $(uname -s)";;
    esac

    # Detect architecture
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64|amd64) ARCH=x86_64;;
        aarch64|arm64) ARCH=arm64;;
        *) error "Unsupported architecture: $arch";;
    esac

    # Set asset extension
    if [ "$OS" = "windows" ]; then
        EXT=".zip"
    else
        EXT=".tar.gz"
    fi
}

# Get the latest release version
get_latest_version() {
    info "Fetching latest release version..."
    LATEST_VERSION=$(curl -sL ${GITHUB_API}/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest | grep '"tag_name":' | cut -d'"' -f4)
    [ -z "$LATEST_VERSION" ] && error "Failed to fetch latest version"
    info "Latest version: $LATEST_VERSION"
}

# Download and install the binary
install_binary() {
    local asset_name="thira-${OS}-${ARCH}${EXT}"
    local download_url="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${LATEST_VERSION}/${asset_name}"
    local temp_dir
    temp_dir=$(mktemp -d)
    local install_dir="$HOME/.local/bin"

    info "Downloading $asset_name..."
    curl -sL -o "${temp_dir}/${asset_name}" "$download_url" || error "Failed to download binary"

    # Create install directory and its parent directories
    info "Creating install directory: $install_dir"
    mkdir -p "$install_dir" || error "Failed to create install directory"

    # Extract and install binary
    cd "$temp_dir"
    info "Extracting binary..."
    if [ "$OS" = "windows" ]; then
        unzip -q "${asset_name}" || error "Failed to extract binary"
        mv thira.exe "$install_dir/" || error "Failed to install binary"
    else
        tar xzf "${asset_name}" || error "Failed to extract binary"
        # List contents of temp directory for debugging
        info "Extracted contents:"
        ls -la

        # Try to find the binary
        if [ -f "thira-${OS}-${ARCH}" ]; then
            mv "thira-${OS}-${ARCH}" "$install_dir/thira" || error "Failed to install binary"
        elif [ -f "thira" ]; then
            mv "thira" "$install_dir/thira" || error "Failed to install binary"
        else
            error "Could not find binary after extraction. Contents: $(ls -la)"
        fi

        chmod +x "$install_dir/thira" || error "Failed to set executable permissions"
    fi

    # Clean up
    rm -rf "$temp_dir"

    info "Installation successful!"
    info "Installed to: $install_dir/thira"

    # Check if install directory is in PATH and provide ALL shell instructions
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        print_shell_config "$install_dir"
    fi
}

main() {
    # Check for required commands
    command -v curl >/dev/null 2>&1 || error "curl is required but not installed"
    command -v tar >/dev/null 2>&1 || error "tar is required but not installed"

    info "Installing thira..."
    detect_platform
    get_latest_version
    install_binary

    info "Installation complete! Make sure to add the binary location to your PATH if needed."
}

main