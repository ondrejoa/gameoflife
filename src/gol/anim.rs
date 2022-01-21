use super::{Cell, GolConfig, Labels, Position};
use crate::gol::CellDeath;
use bevy::prelude::*;
use std::cmp::min;

pub struct GolAnim;

impl Plugin for GolAnim {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(
                cell_death
                    .label(Labels::Anim)
                    .after(Labels::Core)
                    .after(Labels::Input),
            )
            .add_system_to_stage(CoreStage::PostUpdate, add_cell_sprite)
            .add_system_to_stage(CoreStage::PostUpdate, live_cell_aging)
            .add_system_to_stage(CoreStage::PostUpdate, dead_cell_decay)
            .add_system_to_stage(CoreStage::Last, cell_scale)
            .add_system_to_stage(CoreStage::Last, cell_translate);
    }
}

const MAX_CELL_AGE: u32 = 15;
const CELL_AGING_SPEED: u32 = 1;
const CELL_DECAY_SPEED: u32 = 2;

#[derive(Component)]
struct CellAge(u32);

#[derive(Component)]
struct DeadCell;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_cell_sprite(
    mut commands: Commands,
    config: Res<GolConfig>,
    mut cells: Query<Entity, (With<Cell>, Without<Sprite>)>,
) {
    for entity in cells.iter_mut() {
        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: (config.color_func)(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(CellAge(0));
    }
}

fn cell_death(
    mut commands: Commands,
    mut death: EventReader<CellDeath>,
    cells: Query<(&Position, &Sprite), With<Cell>>,
) {
    for entity in death.iter() {
        if let Ok((position, sprite)) = cells.get(entity.0) {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: sprite.clone(),
                    ..Default::default()
                })
                .insert(DeadCell)
                .insert(*position)
                .insert(CellAge(MAX_CELL_AGE));
        }
    }
}

fn live_cell_aging(mut cells: Query<(&mut CellAge, &mut Sprite), With<Cell>>) {
    for (mut age, mut sprite) in cells.iter_mut() {
        age.0 = min(age.0 + CELL_AGING_SPEED, MAX_CELL_AGE);
        sprite.color.set_a(age.0 as f32 / MAX_CELL_AGE as f32); // A linearis nagyon gagyi
    }
}

fn dead_cell_decay(mut cells: Query<(&mut CellAge, &mut Sprite), With<DeadCell>>) {
    for (mut age, mut sprite) in cells.iter_mut() {
        age.0 = min(age.0 + CELL_DECAY_SPEED, 2 * MAX_CELL_AGE);
        sprite
            .color
            .set_a(1.0 - ((age.0 - MAX_CELL_AGE) as f32 / MAX_CELL_AGE as f32));
    }
}

fn cell_scale(
    windows: Res<Windows>,
    config: Res<GolConfig>,
    mut cells: Query<(&mut Transform, &CellAge), Or<(With<Cell>, With<DeadCell>)>>,
) {
    let window = windows.get_primary().unwrap();
    for (mut transform, &CellAge(age)) in cells.iter_mut() {
        let age_scale = age as f32 / MAX_CELL_AGE as f32;
        transform.scale = Vec3::new(
            age_scale * window.width() / config.width as f32,
            age_scale * window.height() / config.height as f32,
            1.0,
        );
    }
}

fn cell_translate(
    windows: Res<Windows>,
    config: Res<GolConfig>,
    mut cells: Query<(&mut Transform, &Position), Or<(With<Cell>, With<DeadCell>)>>,
) {
    let window = windows.get_primary().unwrap();
    fn translation(pos: f32, bound_window: f32, tile_size: f32) -> f32 {
        pos * tile_size - (bound_window / 2.) + (tile_size / 2.)
    }
    for (mut transform, &Position { x, y }) in cells.iter_mut() {
        transform.translation.x = translation(
            x as f32,
            window.width(),
            window.width() / config.width as f32,
        );
        transform.translation.y = translation(
            y as f32,
            window.height(),
            window.height() / config.height as f32,
        );
    }
}
