mod app;
mod ui;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

fn main() {
    ratatui::run(app);
}


fn app(terminal: &mut DefaultTerminal) {
    let mut app  = app::App::new();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app)).expect("Error");
        match crossterm::event::read().expect("No key pressed") {
            Event::Key(event) => {
                if event.kind == KeyEventKind::Press {
                    match event.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Down => {
                            app.select_next_item()
                        }
                        KeyCode::Up => {
                            app.select_previous_item()
                        }
                        KeyCode::Enter => {
                            app.open_directory()
                        }
                        KeyCode::Esc => {
                            app.go_to_parent_directory()
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}