#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "Please provide a tag..."
    echo "Usage: ./release.sh v0.1.0"
    exit
fi

branch_name=$(echo "${1}" | sed 's/\.//g')
git checkout -b "release_${branch_name}"

# Update the version
msg="# managed by release.sh"
sed -i '' "s/^version = .* $msg$/version = \"${1#v}\" $msg/" Cargo.toml

# Update the changelog
git-cliff --tag "$1" > CHANGELOG.md
# Remove the last "\n" symbol from the changelog
sed -i '' '$d' CHANGELOG.md

# Commit changes
git add CHANGELOG.md Cargo.toml
git commit -m "chore(release): $1"
git tag "$1"

echo "Done."
echo "Now push the commit and tag to GitHub."
