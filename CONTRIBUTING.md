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

**1. Pre-release checks** (fix any issues before versioning)
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

**3. Run release**
```bash
# For patch release (0.5.2 -> 0.5.3)
cargo release patch --execute

# For minor release (0.5.3 -> 0.6.0)
cargo release minor --execute

# For major release (0.6.0 -> 1.0.0)
cargo release major --execute
```

This will:
- Update version in `Cargo.toml`
- Create a version bump commit
- Create a git tag (e.g., `v0.5.3`)
- Push commits and tags to GitHub
- Publish to crates.io (if configured)

**4. Merge to master** (optional, for syncing main branch)
```bash
git checkout master
git merge v0.5.x
git push origin master
```

### Version Numbering
We follow [Semantic Versioning](https://semver.org/):
- **Patch** (0.5.2 -> 0.5.3): Bug fixes, non-breaking changes
- **Minor** (0.5.3 -> 0.6.0): New features, non-breaking changes
- **Major** (0.6.0 -> 1.0.0): Breaking changes

### Manual Release (if needed)
If `cargo-release` isn't available:

```bash
# 1. Update version in Cargo.toml manually
# 2. Commit version bump
git add Cargo.toml
git commit -m "chore: bump version to 0.5.3"

# 3. Create and push tag
git tag v0.5.3
git push origin v0.5.x --tags

# 4. Publish to crates.io
cargo publish
```

## Branch Strategy

- **master**: Stable releases
- **v0.5.x**: Current development branch for 0.5.x releases
- **feature branches**: For new features, merged into v0.5.x

## Code Style

- Follow Rust conventions
- Use `rustfmt` for formatting
- Address `clippy` warnings
- Add documentation comments for public APIs
- Keep functions focused and testable

## Questions?

Feel free to open an issue for any questions about contributing!
