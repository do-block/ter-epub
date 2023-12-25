use super::event::handle_events;
use crate::book::Book;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, stdout};

pub fn start(book: &mut Book) -> io::Result<()> {
    enable_raw_mode()?;

    stdout().execute(EnterAlternateScreen)?;

    handle_events(book)?;

    disable_raw_mode()?;

    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
