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
    pub block_active: Style,
    pub field_album: Style,
    pub field_artistsort: Style,
    pub item_highlight_active: Style,
    pub item_highlight_inactive: Style,
    pub progress_bar_filled: Style,
    pub progress_bar_unfilled: Style,
    pub search_query_active: Style,
    pub search_query_inactive: Style,
    pub slash_span: Style,
    pub status_album: Style,
    pub status_artist: Style,
    pub status_paused: Style,
    pub status_playing: Style,
    pub status_stopped: Style,
    pub status_title: Style,
}
impl Theme {
    pub fn new() -> Self {
        Self {
            block_active: Style::default().fg(LightYellow),
            field_album: Style::default().bold().italic().fg(LightYellow),
            field_artistsort: Style::default().fg(LightYellow),
            item_highlight_active: Style::default().fg(Black).bg(LightYellow),
            item_highlight_inactive: Style::default().fg(Black).bg(LightYellow),
            progress_bar_filled: Style::default()
                .fg(LightYellow)
                .bg(Black)
                .add_modifier(Modifier::BOLD),
            progress_bar_unfilled: Style::default().fg(Black),
            search_query_active: Style::default().bg(LightYellow).fg(Black),
            search_query_inactive: Style::default().bg(LightYellow).fg(Black),
            slash_span: Style::default().fg(LightYellow),
            status_album: Style::default().bold().italic().fg(LightYellow),
            status_artist: Style::default().fg(LightYellow),
            status_paused: Style::default().fg(LightYellow),
            status_playing: Style::default().fg(LightYellow),
            status_stopped: Style::default().fg(LightYellow),
            status_title: Style::default().bold(),
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

