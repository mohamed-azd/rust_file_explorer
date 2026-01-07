use crate::app::App;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::Style;
use ratatui::widgets::{Block, BorderType, List, Padding, Paragraph};
use ratatui::Frame;
use std::rc::Rc;

const BLOC_PADDING: Padding = Padding::new(2, 2, 1, 1);

pub fn render(frame: &mut Frame, app: &mut App) {

    let (left_layout, right_layout) = build_layout(frame);

    render_path_bar(frame, app, left_layout[0]);
    render_explorer_area(frame, app, left_layout[1]);
    render_selection_infos_area(frame, app, right_layout[0]);
    render_selection_preview_area(frame, app, right_layout[1]);
}

fn build_layout(frame: &Frame) -> (Rc<[Rect]>,  Rc<[Rect]>)  {
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
            Constraint::Percentage(10),
            Constraint::Percentage(90),
        ])
        .split(layout[2]);

    (left_layout, right_layout)
}


fn get_bloc(title: &str, padding: Padding) -> Block<'_> {
    Block::bordered().border_type(BorderType::Rounded).padding(padding).title(title)
}


fn render_path_bar(frame: &mut Frame, app: &mut App, area: Rect) {
    let path = Paragraph::new(app.current_dir.display().to_string())
        .block(get_bloc("Path", Padding::ZERO))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Left);

    frame.render_widget(path, area);
}

fn render_explorer_area(frame: &mut Frame, app: &mut App, area: Rect) {
    let explorer = List::new(app.items.iter().map(|item| item.name.as_str()).collect::<Vec<_>>())
        .block(get_bloc("Explorer", BLOC_PADDING))
        .style(Style::new().white().on_black())
        .highlight_style(Style::new().bg(ratatui::style::Color::Green));

    frame.render_stateful_widget(explorer, area, &mut app.state);
}

fn render_selection_infos_area(frame: &mut Frame, app: &mut App, area: Rect) {
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
        .block(get_bloc("Infos", Padding::ZERO))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Left);

    frame.render_widget(selection, area);
}

fn render_selection_preview_area(frame: &mut Frame, app: &mut App, area: Rect) {
    let preview_selected_item;
    if let Some(item) = app.items.get(app.selected_item) {
        preview_selected_item = item.preview.as_str();
    } else {
        preview_selected_item = "";
    }


    let preview = Paragraph::new(preview_selected_item)
        .block(get_bloc("Preview", BLOC_PADDING))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Center);

    frame.render_widget(preview, area);
}