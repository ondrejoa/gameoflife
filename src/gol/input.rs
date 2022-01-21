use super::{Cell, GolConfig, Labels, Position};
use crate::gol::{AppState, CellDeath};
use bevy::prelude::*;

pub struct GolInput;

impl Plugin for GolInput {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Paused).with_system(click.label(Labels::Input)),
        )
        .add_system(resume_pause);
    }
}

fn click(
    mut commands: Commands,
    mut death: EventWriter<CellDeath>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    config: Res<GolConfig>,
    mut cells: Query<(Entity, &Position), With<Cell>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(cursor) = window.cursor_position() {
            let position = Position {
                x: (cursor.x / (window.width() / config.width as f32)) as i32,
                y: (cursor.y / (window.height() / config.height as f32)) as i32,
            };
            if let Some(entity) =
                cells
                    .iter_mut()
                    .find_map(|(entity, pos)| if position == *pos { Some(entity) } else { None })
            {
                death.send(CellDeath(entity));
            } else {
                commands.spawn().insert(Cell).insert(position);
            }
        }
    }
}

fn resume_pause(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::S) {
        let _ = match app_state.current() {
            AppState::InGame => app_state.set(AppState::Paused),
            AppState::Paused => app_state.set(AppState::InGame),
        };
    }
}
