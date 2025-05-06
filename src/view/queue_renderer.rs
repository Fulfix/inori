use super::search_renderer::make_search_box;
use super::Theme;
use crate::model::proto::Searchable;
use crate::model::*;
use crate::util::{format_time, song_album};
use ratatui::prelude::Constraint::*;
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::time::Duration;

use super::status_renderer::render_status;

pub fn make_progress_bar<'a>(ratio: f64, theme: &Theme) -> LineGauge<'a> {
    let progress_bar = LineGauge::default()
        .block(Block::bordered().title("Progress"))
        .filled_style(theme.progress_bar_filled)
        .unfilled_style(theme.progress_bar_unfilled)
        .line_set(symbols::line::THICK)
        .ratio(ratio);
    progress_bar
}

pub fn make_queue<'a>(model: &mut Model, theme: &Theme) -> Table<'a> {
    let rows: Vec<Row> = model
        .queue
        .contents()
        .map(|song| {
            Row::new(vec![
                Cell::from(song.title.clone().unwrap_or("".to_string())),
                Cell::from(
                    Text::from(
                        song.artist.clone().unwrap_or("Unknown Artist".into()),
                    )
                    .style(theme.status_artist)
                    .left_aligned(),
                ),
                Cell::from(
                    Text::from(
                        song_album(song)
                            .cloned()
                            .unwrap_or("Unknown Album".into()),
                    )
                    .style(theme.album)
                    .left_aligned(),
                ),
                Cell::from(
                    Text::from(format_time(
                        song.duration.unwrap_or(Duration::new(0, 0)),
                    ))
                    .left_aligned(),
                ),
            ])
            .add_modifier(
                if song
                    .place
                    .is_some_and(|s| model.status.song.is_some_and(|o| s == o))
                {
                    Modifier::ITALIC | Modifier::BOLD
                } else {
                    Modifier::empty()
                },
            )
        })
        .collect();
    let table = Table::new(
        rows,
        vec![Percentage(50), Percentage(30), Percentage(20), Min(5)],
    )
    .row_highlight_style(theme.item_highlight_active)
    .block(Block::bordered().title("Queue"));

    table
}

pub fn render(model: &mut Model, frame: &mut Frame, theme: &Theme) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Max(4), Min(1), Max(3)])
        .split(frame.area());
    let queue_and_search =
        Layout::vertical(vec![Max(3), Min(1)]).split(layout[1]);

    render_status(model, frame, layout[0], theme);
    let table = make_queue(model, theme);
    if model.queue.search.active {
        frame.render_widget(
            make_search_box(
                &model.queue.search.query,
                matches!(model.state, State::Searching),
                theme,
            ),
            queue_and_search[0],
        );
        frame.render_stateful_widget(
            table,
            queue_and_search[1],
            &mut model.queue.state,
        );
    } else {
        frame.render_stateful_widget(table, layout[1], &mut model.queue.state);
    }

    let ratio: f64 = match (model.status.elapsed, model.status.duration) {
        (Some(e), Some(t)) => e.as_secs_f64() / t.as_secs_f64(),
        _ => 0 as f64,
    };

    frame.render_widget(make_progress_bar(ratio, theme), layout[2])
}
