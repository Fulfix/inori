use crate::model::*;
use ratatui::prelude::*;
use ratatui::style::Color::*;
use ratatui::style::Style;
mod artist_select_renderer;
pub mod library_renderer;
pub mod queue_renderer;
mod search_renderer;
mod status_renderer;
mod track_select_renderer;

#[derive(Clone)]
pub struct Theme {
    pub item_highlight_active: Style,
    pub item_highlight_inactive: Style,
    pub block_active: Style,
    pub status_artist: Style,
    pub status_album: Style,
    pub status_title: Style,
    pub artist_sort: Style,
    pub album: Style,
    pub playing: Style,
    pub paused: Style,
    pub stopped: Style,
    pub slash_span: Style,
    pub search_query_active: Style,
    pub search_query_inactive: Style,
}
impl Theme {
    pub fn new() -> Theme {
    Theme {
        item_highlight_active: Style::new().fg(Color::LightYellow).bg(Color::Black),  // LightMagenta foreground, Black background
        item_highlight_inactive: Style::new().fg(Color::LightYellow).bg(Color::Black), // LightMagenta foreground, Black background
        block_active: Style::new().fg(Color::LightYellow).bg(Color::Black),            // LightMagenta foreground, Black background
        status_artist: Style::new().fg(Color::LightYellow).bg(Color::Black),           // LightMagenta foreground, Black background
        status_album: Style::new().fg(Color::LightYellow).bg(Color::Black),            // LightMagenta foreground, Black background
        status_title: Style::new().fg(Color::LightYellow).bg(Color::Black),            // LightMagenta foreground, Black background
        artist_sort: Style::new().fg(Color::LightYellow).bg(Color::Black),             // LightMagenta foreground, Black background
        album: Style::new().fg(Color::LightYellow).bg(Color::Black),                   // LightMagenta foreground, Black background
        playing: Style::new().fg(Color::LightYellow).bg(Color::Black),                 // LightMagenta foreground, Black background
        paused: Style::new().fg(Color::LightYellow).bg(Color::Black),                  // LightMagenta foreground, Black background
        stopped: Style::new().fg(Color::LightYellow).bg(Color::Black),                 // LightMagenta foreground, Black background
        slash_span: Style::new().fg(Color::LightYellow).bg(Color::Black),              // LightMagenta foreground, Black background
        search_query_active: Style::new().fg(Color::LightYellow).bg(Color::Black),     // LightMagenta foreground, Black background
        search_query_inactive: Style::new().fg(Color::LightYellow).bg(Color::Black),   // LightMagenta foreground, Black background
    }
}

}

pub fn view(model: &mut Model, frame: &mut Frame) {
    // only &mut for ListState/TableState updating.
    // view function should be pure!

    let theme = model.config.theme.clone();
    match model.screen {
        Screen::Library => library_renderer::render(model, frame, &theme),
        Screen::Queue => queue_renderer::render(model, frame, &theme),
    }
}

