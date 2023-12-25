use super::render::render;
use crate::book::Book;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::time::Duration;

// 大纲当前选定的章或者节
#[derive(Debug, Default, Clone)]
struct Selected {
    chapter: usize,
    section: usize,
}

pub fn handle_events(book: &mut Book) -> io::Result<()> {
    let mut chapter_input = String::new();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // let mut outline_select = Selected::default();

    loop {
        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('j') => {
                            if book.selected < book.toc.len() - 1 {
                                book.selected += 1;
                            } else {
                                book.selected = 0;
                            }

                            book.read_and_show_text();
                        }
                        KeyCode::Char('k') => {
                            if book.selected > 0 {
                                book.selected -= 1;
                            } else {
                                book.selected = book.toc.len() - 1;
                            }
                            book.read_and_show_text();
                        }
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => {
                            book.read_and_show_text();
                        }
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            chapter_input.push(c);
                            // println!("Current input: {}", chapter_input);
                        }
                        KeyCode::Enter => {
                            if let Ok(selected_chapter) = chapter_input.parse::<usize>() {
                                // 确保 selected_chapter 在有效范围内
                                if selected_chapter > 0 && selected_chapter <= book.toc.len() {
                                    book.selected = selected_chapter - 1;
                                    book.read_and_show_text();
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
    Ok(())
}
