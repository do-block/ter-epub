use crossterm::event::{self, Event, KeyCode};

use std::io;

enum View {
    Outline,
    Content,
}

pub fn handle_events() -> io::Result<bool> {
    let current_view = View::Outline;

    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('j') | KeyCode::Down => {
                        println!("down")
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        println!("up")
                    }
                    _ => {
                        return Ok(false);
                    }
                }
            }
        }
    }
    Ok(false)
}
