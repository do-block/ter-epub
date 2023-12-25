use super::render::render;
use crate::book::Book;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::ScrollbarState;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::time::Duration;

// 大纲当前选定的章或者节
#[derive(Debug, Default, Clone)]
struct Selected {
    chapter: usize,
    section: usize,
}

#[derive(Default)]
pub struct App {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub focus_content : bool,
}

pub fn handle_events(book: &mut Book) -> io::Result<()> {
    let mut chapter_input = String::new();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::default();

    loop {
        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('j') => {
                            if !app.focus_content {
                                if book.selected < book.flat_toc.len() - 1 {
                                    book.selected += 1;
                                } else {
                                    book.selected = 0;
                                }

                                app.vertical_scroll = 0;
                                app.vertical_scroll_state =
                                    app.vertical_scroll_state.position(app.vertical_scroll);

                                book.read_and_show_text();
                            } else {
                                app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                                app.vertical_scroll_state =
                                    app.vertical_scroll_state.position(app.vertical_scroll);
                            }
                        }
                        KeyCode::Char('k') => {
                            if !app.focus_content {
                                if book.selected > 0 {
                                    book.selected -= 1;
                                } else {
                                    book.selected = book.flat_toc.len() - 1;
                                }

                                app.vertical_scroll = 0;
                                app.vertical_scroll_state =
                                    app.vertical_scroll_state.position(app.vertical_scroll);

                                book.read_and_show_text();
                            } else {
                                app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                                app.vertical_scroll_state =
                                    app.vertical_scroll_state.position(app.vertical_scroll);
                            }
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            app.focus_content = true;
                        }
                        KeyCode::Char('h') | KeyCode::Left => {
                            app.focus_content = false;
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
            render(f, &book,  &mut app);
        })?;
    }
    Ok(())
}
