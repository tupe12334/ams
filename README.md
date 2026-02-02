# AMS - Agents Manager Service

[![Coverage](https://codecov.io/gh/tupe12334/ams/branch/main/graph/badge.svg)](https://codecov.io/gh/tupe12334/ams)

A Rust CLI tool with TUI for managing multiple AI coding agents through tmux sessions.

## Overview

AMS helps developers who work with multiple AI coding assistants simultaneously. Whether you're running Claude Code, Codex, Aider, or other CLI-based AI agents, AMS provides a unified interface to:

- View all running agent sessions in a terminal UI
- Attach to any agent session with a keystroke
- Spawn new agent sessions
- Open new terminals attached to existing sessions
- Monitor agent activity across sessions

## Features

- **TUI Dashboard** - See all your running AI agents at a glance
- **tmux Integration** - Leverages tmux for robust session management
- **Quick Attach** - Jump into any agent session instantly
- **Multi-Terminal Support** - Open multiple terminals attached to the same session
- **Agent Agnostic** - Works with any CLI-based AI coding assistant

## Supported Agents

- [Claude Code](https://github.com/anthropics/claude-code)
- [Codex](https://github.com/openai/codex)
- [Aider](https://github.com/paul-gauthier/aider)
- Any other CLI-based coding agent

## Requirements

- Rust 1.70+
- tmux 3.0+

## Installation

```bash
cargo install ams
```

Or build from source:

```bash
git clone https://github.com/tupe12334/ams.git
cd ams
cargo build --release
```

## Usage

```bash
# Launch the TUI dashboard
ams

# List all agent sessions
ams list

# Attach to a specific session
ams attach <session-name>

# Spawn a new agent session
ams new claude-code
ams new codex
ams new aider

# Open a new terminal window attached to an existing session
ams open <session-name>
```

## TUI Keybindings

| Key | Action |
|-----|--------|
| `j/k` or `↓/↑` | Navigate sessions |
| `Enter` | Attach to selected session |
| `n` | New agent session |
| `o` | Open new terminal for session |
| `d` | Detach from current view |
| `q` | Quit |
| `?` | Help |

## How It Works

AMS uses tmux as the underlying session manager. Each AI agent runs in its own tmux session, which allows:

1. **Persistence** - Sessions survive terminal closures
2. **Multiplexing** - Multiple terminals can view the same session
3. **Detachment** - Work continues in the background when detached

The TUI polls tmux for session information and provides a clean interface for managing your AI coding workflow.

## Configuration

Configuration file location: `~/.config/ams/config.toml`

```toml
# Default agent to spawn
default_agent = "claude-code"

# Agent command definitions
[agents.claude-code]
command = "claude"
args = []

[agents.codex]
command = "codex"
args = []

[agents.aider]
command = "aider"
args = []

# TUI settings
[tui]
refresh_rate_ms = 1000
```

## Contributing

### Prerequisites

This project uses [cargo-husky](https://github.com/rhysd/cargo-husky) for git hooks and [typos](https://github.com/crate-ci/typos) for spell checking.

Install typos before contributing:

```bash
cargo install typos-cli
```

### Pre-commit Hooks

After cloning and running `cargo build`, cargo-husky will automatically set up a pre-commit hook that runs:

1. `typos` - Spell checker for source code
2. `cargo fmt -- --check` - Code formatting check
3. `cargo clippy -- -D warnings` - Linting

If any of these fail, the commit will be rejected. Fix the issues and try again.

### Running Checks Manually

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

### Test Coverage

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

## License

MIT
