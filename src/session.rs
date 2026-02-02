use chrono::{DateTime, Utc};
use std::path::PathBuf;

/// Represents the current status of a tmux session
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionStatus {
    /// Has at least one attached client
    Active,
    /// Running but no clients attached
    Idle,
    /// Session no longer exists
    Dead,
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionStatus::Active => write!(f, "Active"),
            SessionStatus::Idle => write!(f, "Idle"),
            SessionStatus::Dead => write!(f, "Dead"),
        }
    }
}

/// Represents a tmux session with its metadata
#[derive(Debug, Clone)]
pub struct Session {
    /// Session name
    pub name: String,
    /// Current status of the session
    pub status: SessionStatus,
    /// Working directory of the session
    pub working_directory: PathBuf,
    /// Timestamp of last activity
    pub last_activity: DateTime<Utc>,
    /// Timestamp when session was created
    pub created_at: DateTime<Utc>,
    /// Number of windows in the session
    pub window_count: u32,
}
