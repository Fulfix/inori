use super::Theme;
use crate::model::selector_state::*;
use crate::model::LibActiveSelector::*;
use crate::model::*;
use crate::util::format_time;
use ratatui::prelude::Constraint::*;
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::time::Duration;

fn get_track_data<'a>(
    artist: Option<&ArtistData>,
    theme: &Theme,
    width: u16,
) -> Table<'a> {
    if let Some(artist) = artist {
        let items = artist
            .contents()
            .iter()
            .map(|i| match i {
                TrackSelItem::Album(a) => Row::new(vec![
                    Text::from(format!(
                        " {} {}",
                        a.name.clone(),
                        &str::repeat("─", width.into())
                    )),
                    Text::from(format_time(a.total_time())).right_aligned(),
                ])
                .style(theme.album),
                TrackSelItem::Song(s) => Row::new(vec![
                    Text::from(
                        str::repeat(" ", 3)
                            + &s.title.clone().unwrap_or("Unknown Song".into()),
                    ),
                    Text::from(format_time(
                        s.duration.unwrap_or(Duration::from_secs(0)),
                    ))
                    .right_aligned(),
                ]),
            })
            .collect::<Vec<Row>>();
        Table::new::<Vec<Row>, Vec<Constraint>>(items, vec![Min(10), Max(9)])
    } else {
        return Table::new::<Vec<Row>, Vec<u16>>(vec![], vec![]);
    }
}

pub fn render_track_list(
    model: &mut Model,
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
) {
    let list = get_track_data(model.library.selected_item(), theme, area.width)
        .block(
            match model.library.active {
                ArtistSelector => Block::bordered(),
                TrackSelector => {
                    Block::bordered().border_style(theme.block_active)
                }
            }
            .title("Tracks"),
        )
        .highlight_style(match model.library.active {
            ArtistSelector => theme.item_highlight_inactive,
            TrackSelector => theme.item_highlight_active,
        })
        .highlight_spacing(HighlightSpacing::Always);

    match model.library.selected_item_mut() {
        Some(artist) => frame.render_stateful_widget(
            list,
            area,
            &mut artist.track_sel_state,
        ),
        None => frame.render_widget(list, area),
    }
}
