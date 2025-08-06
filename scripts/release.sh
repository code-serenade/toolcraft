#!/bin/bash
# Release script for toolcraft packages
# 
# This script helps manage independent versioning for each crate in the workspace
# It updates the version and runs cargo release for the specified package
#
# Usage: ./scripts/release.sh <package-name> <version> [--dry-run]
# Example: ./scripts/release.sh toolcraft-jwt 0.2.1
# Example: ./scripts/release.sh toolcraft-jwt 0.2.1 --dry-run

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PACKAGE=$1
VERSION=$2
DRY_RUN=$3

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check arguments
if [ -z "$PACKAGE" ] || [ -z "$VERSION" ]; then
    print_error "Missing required arguments"
    echo "Usage: ./scripts/release.sh <package-name> <version> [--dry-run]"
    echo ""
    echo "Available packages:"
    echo "  - toolcraft-jwt"
    echo "  - toolcraft-axum-kit"
    echo "  - toolcraft-request"
    echo "  - toolcraft-config"
    echo "  - toolcraft-utils"
    echo ""
    echo "Example:"
    echo "  ./scripts/release.sh toolcraft-jwt 0.2.1"
    echo "  ./scripts/release.sh toolcraft-jwt 0.2.1 --dry-run"
    exit 1
fi

# Check if package exists
if [ ! -d "crates/$PACKAGE" ]; then
    print_error "Package '$PACKAGE' not found in crates/"
    exit 1
fi

# Check current version
CURRENT_VERSION=$(grep '^version = ' "crates/$PACKAGE/Cargo.toml" | sed 's/version = "\(.*\)"/\1/')
print_info "Current version of $PACKAGE: $CURRENT_VERSION"
print_info "New version will be: $VERSION"

# Dependency check for toolcraft-axum-kit
if [ "$PACKAGE" = "toolcraft-axum-kit" ]; then
    JWT_VERSION=$(grep 'toolcraft-jwt.*version' "crates/$PACKAGE/Cargo.toml" | sed 's/.*version = "\([^"]*\)".*/\1/')
    print_warning "toolcraft-axum-kit depends on toolcraft-jwt version: $JWT_VERSION"
    print_warning "Make sure toolcraft-jwt $JWT_VERSION is already published!"
    read -p "Continue? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Release cancelled"
        exit 0
    fi
fi

# Update version in Cargo.toml
print_info "Updating version in Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" "crates/$PACKAGE/Cargo.toml"
rm "crates/$PACKAGE/Cargo.toml.bak"

# Show the change
print_info "Version updated in Cargo.toml"

# Change to package directory
cd "crates/$PACKAGE"

# Run cargo release
if [ "$DRY_RUN" = "--dry-run" ]; then
    print_info "Running cargo release in dry-run mode..."
    cargo release --dry-run
else
    print_info "Running cargo release..."
    print_warning "This will publish to crates.io!"
    read -p "Are you sure you want to publish $PACKAGE v$VERSION? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cargo release --execute
        print_info "Release complete for $PACKAGE v$VERSION"
        print_info "Don't forget to:"
        print_info "  1. Push the changes: git push"
        print_info "  2. Push the tags: git push --tags"
    else
        print_info "Release cancelled"
        # Revert version change
        git checkout Cargo.toml
    fi
fi