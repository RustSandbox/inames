#!/bin/bash

# 🦀 Remolab Rust Template Publisher
# This script helps publish the template to GitHub

set -e

echo "🦀 Publishing Remolab Rust Template..."
echo ""

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Not in a git repository. Initializing..."
    git init
fi

# Add all files
echo "📦 Adding all template files..."
git add .

# Commit changes
echo "💾 Committing template..."
git commit -m "🦀 Remolab Rust Template - Ready for use

Features:
✅ Full async runtime (tokio)
✅ CLI framework (clap)
✅ Error handling (anyhow)
✅ Serialization (serde)
✅ AI/LLM ready (rig-core)
✅ Schema generation (schemars)
✅ Structured logging (tracing)
✅ Docker support
✅ CI/CD pipeline
✅ Development automation (just)
✅ GitHub repo creation

Template created by Remolab 🚀"

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI not found. Please install it:"
    echo "   brew install gh"
    echo "   gh auth login"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo "❌ Not authenticated with GitHub. Please run:"
    echo "   gh auth login"
    exit 1
fi

# Get repository name
REPO_NAME=$(basename "$PWD")
echo "📡 Creating GitHub repository: $REPO_NAME"

# Create repository and push
gh repo create "$REPO_NAME" \
    --public \
    --description "🦀 Ultimate Rust project template with async runtime, AI/LLM support, Docker, CI/CD, and automation. Created by Remolab." \
    --source=. \
    --remote=origin \
    --push

echo ""
echo "✅ Template published successfully!"
echo "🌐 Repository: https://github.com/$(gh api user --jq .login)/$REPO_NAME"
echo ""
echo "📋 Users can now create projects with:"
echo "   cargo generate --git https://github.com/$(gh api user --jq .login)/$REPO_NAME.git --name my-project"
echo ""
echo "🎉 Happy coding with Remolab!" 