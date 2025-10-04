# Contributing to dots-cli

Thank you for your interest in contributing to dots-cli! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites
- Rust stable toolchain
- Git

### Getting Started
```bash
# Clone the repository
git clone https://github.com/webdesserts/dots-cli
cd dots-cli

# Run tests
cargo test --all

# Run the CLI locally
cargo run -- <command>
```

## Testing

### Philosophy
dots-cli follows an integration testing approach:
- **Focus on CLI behavior**, not internal implementation details
- **Snapshot testing** for comprehensive output validation
- **Tests should survive refactoring** by testing user-visible behavior

### Running Tests
```bash
# Run all tests
cargo test --all

# Run specific test file
cargo test --test subcommand_install

# Run specific test
cargo test it_should_display_and_install_the_given_plan
```

### Writing Tests
Tests are organized in `tests/` by subcommand:
- `tests/subcommand_install.rs`
- `tests/subcommand_add.rs`
- etc.

Expected output is stored in `tests/output/` as snapshot files.

**Test Naming Convention:**
```rust
#[test]
fn it_should_[action]_[condition]() -> TestResult {
    // Test implementation
}
```

**Example:**
```rust
#[test]
fn it_should_fail_when_footprint_is_readonly() -> TestResult {
    let manager = TestManager::new()?;
    let fixture_path = manager.setup_fixture_as_git_repo(&Fixture::ExampleDot)?;

    manager.cmd(BIN)?.arg("add").arg(&fixture_path).output()?;
    manager.cmd(BIN)?.arg("install").output()?;

    // Make footprint readonly to simulate permission error
    manager.make_readonly(manager.footprint_path())?;

    let output = manager.cmd(BIN)?.arg("install").output()?;
    let expected_err = include_str!("output/install_fail_with_readonly_footprint.err");

    output
        .assert_stderr_eq(expected_err)
        .assert_fail();

    Ok(())
}
```

See `CLAUDE.md` for more detailed testing guidelines.

## Code Quality

### Before Committing
Run these checks locally:
```bash
# Run tests
cargo test --all

# Check for clippy warnings
cargo clippy --all-targets -- -D warnings

# Format code
cargo fmt --all
```

### CI
GitHub Actions runs tests on:
- Ubuntu latest (stable Rust)
- macOS latest (stable Rust)

Checks formatting and clippy on nightly.

## Release Process

### Prerequisites
- `cargo-release` installed: `cargo install cargo-release`
- Write access to the repository
- Clean working directory

### Release Steps

**1. Pre-release checks on version branch** (fix any issues before versioning)
```bash
cargo test --all
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

Commit any fixes if needed.

**2. Update CHANGELOG.md**
- Move unreleased changes to new version section
- Add release date
- Update comparison links at bottom
- Commit the changelog:
```bash
git add CHANGELOG.md
git commit -m "docs: update CHANGELOG for v0.X.X"
```

**3. Bump versions on version branch**
```bash
# This will bump versions and create a commit (but not publish or tag)
cargo release patch --workspace --exclude test_utils --execute --no-publish --no-tag --no-push
```

This updates:
- Version numbers in all workspace `Cargo.toml` files
- Dependencies between workspace packages
- Creates a version bump commit

Push the version branch:
```bash
git push origin v0.5.x
```

**4. Create PR and merge to main**
- Create a PR from your version branch to `main`
- Wait for CI checks to pass
- Merge the PR on GitHub

**5. Publish from main**
After the PR is merged:
```bash
git checkout main
git pull origin main

# Publish packages, create tags, and push
cargo release --workspace --exclude test_utils --execute --no-commit
```

This will:
- Publish `dots_internal_utils` to crates.io
- Publish `dots` to crates.io
- Create and push git tags

**Note**: The `--workspace` flag processes all packages. We exclude `test_utils` since it's internal-only.

### Version Numbering
Version numbers follow the pattern `major.minor.patch`:
- **Patch** (0.5.2 -> 0.5.3): Bug fixes and minor changes
- **Minor** (0.5.3 -> 0.6.0): New features and improvements
- **Major** (0.6.0 -> 1.0.0): Major changes

Note: This project does not strictly adhere to semantic versioning.

### Manual Release (if needed)
If `cargo-release` isn't available:

```bash
# 1. Update versions in Cargo.toml files manually
# 2. Commit version bump
git add Cargo.toml packages/*/Cargo.toml
git commit -m "chore: Release v0.X.X"

# 3. Create PR to main and merge

# 4. From main, publish and tag
cargo publish -p dots_internal_utils
cargo publish -p dots
git tag v0.X.X
git push origin --tags
```

## Branch Strategy

- **main**: Stable releases (publish from this branch)
- **v0.5.x**: Current development branch for 0.5.x releases
- **feature branches**: For new features, merged into version branches via PR to main

## Code Style

- Follow Rust conventions
- Use `rustfmt` for formatting
- Address `clippy` warnings
- Add documentation comments for public APIs
- Keep functions focused and testable

## Questions?

Feel free to open an issue for any questions about contributing!
