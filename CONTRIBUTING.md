# Contributing

## Prerequisites

This project uses [cargo-husky](https://github.com/rhysd/cargo-husky) for git hooks and [typos](https://github.com/crate-ci/typos) for spell checking.

Install typos before contributing:

```bash
cargo install typos-cli
```

## Pre-commit Hooks

After cloning and running `cargo build`, cargo-husky will automatically set up a pre-commit hook that runs:

1. `typos` - Spell checker for source code
2. `cargo fmt -- --check` - Code formatting check
3. `cargo clippy -- -D warnings` - Linting

If any of these fail, the commit will be rejected. Fix the issues and try again.

## Running Checks Manually

```bash
# Run typos spell checker
typos

# Fix typos automatically (use with caution)
typos -w

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings
```

## Test Coverage

This project enforces 100% test coverage. All new code must include tests.

Install cargo-llvm-cov for coverage reports:

```bash
cargo install cargo-llvm-cov
```

Run coverage locally:

```bash
# Run tests with coverage check (fails if below 100%)
cargo llvm-cov --fail-under-lines 100

# Generate HTML report
cargo llvm-cov --html --open

# Generate lcov report
cargo llvm-cov --lcov --output-path lcov.info
```

Coverage reports are automatically generated in CI and uploaded as artifacts.
