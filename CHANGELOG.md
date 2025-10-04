# Changelog

All notable changes to dots-cli will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.3] - 2025-01-10

### Added
- Debug flag support for verbose logging (#57)
- Permission error testing utilities in TestManager
- `make_readonly()` method for testing file permission scenarios
- Display implementation for Link with pretty path formatting

### Changed
- **BREAKING INTERNAL**: Refactored reconciliation logic - cleanup now runs after installation instead of before
- Moved footprint cleanup to execute after symlink creation for better error recovery
- Improved error messages to show specific footprint file path on permission errors
- Error output now displays context and root cause on separate lines for clarity
- Validation now only prints newlines between dots, not before the first dot

### Fixed
- Fix #9: Now properly tracks and removes extra directories created during installation
- Fix #26: Improved directory tracking and cleanup logic
- Fixed footprint directory tracking to handle nested directory structures
- Fixed issue where directories with user-created files were incorrectly removed
- Added warnings when directories can't be removed due to user files

### Internal
- Implemented Actions system for filesystem operations, enabling better error collection and dry-run support
- Added comprehensive snapshot tests for permission errors
- Refactored Plan module to use three-phase approach: clean → validate → execute
- Improved test infrastructure with reusable permission testing utilities
- Ignore dotfiles in dots directory to prevent unintended tracking

## [0.5.2] - 2024-XX-XX
<!-- Previous release, details to be filled in -->

## [0.5.1] - 2024-XX-XX
<!-- Previous release, details to be filled in -->

## [0.5.0] - 2024-XX-XX
<!-- Previous release, details to be filled in -->

[Unreleased]: https://github.com/webdesserts/dots-cli/compare/v0.5.3...HEAD
[0.5.3]: https://github.com/webdesserts/dots-cli/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/webdesserts/dots-cli/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/webdesserts/dots-cli/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/webdesserts/dots-cli/releases/tag/v0.5.0
