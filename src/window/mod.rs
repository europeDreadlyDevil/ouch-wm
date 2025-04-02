use ratatui::buffer::Buffer;
use ratatui::layout::{Rect};
use ratatui::prelude::{Color, Line, Style, Stylize, Widget};
use ratatui::style::Styled;
use ratatui::widgets::{Block, BorderType, Paragraph};
use crate::window::window_type::WindowType;

pub mod window_type;

#[derive(Debug, Clone)]
pub struct Window {
    window_type: WindowType,
    title: String,
    is_selected: bool,
}

impl Window {
    pub fn new(title: String, is_selected: bool, window_type: WindowType) -> Window {
        Self { window_type, title, is_selected }
    }

    pub fn selection(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }
}

impl From<&str> for Window {
    fn from(value: &str) -> Self {
        Self::new(value.into(), false, Default::default())
    }
}

impl From<(&str, bool)> for Window {
    fn from(value: (&str, bool)) -> Self {
        Self::new(value.0.into(), value.1, Default::default())
    }
}


impl From<(&str, bool, WindowType)> for Window {
    fn from(value: (&str, bool, WindowType)) -> Self {
        Self::new(value.0.into(), value.1, value.2)
    }
}

impl From<(&str, WindowType)> for Window {
    fn from(value: (&str, WindowType)) -> Self {
        Self::new(value.0.into(), false, value.1)
    }
}

impl From<WindowType> for Window {
    fn from(value: WindowType) -> Self {
        Self::new("New window".into(), false, value)
    }
}

impl Widget for &Window {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let title = Line::from(format!(" {} ", self.title))
            .bold()
            .centered();
        let border = Block::bordered()
            .border_type(BorderType::Plain)
            .title(title)
            .set_style(
                Style::default()
                    .fg(
                        if self.is_selected {
                            Color::White
                        } else {
                            Color::DarkGray
                        }
                    )
            );
        Paragraph::new("Hello")
            .block(border)
            .centered()
            .render(area, buf);
    }
}

