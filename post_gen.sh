#!/bin/bash

set -e
PROJECT_NAME=$(basename "$PWD")

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if GitHub CLI is installed
if ! command_exists gh; then
    echo "❌ GitHub CLI (gh) is not installed."
    echo ""
    echo "📦 To install GitHub CLI:"
    echo ""
    echo "🍎 On macOS (using Homebrew):"
    echo "   brew install gh"
    echo ""
    echo "🐧 On Linux (Ubuntu/Debian):"
    echo "   curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg"
    echo "   echo \"deb [arch=\$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main\" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null"
    echo "   sudo apt update"
    echo "   sudo apt install gh"
    echo ""
    echo "🪟 On Windows:"
    echo "   winget install --id GitHub.cli"
    echo "   # or download from: https://github.com/cli/cli/releases"
    echo ""
    echo "After installation, run:"
    echo "   gh auth login"
    echo "   ./post_gen.sh"
    echo ""
    exit 1
fi

# Check if user is authenticated with GitHub
if ! gh auth status >/dev/null 2>&1; then
    echo "❌ You are not authenticated with GitHub CLI."
    echo ""
    echo "🔐 Please run:"
    echo "   gh auth login"
    echo ""
    echo "Then run this script again:"
    echo "   ./post_gen.sh"
    echo ""
    exit 1
fi

echo "🚀 Creating Git repository and pushing to GitHub..."

git init
git add .
git commit -m "Initial commit from Hamze's custom Rust template"

# Rename the current branch to main (if not already)
git branch -M main

gh repo create "$PROJECT_NAME" \
  --private \
  --source=. \
  --remote=origin \
  --push

echo "✅ Repo '$PROJECT_NAME' created and pushed to GitHub."
echo "🌐 Repository URL: https://github.com/$(gh api user --jq .login)/$PROJECT_NAME" 