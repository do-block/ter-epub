use super::{app, render::render};
use crate::book::Book;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::time::Duration;

pub fn handle_events(book: &mut Book) -> io::Result<()> {
    let mut chapter_input = String::new();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = app::App::default();

    loop {
        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('j') => {
                            if !app.focus_content {
                                if book.selected < book.flat_toc.len() - 1 {
                                    book.selected += 1;
                                }

                                app.outline_down();
                                book.read_and_show_text();
                            } else {
                                app.content_down();
                            }
                        }

                        KeyCode::Char('k') => {
                            if !app.focus_content {
                                if book.selected > 0 {
                                    book.selected -= 1;
                                }
                                app.outline_up();
                                book.read_and_show_text();
                            } else {
                                app.content_up();
                            }
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            app.focus_content = true;
                            app.reset_content_scroll();
                        }
                        KeyCode::Char('h') | KeyCode::Left => {
                            app.focus_content = false;
                        }
                        // 拦截g开头的按键
                        KeyCode::Char('g') => {
                            match crossterm::event::read()? {
                                Event::Key(key) => match key.code {
                                    KeyCode::Char('g') => {
                                        app.go_top(book);
                                    }
                                    _ => {
                                        // println!("key: {:?}", key);
                                    }
                                },
                                _ => {}
                            }
                        }
                        KeyCode::Char('G') => {
                            // TODO: go to bottom
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
                            } else {
                                // println!("Invalid chapter number");
                            }
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
            render(f, &book, &mut app);
        })?;
    }
    Ok(())
}
