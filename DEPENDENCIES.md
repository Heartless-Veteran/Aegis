# Dependency Management

This document explains how dependencies are managed in the Aegis project.

## Overview

We use **Renovate** as our primary dependency management tool, with **Dependabot** as a backup. Both tools automatically detect dependencies and create pull requests to update them.

## Detected Dependencies

### GitHub Actions
The following GitHub Actions are used across our CI/CD workflows:

- `actions/checkout@v5` - Repository checkout
- `actions/cache@v4` - Build artifact caching  
- `actions/upload-artifact@v4` - Artifact uploads
- `actions/download-artifact@v4` - Artifact downloads
- `actions/configure-pages@v5` - GitHub Pages setup
- `actions/deploy-pages@v4` - GitHub Pages deployment
- `actions/upload-pages-artifact@v3` - Pages artifact uploads
- `actions/dependency-review-action@v4` - Dependency security review
- `actions/create-release@v1` - Release creation
- `actions/upload-release-asset@v1` - Release asset uploads

### Container Images
- **DevContainer**: `mcr.microsoft.com/devcontainers/rust:1-1-bullseye`
- **Release Docker**: Multi-stage builds using `rust:1.75-slim` and `debian:bookworm-slim`

### Rust Dependencies
Managed via `Cargo.toml` files in:
- Root workspace (`Cargo.toml`)
- Aegis Compiler (`Aegis_Compiler/Cargo.toml`)
- Aegis LSP (`Aegis_LSP/Cargo.toml`) 
- Aegis Bridge (`Aegis_Bridge/Cargo.toml`)

## Configuration Files

### Renovate (`renovate.json`)
- **Schedule**: Runs before 6am UTC on Mondays
- **Grouping**: GitHub Actions and Rust dependencies are grouped separately
- **Auto-merge**: GitHub Actions updates are auto-merged, others require review
- **Managers**: Supports cargo, github-actions, dockerfile, and devcontainer

### Dependabot (`.github/dependabot.yml`)  
- **Backup system**: Provides redundant dependency tracking
- **Schedule**: Staggered throughout Monday mornings
- **Focus**: Ignores patch-level updates to reduce noise

## Dependency Dashboard

The **Dependency Dashboard** is automatically maintained in GitHub Issues by Renovate. It shows:

- All detected dependencies
- Available updates
- Update status and history
- Manual trigger checkbox for on-demand scans

## Development Workflow

1. **Automatic Detection**: Both Renovate and Dependabot scan for dependencies
2. **Pull Request Creation**: Updates are proposed via PRs with appropriate labels
3. **Automated Testing**: All dependency updates are tested by CI workflows
4. **Review Process**: Non-trivial updates require maintainer review
5. **Auto-merge**: Safe updates (like GitHub Actions) are automatically merged

## Troubleshooting

### If Renovate isn't detecting dependencies:
1. Check the `renovate.json` configuration
2. Verify enabled managers include your dependency type
3. Use the manual trigger in the Dependency Dashboard issue

### If builds fail after updates:
1. Check the CI logs for specific errors
2. Review the dependency changelog for breaking changes
3. Update code to accommodate breaking changes if necessary

## Supported Dependency Types

- ✅ **GitHub Actions**: Fully automated updates
- ✅ **Rust/Cargo**: Semantic version management
- ✅ **Docker Images**: Container base image updates
- ✅ **DevContainer**: Development environment updates
- ⚠️ **npm/Node**: Limited support (only used in devcontainer features)