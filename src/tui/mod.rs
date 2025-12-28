mod event;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{Event, KeyEventKind, poll, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, time::Duration};

use crate::context::ScillaContext;

pub struct App {
    pub running: bool,
    pub selected_index: usize,
    pub current_screen: Screen,
    pub ctx: ScillaContext,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    AccountMenu,
    ClusterMenu,
    StakeMenu,
    VoteMenu,
    ConfigMenu,
}

impl App {
    pub fn new(ctx: ScillaContext) -> Self {
        Self {
            running: true,
            selected_index: 0,
            current_screen: Screen::MainMenu,
            ctx,
        }
    }

    pub fn menu_items(&self) -> Vec<&str> {
        match self.current_screen {
            Screen::MainMenu => vec!["Account", "Cluster", "Stake", "Vote", "Config", "Exit"],
            Screen::AccountMenu => vec![
                "Fetch Account",
                "Balance",
                "Transfer",
                "Airdrop",
                "Check Transaction",
                "Largest Accounts",
                "Nonce Account",
                "← Back",
            ],
            Screen::VoteMenu => vec![
                "Create Vote Account",
                "Authorize Voter",
                "Withdraw From Vote",
                "Show Vote Account",
                "Close Vote Account",
                "← Back",
            ],
            _ => vec!["← Back"],
        }
    }
}

pub async fn run(ctx: ScillaContext) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(ctx);

    // Main loop
    while app.running {
        // Render
        terminal.draw(|frame| ui::render(frame, &app))?;

        // Handle input (with timeout for future background tasks)
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    event::handle_key(&mut app, key.code).await;
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

