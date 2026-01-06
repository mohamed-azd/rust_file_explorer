use std::ptr::null;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::Style;
use ratatui::widgets::{Block, BorderType, List, Padding, Paragraph, Wrap};
use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(64),
            Constraint::Percentage(4),
            Constraint::Percentage(32)
        ])
        .split(frame.area());

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(90),
        ])
        .split(layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100),
        ])
        .split(layout[2]);

    let bloc_padding = Padding::new(2, 2, 1, 1);


    let path = Paragraph::new(app.current_dir.display().to_string())
        .block(get_bloc("Path", Padding::ZERO))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Left);

    let explorer = List::new(app.items.iter().map(|item| item.name.as_str()).collect::<Vec<_>>())
        .block(get_bloc("Explorer", bloc_padding))
        .style(Style::new().white().on_black())
        .highlight_style(Style::new().bg(ratatui::style::Color::Green));

    let name_selected_item;
    let type_selected_item;
    if let Some(item) = app.items.get(app.selected_item) {
        name_selected_item = item.name.as_str();
        type_selected_item = item.item_type.as_str();
    } else {
        name_selected_item = "Selection";
        type_selected_item = "";
    }

    let selection = Paragraph::new(format!("name: {}\ntype: {}", name_selected_item, type_selected_item))
        .block(get_bloc("Selection", bloc_padding))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Left);

    frame.render_widget(path, left_layout[0]);
    frame.render_stateful_widget(explorer, left_layout[1], &mut app.state);
    frame.render_widget(selection, right_layout[0]);
}


fn get_bloc(title: &str, padding: Padding) -> Block {
    Block::bordered().border_type(BorderType::Rounded).padding(padding).title(title)
}