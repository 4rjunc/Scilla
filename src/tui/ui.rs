use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::{App, Screen};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_status_bar(frame, app, chunks[0]);
    render_main_content(frame, app, chunks[1]);
    render_keybindings(frame, app, chunks[2]);
}

fn render_main_content(frame: &mut Frame, app: &App, area: Rect) {
    match &app.current_screen {
        Screen::ShowVoteAccountForm => render_form(frame, app, area),
        Screen::Result => render_result(frame, app, area),
        Screen::Loading => render_loading(frame, area),
        _ => render_menu(frame, app, area),
    }
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let balance_str = match app.wallet_balance {
        Some(bal) => format!("{:.4} SOL", bal),
        None => "--- SOL".to_string(),
    };

    // Truncate wallet address for display
    let pubkey_str = app.ctx.pubkey().to_string();
    let short_pubkey = format!(
        "{}...{}",
        &pubkey_str[..4],
        &pubkey_str[pubkey_str.len() - 4..]
    );

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            "⚡ SCILLA",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("    "),
        Span::styled(&short_pubkey, Style::default().fg(Color::Gray)),
        Span::raw("  │  "),
        Span::styled("Balance: ", Style::default().fg(Color::Gray)),
        Span::styled(balance_str, Style::default().fg(Color::Green)),
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
        Screen::MainMenu => "Command Groups",
        Screen::AccountMenu => "Account Commands",
        Screen::ClusterMenu => "Cluster Commands",
        Screen::StakeMenu => "Stake Commands",
        Screen::VoteMenu => "Vote Commands",
        Screen::ConfigMenu => "Config Commands",
        _ => "",
    };

    let list = List::new(items).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray)),
    );

    let menu_area = centered_rect(30, 50, area);
    frame.render_widget(list, menu_area);
}

fn render_form(frame: &mut Frame, app: &App, area: Rect) {
    let form_area = centered_rect(60, 30, area);

    // Clear the area behind the form
    frame.render_widget(Clear, form_area);

    let title = match app.current_screen {
        Screen::ShowVoteAccountForm => "Show Vote Account",
        _ => "Form",
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block, form_area);

    // Inner area for form content
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1), // Label
            Constraint::Length(3), // Input
            Constraint::Min(0),    // Spacer
        ])
        .split(form_area);

    // Label
    let label = Paragraph::new("Vote Account Address:").style(Style::default().fg(Color::Gray));
    frame.render_widget(label, inner[0]);

    // Input field
    let input_text = format!("{}_", app.input_buffer);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );
    frame.render_widget(input, inner[1]);
}

fn render_result(frame: &mut Frame, app: &App, area: Rect) {
    let result_area = centered_rect(70, 60, area);

    frame.render_widget(Clear, result_area);

    let result = app.result_message.as_ref().unwrap();

    let border_color = if result.success {
        Color::Green
    } else {
        Color::Red
    };

    let title_style = if result.success {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    };

    let icon = if result.success { "✅ " } else { "❌ " };

    let block = Block::default()
        .title(Span::styled(
            format!("{}{}", icon, result.title),
            title_style,
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let content = Paragraph::new(result.body.clone())
        .style(Style::default().fg(Color::White))
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(content, result_area);
}

fn render_loading(frame: &mut Frame, area: Rect) {
    let loading_area = centered_rect(30, 20, area);

    frame.render_widget(Clear, loading_area);

    let loading = Paragraph::new("Loading...")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    frame.render_widget(loading, loading_area);
}

fn render_keybindings(frame: &mut Frame, app: &App, area: Rect) {
    let keys = match app.current_screen {
        Screen::ShowVoteAccountForm => vec![("[Enter]", "Submit"), ("[Esc]", "Cancel")],
        Screen::Result => vec![("[Enter]", "Continue")],
        Screen::Loading => vec![],
        _ => vec![
            ("[↑↓]", "Navigate"),
            ("[Enter]", "Select"),
            ("[Esc]", "Back"),
            ("[q]", "Quit"),
        ],
    };

    let spans: Vec<Span> = keys
        .iter()
        .flat_map(|(key, action)| {
            vec![
                Span::styled(*key, Style::default().fg(Color::Yellow)),
                Span::raw(format!(" {}  ", action)),
            ]
        })
        .collect();

    let keybindings = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(keybindings, area);
}

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
