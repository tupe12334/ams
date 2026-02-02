//! Session types and data structures.

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
            Self::Active => write!(f, "Active"),
            Self::Idle => write!(f, "Idle"),
            Self::Dead => write!(f, "Dead"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_status_display_active() {
        assert_eq!(SessionStatus::Active.to_string(), "Active");
    }

    #[test]
    fn test_session_status_display_idle() {
        assert_eq!(SessionStatus::Idle.to_string(), "Idle");
    }

    #[test]
    fn test_session_status_display_dead() {
        assert_eq!(SessionStatus::Dead.to_string(), "Dead");
    }

    #[test]
    fn test_session_status_equality() {
        assert_eq!(SessionStatus::Active, SessionStatus::Active);
        assert_ne!(SessionStatus::Active, SessionStatus::Idle);
        assert_ne!(SessionStatus::Idle, SessionStatus::Dead);
    }

    #[test]
    fn test_session_status_clone() {
        let status = SessionStatus::Active;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_session_status_debug() {
        let debug_str = format!("{:?}", SessionStatus::Active);
        assert!(debug_str.contains("Active"));
    }
}
