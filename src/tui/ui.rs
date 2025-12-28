use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::App;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Status bar
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Keybindings
        ])
        .split(frame.area());

    render_status_bar(frame, app, chunks[0]);
    render_menu(frame, app, chunks[1]);
    render_keybindings(frame, app, chunks[2]);
}

fn render_status_bar(frame: &mut Frame, _app: &App, area: Rect) {
    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            "⚡ SCILLA",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("          "),
        Span::styled("Epoch: ", Style::default().fg(Color::Gray)),
        Span::styled("---", Style::default().fg(Color::White)),
        Span::raw("  │  "),
        Span::styled("Slot: ", Style::default().fg(Color::Gray)),
        Span::styled("---", Style::default().fg(Color::White)),
        Span::raw("  │  "),
        Span::styled("Balance: ", Style::default().fg(Color::Gray)),
        Span::styled("--- SOL", Style::default().fg(Color::Green)),
        Span::raw("  │  "),
        Span::styled("devnet", Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(status, area);
}

fn render_menu(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .menu_items()
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_index {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if i == app.selected_index { "> " } else { "  " };
            ListItem::new(format!("{}{}", prefix, item)).style(style)
        })
        .collect();

    let title = match app.current_screen {
        super::Screen::MainMenu => "Command Groups",
        super::Screen::AccountMenu => "Account Commands",
        super::Screen::ClusterMenu => "Cluster Commands",
        super::Screen::StakeMenu => "Stake Commands",
        super::Screen::VoteMenu => "Vote Commands",
        super::Screen::ConfigMenu => "Config Commands",
    };

    let list = List::new(items).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray)),
    );

    // Center the menu
    let menu_area = centered_rect(30, 50, area);
    frame.render_widget(list, menu_area);
}

fn render_keybindings(frame: &mut Frame, _app: &App, area: Rect) {
    let keybindings = Paragraph::new(Line::from(vec![
        Span::styled("[↑↓]", Style::default().fg(Color::Yellow)),
        Span::raw(" Navigate  "),
        Span::styled("[Enter]", Style::default().fg(Color::Yellow)),
        Span::raw(" Select  "),
        Span::styled("[Esc]", Style::default().fg(Color::Yellow)),
        Span::raw(" Back  "),
        Span::styled("[q]", Style::default().fg(Color::Yellow)),
        Span::raw(" Quit"),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::TOP));

    frame.render_widget(keybindings, area);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
