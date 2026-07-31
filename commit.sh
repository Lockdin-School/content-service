#!/bin/bash

# Lockdin Commit Helper
# Usage: ./commit.sh

echo "Lockdin Commit Helper"
echo "------------------------"

# Type
echo "Type: feat | fix | refactor | perf | docs | style | chore | ci | revert"
read -p "→ Type: " TYPE

# Scope
echo "Scope: config | service | model | domain | routes | api | deps | infra | docker | db"
read -p "→ Scope: " SCOPE

# Description
read -p "→ Short description: " DESC

# Body
read -p "→ Body (optional, enter to skip): " BODY

# Breaking change
read -p "→ Breaking change? (y/n): " BREAKING

# Build message
MESSAGE="$TYPE($SCOPE): $DESC"

if [ -n "$BODY" ]; then
    MESSAGE="$MESSAGE

$BODY"
fi

if [ "$BREAKING" = "y" ]; then
    read -p "→ Describe the breaking change: " BREAK_DESC
    MESSAGE="$MESSAGE

BREAKING CHANGE: $BREAK_DESC"
fi

# Stage and commit
git add .
git commit -m "$MESSAGE"

echo ""
echo "✓ $TYPE($SCOPE): $DESC"