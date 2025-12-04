#!/bin/bash
# ANTIMONY COIN 2.0 - GitHub Push Script
# This script will push the local repository to GitHub

set -e

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║    ANTIMONY COIN 2.0 - GITHUB REPOSITORY PUSH               ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Configuration
GITHUB_USER="msrusu87-web"
REPO_NAME="antimony-2.0"
GITHUB_URL="https://github.com/${GITHUB_USER}/${REPO_NAME}.git"

echo "📋 Repository Details:"
echo "  Owner:     ${GITHUB_USER}"
echo "  Repo:      ${REPO_NAME}"
echo "  URL:       ${GITHUB_URL}"
echo ""

# Check if git is configured
echo "🔍 Checking git configuration..."
if [ -z "$(git config user.name)" ]; then
    echo "❌ Git user not configured globally"
    echo "   Run: git config --global user.name 'Your Name'"
    exit 1
fi

if [ -z "$(git config user.email)" ]; then
    echo "❌ Git email not configured globally"
    echo "   Run: git config --global user.email 'your@email.com'"
    exit 1
fi

echo "✅ Git user: $(git config user.name)"
echo "✅ Git email: $(git config user.email)"
echo ""

# Check if remote already exists
echo "🔗 Configuring remote origin..."
if git remote get-url origin &> /dev/null; then
    echo "⚠️  Remote 'origin' already exists:"
    echo "   $(git remote get-url origin)"
    echo ""
    read -p "   Update remote? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git remote set-url origin "${GITHUB_URL}"
        echo "✅ Remote URL updated"
    else
        echo "⏭️  Skipping remote configuration"
    fi
else
    git remote add origin "${GITHUB_URL}"
    echo "✅ Remote 'origin' added"
fi
echo ""

# Rename master to main if needed
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
echo "🌳 Checking branch..."
echo "   Current branch: ${CURRENT_BRANCH}"

if [ "${CURRENT_BRANCH}" = "master" ]; then
    echo "   Renaming 'master' to 'main'..."
    git branch -M main
    echo "✅ Branch renamed to 'main'"
else
    echo "✅ Already on 'main' or other branch"
fi
echo ""

# Show commits to be pushed
echo "📊 Commits to push:"
git log --oneline -n 5
echo ""

# Verify all files are committed
echo "🔐 Checking for uncommitted changes..."
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Uncommitted changes detected:"
    git status --porcelain
    echo ""
    read -p "   Commit changes? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git add -A
        git commit -m "chore: final cleanup before push"
        echo "✅ Changes committed"
    fi
else
    echo "✅ No uncommitted changes"
fi
echo ""

# Push to GitHub
echo "🚀 Ready to push to GitHub!"
echo ""
echo "   Repository: ${GITHUB_URL}"
echo "   Branch:     main"
echo ""

read -p "   Proceed with push? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "⏳ Pushing to GitHub..."
    git push -u origin main
    
    echo ""
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║          ✅ PUSH SUCCESSFUL!                                ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo ""
    echo "🎉 Your repository is now live at:"
    echo "   ${GITHUB_URL}"
    echo ""
    echo "📝 Next steps:"
    echo "   1. Visit GitHub and configure repository settings"
    echo "   2. Enable GitHub Pages for documentation"
    echo "   3. Create GitHub Discussions for community"
    echo "   4. Setup branch protection rules"
    echo "   5. Invite team members"
    echo ""
    echo "📱 Repository URL:"
    echo "   https://github.com/${GITHUB_USER}/${REPO_NAME}"
    echo ""
else
    echo "❌ Push cancelled"
    exit 1
fi
