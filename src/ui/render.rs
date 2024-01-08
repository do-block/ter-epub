use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{block::Title, Block, Borders, Paragraph, Wrap},
    Frame,
};
use ratatui::{prelude::*, widgets::*};
use crate::book::Book;
use super::app::App;

pub fn render(frame: &mut Frame, book: &Book, app: &mut App) {
    let mut content_title = String::new();
    let mut index = 0;

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut items = vec![];
    book.toc.iter().for_each(|item| {
        let mut select_tag = ' ';
        let mut fg = Style::default().fg(Color::LightCyan);
        if index == book.selected {
            content_title = item.title.clone();
            select_tag = '*';
            fg = get_select_fg(true);
        }

        items.push(Line::from(vec![Span::styled(
            format!("{} {}", select_tag, item.title.clone()),
            fg,
        )]));
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
                    items.push(Line::from(vec![Span::styled(
                        format!("  {} {}", select_tag, child.title.clone()),
                        fg,
                    )]));
                }
                index += 1;
            });
        }
    });

    let size = frame.size();

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Min(30), Constraint::Min(0)])
        .split(size);

    // -------- outline scroll config start --------
    let outline_line_count = items.len();
    let outline_index_text = format!("[{}/{}]", outline_line_count + 1, book.selected + 1);
    let mut outline_titile = Title::from(format!("大纲 {}", outline_index_text).white().on_gray());
    if !app.focus_content {
        outline_titile = Title::from(format!("大纲 {}", outline_index_text).gray().on_white());
    }
    app.outline_vertical_scroll_state = app
        .outline_vertical_scroll_state
        .content_length(outline_line_count);
    frame.render_widget(
        Paragraph::new(items)
            .block(
                Block::default()
                    .title(outline_titile)
                    .borders(Borders::ALL)
                    .padding(Padding::new(0, 0, 0, 0)),
            )
            .style(Style::default().fg(Color::White))
            .scroll((app.outline_vertical_scroll as u16, 0)),
        layout[0],
    );
    frame.render_stateful_widget(
        scrollbar.clone(),
        layout[0],
        &mut app.outline_vertical_scroll_state,
    );
    // -------- outline scroll config end --------
    
    // let pure_text =  html_to_text(&book.context);
    let content = format!("{}", &book.context);

    // -------- content scroll config start --------
    app.content_vertical_scroll_state = app
        .content_vertical_scroll_state
        .content_length(book.context.lines().count());

    let mut content_title = Title::from("内容".gray().on_white());

    if app.focus_content {
        content_title = Title::from("内容 [按left或者h回到大纲]".white().bold().on_gray());
    }

    frame.render_widget(
        Paragraph::new(content)
            .block(Block::default().title(content_title).borders(Borders::ALL))
            .wrap(Wrap { trim: true })
            .scroll((app.content_vertical_scroll as u16, 0)),
        layout[1],
    );
    frame.render_stateful_widget(scrollbar, layout[1], &mut app.content_vertical_scroll_state);
    // -------- content  scroll config end --------
}

fn get_select_fg(light: bool) -> Style {
    if light {
        Style::default().bg(Color::LightBlue).fg(Color::White)
    } else {
        Style::default().bg(Color::Blue).fg(Color::Cyan)
    }
}
