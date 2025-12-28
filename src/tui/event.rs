use crossterm::event::KeyCode;

use super::{App, Screen};

pub async fn handle_key(app: &mut App, key: KeyCode) {
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
            "← Back" => {
                app.current_screen = Screen::MainMenu;
                app.selected_index = 0;
            }
            // TODO: Handle other selections (open forms)
            _ => {}
        },

        // Handle other menus similarly
        _ => {
            if selected == "← Back" {
                app.current_screen = Screen::MainMenu;
                app.selected_index = 0;
            }
        }
    }
}
