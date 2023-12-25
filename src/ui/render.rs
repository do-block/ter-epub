use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::book::Book;

pub fn render(frame: &mut Frame, book: &Book) {
    let mut index = 0;
    let mut content_title = String::new();

    let items = book
        .toc
        .iter()
        .map(|item| {
            let mut select_tag = ' ';
            let mut fg = Style::default().fg(Color::LightCyan);

            if index == book.selected {
                content_title = item.title.clone();
                select_tag = '*';
                fg = get_select_fg(true);
            }

            let mut text = Text::default();

            let mut extends = vec![];

            extends.push(Span::styled(
                format!("{} {}", select_tag, item.title.clone()),
                fg,
            ));

            index += 1;

            if !item.children.is_empty() {
                item.children.iter().for_each(|child| {
                    if child.title.trim() != item.title.trim() {
                        let mut select_tag = ' ';

                        let mut fg = Style::default().fg(Color::White);

                        if index == book.selected {
                            content_title = child.title.clone();
                            select_tag = '*';
                            fg = get_select_fg(true);
                        }

                        extends.push(Span::styled(
                            format!("    {} {}", select_tag, child.title.clone()),
                            fg,
                        ));
                    }

                    index += 1;
                });
            }

            text.extend(extends);

            ListItem::new(text)
        })
        .collect::<Vec<ListItem>>();

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

    let content = format!("{}:\n\n {}", content_title, book.context);

    frame.render_widget(
        Paragraph::new(content)
            .block(Block::default().title("内容").borders(Borders::ALL))
            .wrap(Wrap { trim: true }),
        layout[1],
    );
}

fn get_select_fg(light: bool) -> Style {
    if light {
        Style::default().bg(Color::LightBlue).fg(Color::White)
    } else {
        Style::default().bg(Color::Blue).fg(Color::Cyan)
    }
}
