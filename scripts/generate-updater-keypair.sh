#!/bin/bash

# Script to generate Tauri updater keypair
# This will create a private key (keep it secret!) and a public key (add to tauri.conf.json)

echo "Generating Tauri updater keypair..."

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check if we're in the project root
if [ ! -f "$PROJECT_DIR/package.json" ]; then
    echo "Error: Please run this script from the project root directory"
    exit 1
fi

# Check if pnpm is available
if ! command -v pnpm &> /dev/null; then
    echo "Error: pnpm is not installed."
    echo "Please install it with: npm install -g pnpm"
    exit 1
fi

# Create .tauri directory if it doesn't exist
mkdir -p ~/.tauri

# Generate keypair using pnpm tauri signer generate
echo "Running: pnpm tauri signer generate -w ~/.tauri/myapp.key"
echo ""
echo "Note: You will be prompted to enter a password to protect the private key."
echo "      You can press Enter to skip password protection (not recommended for production)."
echo ""
cd "$PROJECT_DIR"

# Check if key already exists
if [ -f ~/.tauri/myapp.key ]; then
    echo "⚠️  Warning: ~/.tauri/myapp.key already exists!"
    read -p "Do you want to overwrite it? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 0
    fi
    # Use --force flag to overwrite
    FORCE_FLAG="--force"
else
    FORCE_FLAG=""
fi

# Run the command interactively (user needs to enter password)
echo "Generating keypair..."
if pnpm tauri signer generate -w ~/.tauri/myapp.key $FORCE_FLAG; then
    echo "✅ Private key generated successfully!"
else
    echo "Error: Failed to generate keypair"
    exit 1
fi

# Try to extract public key from the command output
# Since we can't capture interactive output easily, we'll try to read it from files
PUBLIC_KEY=""

# Try to read from .pub file first
if [ -f ~/.tauri/myapp.key.pub ]; then
    PUBLIC_KEY=$(cat ~/.tauri/myapp.key.pub | tr -d '\n\r ')
fi

# If still not found, try to extract from the key file itself
if [ -z "$PUBLIC_KEY" ] && [ -f ~/.tauri/myapp.key ]; then
    # The public key might be in the key file, try to extract it
    # Tauri stores keys in a specific format, look for base64 strings
    PUBLIC_KEY=$(grep -oE '[a-zA-Z0-9+/=]{40,}' ~/.tauri/myapp.key | head -1 || echo "")
fi

# If we still don't have it, the public key should have been printed to stdout
# We'll need to ask the user to check the output above

if [ -z "$PUBLIC_KEY" ]; then
    echo ""
    echo "⚠️  Keypair generated, but could not extract public key automatically."
    echo ""
    echo "The public key should have been displayed above. Please:"
    echo "1. Look for a line containing 'Public key:' in the output above"
    echo "2. Copy the public key value"
    echo "3. Update the 'pubkey' field in src-tauri/tauri.conf.json"
    echo ""
    echo "Alternatively, you can run the command again to see the public key:"
    echo "   pnpm tauri signer generate -w ~/.tauri/myapp.key --force"
else
    echo ""
    echo "✅ Keypair generated successfully!"
    echo ""
    echo "⚠️  IMPORTANT: Keep the private key secret!"
    echo "   Private key location: ~/.tauri/myapp.key"
    if [ -f ~/.tauri/myapp.key.pub ]; then
        echo "   Public key location: ~/.tauri/myapp.key.pub"
    fi
    echo ""
    echo "📋 Public key (add this to src-tauri/tauri.conf.json):"
    echo "   $PUBLIC_KEY"
    echo ""
    echo "Next steps:"
    echo "1. Update 'pubkey' in src-tauri/tauri.conf.json with the public key above"
    echo "2. Keep ~/.tauri/myapp.key secure and never commit it to git"
    echo "3. Use this keypair when building releases with:"
    echo "   TAURI_SIGNING_PRIVATE_KEY=\"\$(cat ~/.tauri/myapp.key)\" pnpm tauri build"
    echo "   (If the key has a password, also set TAURI_SIGNING_PRIVATE_KEY_PASSWORD)"
fi

