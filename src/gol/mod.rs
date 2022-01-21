use self::{core::GolCore, input::GolInput, anim::GolAnim};
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod anim;
pub mod core;
pub mod input;
mod cell_matrix;

pub struct GoLPlugins;

pub struct GolConfig {
    pub width: u32,
    pub height: u32,
    pub color_func: fn() -> Color,
}

impl PluginGroup for GoLPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(GolCore).add(GolInput).add(GolAnim);
    }
}

#[derive(Component)]
struct Cell;

struct CellDeath(Entity);

#[derive(SystemLabel, Debug, Hash, Eq, PartialEq, Clone)]
enum Labels {
    Core,
    Input,
    Anim,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Paused,
}

#[derive(Component, Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Default for GolConfig {
    fn default() -> Self {
        Self {
            width: 20,
            height: 20,
            color_func: || Color::RED,
        }
    }
}
