#!/bin/sh
set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

# Status helpers
info()    { printf "  ${CYAN}▸${NC} %b\n" "$1"; }
success() { printf "  ${GREEN}✔${NC} %b\n" "$1"; }
error()   { printf "  ${RED}✖${NC} %b\n" "$1"; exit 1; }

# Spinner
spin() {
    local pid=$1 msg=$2
    local chars='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    while kill -0 "$pid" 2>/dev/null; do
        for i in $(seq 0 $((${#chars} - 1))); do
            printf "\r  ${CYAN}%s${NC} %b" "$(echo "$chars" | cut -c$((i+1)))" "$msg"
            sleep 0.08
        done
    done
    printf "\r  ${GREEN}✔${NC} %b\n" "$msg"
}

# Banner
printf "${BOLD}${CYAN}"
cat << 'EOF'

 ▗▄▄▖ ▗▄▖ ▗▖ ▗▖▗▖  ▗▖▗▄▄▄   ▗▄▄▖
▐▌   ▐▌ ▐▌▐▌ ▐▌▐▛▚▖▐▌▐▌  █ ▐▌   
 ▝▀▚▖▐▌ ▐▌▐▌ ▐▌▐▌ ▝▜▌▐▌  █  ▝▀▚▖
▗▄▄▞▘▝▚▄▞▘▝▚▄▞▘▐▌  ▐▌▐▙▄▄▀ ▗▄▄▞▘

EOF
printf "${NC}"

# Detect OS and Arch
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [ "$ARCH" = "x86_64" ]; then
    ARCH="x86_64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH="arm64"
else
    error "Unsupported architecture: $ARCH"
fi

if [ "$OS" = "darwin" ]; then
    TARGET="macos-$ARCH"
elif [ "$OS" = "linux" ]; then
    TARGET="linux-$ARCH"
else
    error "Unsupported OS: $OS"
fi

REPO="alhaymex/sounds"
BINARY_NAME="sounds"

# Fetch latest release
info "Fetching latest release for ${BOLD}$TARGET${NC}..."

LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    error "Could not fetch latest release tag."
fi

success "Found ${BOLD}$LATEST_RELEASE${NC}"

# Download and extract
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/${BINARY_NAME}-${LATEST_RELEASE}-${TARGET}.tar.gz"
TMP_DIR=$(mktemp -d)

curl -sSL "$DOWNLOAD_URL" | tar -xz -C "$TMP_DIR" &
spin $! "Downloading ${BOLD}sounds $LATEST_RELEASE${NC}"

# Install
info "Installing to ${BOLD}/usr/local/bin${NC}..."
if [ -w /usr/local/bin ]; then
    mv "$TMP_DIR/$BINARY_NAME" /usr/local/bin/$BINARY_NAME
else
    sudo mv "$TMP_DIR/$BINARY_NAME" /usr/local/bin/$BINARY_NAME
fi

rm -rf "$TMP_DIR"

printf "\n"
success "${BOLD}sounds $LATEST_RELEASE${NC} installed successfully! 🎵"
printf "  ${DIM}Run 'sounds' to get started.${NC}\n\n"
