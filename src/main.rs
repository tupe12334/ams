use ams::{list_sessions, SessionStatus};
use chrono::Utc;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ams",
    version,
    about = "Agents Manager Service - Manage AI coding agent tmux sessions"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch interactive TUI (default)
    Tui,
    /// List all tmux sessions
    List,
    /// Attach to a tmux session
    Attach {
        /// Name of the session to attach to
        name: String,
    },
    /// Create a new tmux session
    New {
        /// Name for the new session
        name: String,
        /// Working directory for the session
        #[arg(short, long)]
        directory: Option<String>,
    },
    /// Kill a tmux session
    Kill {
        /// Name of the session to kill
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Tui) | None => run_tui(),
        Some(Commands::List) => run_list(),
        Some(Commands::Attach { name }) => run_attach(&name),
        Some(Commands::New { name, directory }) => run_new(&name, directory.as_deref()),
        Some(Commands::Kill { name }) => run_kill(&name),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(session_name) = ams::tui::run()? {
        ams::attach_session(&session_name)?;
    }
    Ok(())
}

fn run_attach(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    ams::attach_session(name)?;
    Ok(())
}

fn run_new(name: &str, directory: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    ams::create_session(name, directory)?;
    println!("Created session: {}", name);
    Ok(())
}

fn run_kill(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    ams::kill_session(name)?;
    println!("Killed session: {}", name);
    Ok(())
}

fn run_list() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = list_sessions()?;

    if sessions.is_empty() {
        println!("No tmux sessions found.");
        return Ok(());
    }

    // Print header
    println!(
        "{:<20} {:<8} {:<35} LAST ACTIVITY",
        "NAME", "STATUS", "WORKING DIR"
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
