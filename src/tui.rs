use crate::{list_sessions, Session, SessionStatus};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};
use std::io::{self, stdout};

pub struct App {
    sessions: Vec<Session>,
    table_state: TableState,
    should_quit: bool,
    selected_session: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            table_state: TableState::default(),
            should_quit: false,
            selected_session: None,
        }
    }

    pub fn refresh_sessions(&mut self) {
        self.sessions = list_sessions().unwrap_or_default();
        if !self.sessions.is_empty() && self.table_state.selected().is_none() {
            self.table_state.select(Some(0));
        }
    }

    fn next(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.sessions.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.sessions.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn select_current(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if let Some(session) = self.sessions.get(i) {
                self.selected_session = Some(session.name.clone());
                self.should_quit = true;
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the terminal for TUI mode
pub fn init_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::new(backend)
}

/// Restore the terminal to normal mode
pub fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

/// Set up panic hook to restore terminal on panic
pub fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));
}

/// Run the TUI application
pub fn run() -> io::Result<Option<String>> {
    install_panic_hook();
    let mut terminal = init_terminal()?;
    let mut app = App::new();
    app.refresh_sessions();

    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        KeyCode::Enter => app.select_current(),
                        KeyCode::Char('r') => app.refresh_sessions(),
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    restore_terminal()?;
    Ok(app.selected_session)
}

fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let header = Row::new(vec!["Name", "Status", "Windows", "Working Directory"])
        .style(Style::default().bold())
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .sessions
        .iter()
        .map(|session| {
            let status_style = match session.status {
                SessionStatus::Active => Style::default().fg(Color::Green),
                SessionStatus::Idle => Style::default().fg(Color::Yellow),
                SessionStatus::Dead => Style::default().fg(Color::Red),
            };

            let working_dir = session
                .working_directory
                .to_string_lossy()
                .chars()
                .rev()
                .take(40)
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>();

            Row::new(vec![
                Cell::from(session.name.clone()),
                Cell::from(session.status.to_string()).style(status_style),
                Cell::from(session.window_count.to_string()),
                Cell::from(working_dir),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(20),
        Constraint::Length(10),
        Constraint::Length(8),
        Constraint::Min(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" AMS - Agents Manager Service ")
                .title_bottom(" q:quit  j/k:nav  Enter:attach  r:refresh "),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(table, area, &mut app.table_state);
}
