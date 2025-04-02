use std::collections::HashMap;
use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{StreamExt, FutureExt};
use ratatui::{DefaultTerminal, Frame};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::macros::{horizontal, vertical};
use ratatui::prelude::{Layout, Stylize, Widget};
use crate::window::{Window};
use crate::window::window_type::WindowType;

pub mod window;

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    event_stream: EventStream,
    windows: Vec<Window>,
    view_grid: Vec<(Layout, Vec<usize>)>,
    selected_window: usize,
}

pub enum LayoutType {
    Horizontal,
    Vertical,
}

impl App {
    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {

        self.running = true;
        self.windows.push(
            Window::from(("Desktop", true))
        );
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    async fn handle_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                match event {
                    Some(Ok(evt)) => {
                        match evt {
                            Event::Key(key)
                            if key.kind == KeyEventKind::Press
                                => self.on_key_event(key),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {

            }
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (KeyModifiers::CONTROL, KeyCode::Char('T') | KeyCode::Char('t')) => self.create_window(WindowType::Terminal, LayoutType::Horizontal),
            (KeyModifiers::ALT, KeyCode::Char('T') | KeyCode::Char('t')) => self.create_window(WindowType::Terminal, LayoutType::Vertical),
            (_, KeyCode::Char('h') | KeyCode::Char('H')) => {
                self.windows[self.selected_window].selection(false);
                if self.selected_window == 0 {
                    self.selected_window = self.windows.len() - 1;
                } else {
                    self.selected_window = self.selected_window.saturating_sub(1);
                }
                self.windows[self.selected_window].selection(true);
            }
            (_, KeyCode::Char('l') | KeyCode::Char('L')) => {
                self.windows[self.selected_window].selection(false);
                if self.selected_window > self.windows.len() - 2 {
                    self.selected_window = 0;
                } else {
                    self.selected_window = self.selected_window.saturating_add(1);
                }
                self.windows[self.selected_window].selection(true);
            }
            _ => {}
        }
    }
    fn quit(&mut self) {
        self.running = false;
    }

    fn create_window(&mut self, window_type: WindowType, layout_type: LayoutType) {
        self.windows = [&self.windows[..self.selected_window+1], &[Window::from((self.windows.len().to_string().as_str(), window_type))], &self.windows[self.selected_window+1..]].concat();
        self.view_grid = self.view_grid.iter().map(|(layout, v)| {
            if v[0] == self.selected_window {
                (layout.clone(), vec![v[0], v[1] + 1])
            } else if v[0] > self.selected_window && v[1] > self.selected_window {
                (layout.clone(), vec![v[0] + 1, v[1] + 1])
            } else if v[0] > self.selected_window && v[1] < self.selected_window {
                (layout.clone(), vec![v[0] + 1, v[1]])
            } else if v[0] < self.selected_window && v[1] > self.selected_window {
                (layout.clone(), vec![v[0], v[1]+1])
            } else {
                (layout.clone(), vec![v[0], v[1]])
            }
        }).collect();

        match layout_type {
            LayoutType::Horizontal => self.view_grid.push((horizontal![==50%, ==50%], vec![self.selected_window, self.selected_window + 1])),
            LayoutType::Vertical => self.view_grid.push((vertical![==50%, ==50%], vec![self.selected_window, self.selected_window + 1])),
        }

    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        //info!("{:?}", self.windows);
        let mut grid_map = HashMap::<usize, Rect>::new();
        if self.view_grid.is_empty() {
            self.windows[0].render(area, buf);
            return;
        }
        for (layout, window_indexes) in &self.view_grid {
            let mut areas = layout.split(area).to_vec();
            for i in window_indexes {
                if let Some(area) = grid_map.get_mut(i) {
                    areas = layout.split(area.clone()).to_vec();
                    grid_map.insert(*i, areas[0]);
                    areas.remove(0);
                } else {
                    grid_map.insert(*i, areas[0]);
                    areas.remove(0);
                }
            }
        }

        for (i, area) in grid_map {
            self.windows[i].render(area, buf);
        }
    }
}