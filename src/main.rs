use ams::{list_sessions, SessionStatus};
use chrono::Utc;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ams", about = "Agents Manager Service - Manage AI coding agent tmux sessions")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all tmux sessions
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::List) | None => {
            if let Err(e) = run_list() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_list() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = list_sessions()?;

    if sessions.is_empty() {
        println!("No tmux sessions found.");
        return Ok(());
    }

    // Print header
    println!(
        "{:<20} {:<8} {:<35} {}",
        "NAME", "STATUS", "WORKING DIR", "LAST ACTIVITY"
    );

    // Print sessions
    for session in sessions {
        let working_dir = session
            .working_directory
            .to_string_lossy()
            .chars()
            .take(35)
            .collect::<String>();

        let last_activity = format_relative_time(session.last_activity);

        let status_str = match session.status {
            SessionStatus::Active => "Active",
            SessionStatus::Idle => "Idle",
            SessionStatus::Dead => "Dead",
        };

        println!(
            "{:<20} {:<8} {:<35} {}",
            truncate(&session.name, 20),
            status_str,
            working_dir,
            last_activity
        );
    }

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

fn format_relative_time(dt: chrono::DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(dt);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        let mins = duration.num_minutes();
        if mins == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{} minutes ago", mins)
        }
    } else if duration.num_hours() < 24 {
        let hours = duration.num_hours();
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else {
        let days = duration.num_days();
        if days == 1 {
            "1 day ago".to_string()
        } else {
            format!("{} days ago", days)
        }
    }
}
