use ratatui::widgets::ScrollbarState;

use crate::book::Book;

#[derive(Default)]
pub struct App {
    pub content_vertical_scroll_state: ScrollbarState,
    pub outline_vertical_scroll_state: ScrollbarState,
    pub content_vertical_scroll: usize,
    pub outline_vertical_scroll: usize,
    pub focus_content: bool,
}

impl App {
    pub fn content_up(&mut self) {
        self.content_vertical_scroll = self.content_vertical_scroll.saturating_sub(1);
        self.content_vertical_scroll_state = self
            .content_vertical_scroll_state
            .position(self.content_vertical_scroll);
    }

    pub fn content_down(&mut self) {
        self.content_vertical_scroll = self.content_vertical_scroll.saturating_add(1);
        self.content_vertical_scroll_state = self
            .content_vertical_scroll_state
            .position(self.content_vertical_scroll);
    }

    pub fn reset_content_scroll(&mut self) {
        self.content_vertical_scroll = 0;
        self.content_vertical_scroll_state = self
            .content_vertical_scroll_state
            .position(self.content_vertical_scroll);
    }

    pub fn outline_up(&mut self) {
        self.outline_vertical_scroll = self.outline_vertical_scroll.saturating_sub(1);
        self.outline_vertical_scroll_state = self
            .outline_vertical_scroll_state
            .position(self.outline_vertical_scroll);
    }

    pub fn outline_down(&mut self) {
        self.outline_vertical_scroll = self.outline_vertical_scroll.saturating_add(1);
        self.outline_vertical_scroll_state = self
            .outline_vertical_scroll_state
            .position(self.outline_vertical_scroll);
    }

    pub fn reset_outline_scroll(&mut self) {
        self.outline_vertical_scroll = 0;
        self.outline_vertical_scroll_state = self
            .outline_vertical_scroll_state
            .position(self.outline_vertical_scroll);
    }

    pub fn go_top(&mut self, book: &mut Book) {
        if self.focus_content {
            self.reset_content_scroll();
        } else {
            self.reset_outline_scroll();
            book.selected = 0;
            book.read_and_show_text();
        }
    }

    pub fn go_bottom(&mut self, book: &mut Book) {
    }
}
