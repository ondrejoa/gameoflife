use super::{cell_matrix::CellMatrix, AppState, Cell, CellDeath, GolConfig, Labels, Position};
use bevy::prelude::*;

pub struct GolCore;

impl Plugin for GolCore {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Paused)
            .add_event::<CellDeath>()
            .insert_resource(CoreTimer(Timer::from_seconds(0.5, true)))
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(apply_rules.label(Labels::Core)),
            )
            .add_system_to_stage(CoreStage::Last, cell_death);
    }
}

struct CoreTimer(Timer);

fn apply_rules(
    mut commands: Commands,
    mut death: EventWriter<CellDeath>,
    config: Res<GolConfig>,
    cells: Query<(Entity, &Position), With<Cell>>,
    time: Res<Time>,
    mut timer: ResMut<CoreTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let cm = CellMatrix::new(config.width, config.height, cells.iter());
        for (entity, nc) in cm.live_cells() {
            if !(nc == 2 || nc == 3) {
                death.send(CellDeath(entity));
            }
        }
        for (position, nc) in cm.dead_cells() {
            if nc == 3 {
                commands.spawn().insert(Cell).insert(position);
            }
        }
    }
}

fn cell_death(mut commands: Commands, mut death: EventReader<CellDeath>) {
    for &CellDeath(entity) in death.iter() {
        commands.entity(entity).despawn();
    }
}
