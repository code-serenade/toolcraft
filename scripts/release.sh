#!/bin/bash
# Release script for toolcraft packages

set -e

PACKAGE=$1
VERSION=$2

if [ -z "$PACKAGE" ] || [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/release.sh <package-name> <version>"
    echo "Example: ./scripts/release.sh toolcraft-jwt 0.2.1"
    exit 1
fi

echo "Releasing $PACKAGE version $VERSION"

# Change to package directory
cd "crates/$PACKAGE"

# Update version in Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Run cargo release
cargo release --execute

echo "Release complete for $PACKAGE v$VERSION"