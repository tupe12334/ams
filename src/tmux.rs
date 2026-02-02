use crate::session::{Session, SessionStatus};
use chrono::{TimeZone, Utc};
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TmuxError {
    #[error("Failed to execute tmux command: {0}")]
    CommandFailed(#[from] std::io::Error),

    #[error("Failed to parse tmux output: {0}")]
    ParseError(String),

    #[error("Tmux server not running")]
    ServerNotRunning,

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Session already exists: {0}")]
    SessionExists(String),
}

/// Lists all tmux sessions with their metadata
pub fn list_sessions() -> Result<Vec<Session>, TmuxError> {
    let output = Command::new("tmux")
        .args([
            "list-sessions",
            "-F",
            "#{session_name}\t#{session_attached}\t#{session_activity}\t#{session_created}\t#{pane_current_path}\t#{session_windows}",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("no server running") || stderr.contains("no sessions") {
            return Ok(Vec::new());
        }
        return Err(TmuxError::ParseError(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_sessions(&stdout)
}

fn parse_sessions(output: &str) -> Result<Vec<Session>, TmuxError> {
    let mut sessions = Vec::new();

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 6 {
            return Err(TmuxError::ParseError(format!(
                "Expected 6 fields, got {}: {}",
                parts.len(),
                line
            )));
        }

        let name = parts[0].to_string();

        let attached_count: u32 = parts[1]
            .parse()
            .map_err(|_| TmuxError::ParseError(format!("Invalid attached count: {}", parts[1])))?;

        let status = if attached_count > 0 {
            SessionStatus::Active
        } else {
            SessionStatus::Idle
        };

        let activity_epoch: i64 = parts[2].parse().map_err(|_| {
            TmuxError::ParseError(format!("Invalid activity timestamp: {}", parts[2]))
        })?;

        let created_epoch: i64 = parts[3].parse().map_err(|_| {
            TmuxError::ParseError(format!("Invalid created timestamp: {}", parts[3]))
        })?;

        let last_activity = Utc
            .timestamp_opt(activity_epoch, 0)
            .single()
            .ok_or_else(|| {
                TmuxError::ParseError(format!("Invalid activity epoch: {}", activity_epoch))
            })?;

        let created_at = Utc
            .timestamp_opt(created_epoch, 0)
            .single()
            .ok_or_else(|| {
                TmuxError::ParseError(format!("Invalid created epoch: {}", created_epoch))
            })?;

        let working_directory = PathBuf::from(parts[4]);

        let window_count: u32 = parts[5]
            .parse()
            .map_err(|_| TmuxError::ParseError(format!("Invalid window count: {}", parts[5])))?;

        sessions.push(Session {
            name,
            status,
            working_directory,
            last_activity,
            created_at,
            window_count,
        });
    }

    Ok(sessions)
}

/// Attaches to an existing tmux session
pub fn attach_session(name: &str) -> Result<(), TmuxError> {
    let status = Command::new("tmux")
        .args(["attach-session", "-t", name])
        .status()?;

    if !status.success() {
        return Err(TmuxError::SessionNotFound(name.to_string()));
    }

    Ok(())
}

/// Creates a new tmux session
pub fn create_session(name: &str, directory: Option<&str>) -> Result<(), TmuxError> {
    let mut cmd = Command::new("tmux");
    cmd.args(["new-session", "-d", "-s", name]);

    if let Some(dir) = directory {
        cmd.args(["-c", dir]);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("duplicate session") {
            return Err(TmuxError::SessionExists(name.to_string()));
        }
        return Err(TmuxError::ParseError(stderr.to_string()));
    }

    Ok(())
}

/// Kills a tmux session
pub fn kill_session(name: &str) -> Result<(), TmuxError> {
    let output = Command::new("tmux")
        .args(["kill-session", "-t", name])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("session not found") || stderr.contains("can't find session") {
            return Err(TmuxError::SessionNotFound(name.to_string()));
        }
        return Err(TmuxError::ParseError(stderr.to_string()));
    }

    Ok(())
}

/// Gets information about a specific session
pub fn get_session(name: &str) -> Result<Session, TmuxError> {
    let filter = format!("#{{==:#{{session_name}},{}}}", name);
    let output = Command::new("tmux")
        .args([
            "list-sessions",
            "-F",
            "#{session_name}\t#{session_attached}\t#{session_activity}\t#{session_created}\t#{pane_current_path}\t#{session_windows}",
            "-f",
            &filter,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("no server running") || stderr.contains("no sessions") {
            return Err(TmuxError::SessionNotFound(name.to_string()));
        }
        return Err(TmuxError::ParseError(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let sessions = parse_sessions(&stdout)?;

    sessions
        .into_iter()
        .next()
        .ok_or_else(|| TmuxError::SessionNotFound(name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sessions_empty() {
        let result = parse_sessions("").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_sessions_single() {
        let output = "test-session\t0\t1704067200\t1704067200\t/home/user/project\t1";
        let sessions = parse_sessions(output).unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].name, "test-session");
        assert_eq!(sessions[0].status, SessionStatus::Idle);
        assert_eq!(sessions[0].window_count, 1);
    }

    #[test]
    fn test_parse_sessions_active() {
        let output = "active-session\t1\t1704067200\t1704067200\t/home/user/project\t2";
        let sessions = parse_sessions(output).unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].status, SessionStatus::Active);
    }
}
