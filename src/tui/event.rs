use super::{App, ResultMessage, Screen};
use crate::commands::vote::process_fetch_vote_account;
use crossterm::event::KeyCode;
use solana_pubkey::Pubkey;
use std::str::FromStr;

pub async fn handle_key(app: &mut App, key: KeyCode) {
    match &app.current_screen {
        Screen::ShowVoteAccountForm => handle_form_input(app, key).await,
        Screen::Result => handle_result_input(app, key),
        Screen::Loading => {} // Ignore input while loading
        _ => handle_menu_input(app, key).await,
    }
}

async fn handle_menu_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') => app.running = false,

        KeyCode::Up => {
            if app.selected_index > 0 {
                app.selected_index -= 1;
            }
        }

        KeyCode::Down => {
            let max = app.menu_items().len().saturating_sub(1);
            if app.selected_index < max {
                app.selected_index += 1;
            }
        }

        KeyCode::Enter => {
            handle_selection(app).await;
        }

        KeyCode::Esc => {
            app.current_screen = Screen::MainMenu;
            app.selected_index = 0;
        }

        _ => {}
    }
}

async fn handle_form_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) => {
            app.input_buffer.insert(app.input_cursor, c);
            app.input_cursor += 1;
        }

        KeyCode::Backspace => {
            if app.input_cursor > 0 {
                app.input_cursor -= 1;
                app.input_buffer.remove(app.input_cursor);
            }
        }

        KeyCode::Left => {
            if app.input_cursor > 0 {
                app.input_cursor -= 1;
            }
        }

        KeyCode::Right => {
            if app.input_cursor < app.input_buffer.len() {
                app.input_cursor += 1;
            }
        }

        KeyCode::Enter => {
            handle_form_submit(app).await;
        }

        KeyCode::Esc => {
            app.clear_input();
            app.current_screen = Screen::VoteMenu;
            app.selected_index = 0;
        }

        _ => {}
    }
}

fn handle_result_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Esc => {
            app.result_message = None;
            app.current_screen = Screen::VoteMenu;
            app.selected_index = 0;
        }
        _ => {}
    }
}

async fn handle_selection(app: &mut App) {
    let items = app.menu_items();
    let selected = items.get(app.selected_index).copied().unwrap_or("");

    match app.current_screen {
        Screen::MainMenu => match selected {
            "Account" => {
                app.current_screen = Screen::AccountMenu;
                app.selected_index = 0;
            }
            "Cluster" => {
                app.current_screen = Screen::ClusterMenu;
                app.selected_index = 0;
            }
            "Stake" => {
                app.current_screen = Screen::StakeMenu;
                app.selected_index = 0;
            }
            "Vote" => {
                app.current_screen = Screen::VoteMenu;
                app.selected_index = 0;
            }
            "Config" => {
                app.current_screen = Screen::ConfigMenu;
                app.selected_index = 0;
            }
            "Exit" => app.running = false,
            _ => {}
        },

        Screen::VoteMenu => match selected {
            "Show Vote Account" => {
                app.clear_input();
                app.current_screen = Screen::ShowVoteAccountForm;
            }
            "← Back" => {
                app.current_screen = Screen::MainMenu;
                app.selected_index = 0;
            }
            _ => {}
        },

        _ => {
            if selected == "← Back" {
                app.current_screen = Screen::MainMenu;
                app.selected_index = 0;
            }
        }
    }
}

async fn handle_form_submit(app: &mut App) {
    match app.current_screen {
        Screen::ShowVoteAccountForm => {
            execute_show_vote_account(app).await;
        }
        _ => {}
    }
}

async fn execute_show_vote_account(app: &mut App) {
    let pubkey_str = app.input_buffer.trim();

    // Parse pubkey
    let pubkey = match Pubkey::from_str(pubkey_str) {
        Ok(pk) => pk,
        Err(_) => {
            app.result_message = Some(ResultMessage {
                success: false,
                title: "Error".to_string(),
                body: format!("Invalid pubkey: {}", pubkey_str),
            });
            app.current_screen = Screen::Result;
            return;
        }
    };

    // Set loading state
    app.current_screen = Screen::Loading;

    // Fetch vote account
    match process_fetch_vote_account(&app.ctx, &pubkey).await {
        Ok(info) => {
            app.result_message = Some(ResultMessage {
                success: true,
                title: "Vote Account Info".to_string(),
                body: info,
            });
        }
        Err(e) => {
            app.result_message = Some(ResultMessage {
                success: false,
                title: "Error".to_string(),
                body: e.to_string(),
            });
        }
    }

    app.clear_input();
    app.current_screen = Screen::Result;
}
