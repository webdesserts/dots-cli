# dots-cli Development Guidelines

This document provides guidance for Claude Code when working on the dots-cli project.

## Testing Approach

### Philosophy
- **Integration over Unit Tests**: Focus on CLI behavior, not internal implementation details
- **Snapshot Testing**: Use comprehensive snapshot tests in `tests/output/` to capture complete CLI output
- **Resilience**: Tests should survive refactoring by testing user-visible behavior

### Structure
- **Test Files**: Organized by subcommand in `tests/` (e.g., `subcommand_install.rs`)
- **Fixtures**: Reusable test scenarios in `fixtures/` directory, managed via `packages/test_utils/src/fixtures.rs`
- **Snapshots**: Expected output stored in `tests/output/` as `.err` (stderr) or `.out` (stdout) files

### Naming Conventions
- **Tests**: `it_should_[action]_[condition]` (e.g., `it_should_display_and_install_the_given_plan`)
- **Fixtures**: `Example[Purpose][Variant]` (e.g., `ExampleDotWithSelfLink`)
- **Snapshots**: `[command]_[outcome]_[condition].[err|out]` (e.g., `install_success_when_self_linking.err`)

### Key Principles
- Test complete user scenarios rather than specific bugs or formatting details
- Verify both CLI output (via snapshots) and actual behavior (e.g., symlinks created)
- Use existing test infrastructure (`TestManager`, fixtures) rather than creating new patterns
- For bug fixes, create comprehensive tests covering the entire user workflow

## Implementation Guidelines
- Follow existing code conventions and patterns
- Use existing libraries and utilities rather than introducing new dependencies
- Maintain consistency with the project's established architecture
- Run linting and type checking when available