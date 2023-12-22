use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListDirection, Paragraph, Wrap},
    Frame,
};

use crate::book::Book;

pub fn render(frame: &mut Frame, book: &Book) {
    let mut index = 1;
    let items = book
        .toc
        .iter()
        .map(|item| {
            // item.title.clone()
            // index 从 1 开始
            index += 1;
            format!("第{}章. {}", index, item.title.clone())
        })
        .collect::<Vec<String>>();

    let list = List::new(items)
        .block(Block::default().title("大纲").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Min(30), Constraint::Min(0)])
        .split(frame.size());

    frame.render_widget(list, layout[0]);

    let content = book.context.clone();

    frame.render_widget(
        Paragraph::new(content)
            .block(Block::default().title("内容").borders(Borders::ALL))
            .wrap(Wrap { trim: true }),
        layout[1],
    );
}
