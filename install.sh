#!/bin/sh
set -e


# check OS and Arch
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [ "$ARCH" = "x86_64" ]; then
    ARCH="x86_64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH="arm64"
else
    echo "Error: Unsupported architecture: $ARCH"
    exit 1
fi

if [ "$OS" = "darwin" ]; then
    TARGET="macos-$ARCH"
elif [ "$OS" = "linux" ]; then
    TARGET="linux-$ARCH"
else
    echo "Error: Unsupported OS: $OS"
    exit 1
fi

REPO="alhaymex/sounds"
BINARY_NAME="sounds"

echo "Fetching latest release for $TARGET..."

LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "Error: Could not fetch latest release tag."
    exit 1
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/${BINARY_NAME}-${LATEST_RELEASE}-${TARGET}.tar.gz"

TMP_DIR=$(mktemp -d)
echo "Downloading $DOWNLOAD_URL..."
curl -sSL "$DOWNLOAD_URL" | tar -xz -C "$TMP_DIR"

echo "Installing $BINARY_NAME to /usr/local/bin..."
if [ -w /usr/local/bin ]; then
    mv "$TMP_DIR/$BINARY_NAME" /usr/local/bin/$BINARY_NAME
else
    echo "Requesting sudo permissions to install to /usr/local/bin"
    sudo mv "$TMP_DIR/$BINARY_NAME" /usr/local/bin/$BINARY_NAME
fi

rm -rf "$TMP_DIR"
echo "Successfully installed $BINARY_NAME version $LATEST_RELEASE!"
