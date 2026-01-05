mod event;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{Event, KeyEventKind, poll, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::Instant;
use std::{io, time::Duration};

use crate::context::ScillaContext;

pub struct App {
    pub running: bool,
    pub selected_index: usize,
    pub current_screen: Screen,
    pub ctx: ScillaContext,
    pub input_buffer: String,
    pub input_cursor: usize,
    pub result_message: Option<ResultMessage>,
    pub wallet_balance: Option<f64>,
    pub last_refresh: Instant,
}

pub struct ResultMessage {
    pub success: bool,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    AccountMenu,
    ClusterMenu,
    StakeMenu,
    VoteMenu,
    ConfigMenu,
    ShowVoteAccountForm,
    Result,
    Loading,
}

impl App {
    pub fn new(ctx: ScillaContext) -> Self {
        Self {
            running: true,
            selected_index: 0,
            current_screen: Screen::MainMenu,
            ctx,
            input_buffer: String::new(),
            input_cursor: 0,
            result_message: None,
            wallet_balance: None,
            last_refresh: Instant::now(),
        }
    }

    pub async fn refresh_status(&mut self) {
        if self.last_refresh.elapsed().as_secs() < 5 {
            return;
        }

        if let Ok(balance) = self.ctx.rpc().get_balance(self.ctx.pubkey()).await {
            self.wallet_balance = Some(balance as f64 / 1_000_000_000.0);
        }

        self.last_refresh = Instant::now();
    }

    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
        self.input_cursor = 0;
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
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(ctx);

    if let Ok(balance) = app.ctx.rpc().get_balance(app.ctx.pubkey()).await {
        app.wallet_balance = Some(balance as f64 / 1_000_000_000.0);
    }

    while app.running {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    event::handle_key(&mut app, key.code).await;
                }
            }
        }

        app.refresh_status().await;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
