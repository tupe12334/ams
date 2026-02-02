//! AMS - Agents Manager Service
//!
//! A CLI tool for managing AI coding agent tmux sessions.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
// Allowed lints with justification
#![allow(clippy::missing_errors_doc)] // Would require extensive doc changes
#![allow(clippy::module_name_repetitions)] // Common pattern in Rust
#![allow(clippy::multiple_crate_versions)] // Cannot control transitive dependencies

pub mod session;
pub mod tmux;
pub mod tui;

pub use session::{Session, SessionStatus};
pub use tmux::{
    attach_session, create_session, get_session, kill_session, list_sessions, TmuxError,
};
