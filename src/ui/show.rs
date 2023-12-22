use super::render::render;
use crate::book::Book;
use crossterm::event::{Event, KeyCode};
use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::{
    io::{self, stdout},
    time::Duration,
};

pub fn start(book: &mut Book) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let mut chapter_input = String::new();

    loop {
        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        // KeyCode::Char('j') => app.increment(),
                        // KeyCode::Char('k') => app.decrement(),
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => {book.read_text();},
                        KeyCode::Char(c) if c.is_digit(10) => {
                            chapter_input.push(c);
                            // println!("Current input: {}", chapter_input);
                        }
                        KeyCode::Enter => {
                            if let Ok(selected_chapter) = chapter_input.parse::<usize>() {
                                // 确保 selected_chapter 在有效范围内
                                if selected_chapter > 0 && selected_chapter <= book.toc.len() {
                                    book.selected = selected_chapter - 1;
                                    book.read_text();
                                }
                                // ...处理选择章节的逻辑...
                                // println!("Selected chapter: {}", selected_chapter);
                            } else {
                                // println!("Invalid chapter number");
                            }
                            // 重置输入以准备下一次输入
                            chapter_input.clear();
                        }
                        KeyCode::Backspace => {
                            // 允许用户删除输入
                            chapter_input.pop();
                            // println!("Current input: {}", chapter_input);
                        }
                        _ => {
                            // println!("key: {:?}", key);
                        }
                    }
                }
            }
        }

        terminal.draw(|f| {
            render(f, &book);
        })?;
    }

    disable_raw_mode()?;

    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
