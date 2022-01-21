use bevy::prelude::*;
use rand::prelude::*;
use self::gol::{GoLPlugins, GolConfig};

mod gol;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game of Life".to_owned(),
            width: 800.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(128. / 255., 204. / 255., 204. / 255.)))
        .insert_resource(GolConfig {
            width: 60,
            height: 60,
            color_func: || {
                let colors = [Color::RED, Color::BLUE, Color::GREEN, Color::YELLOW, Color::PINK, Color::PURPLE, Color::ORANGE, Color::CYAN];
                *colors.choose(&mut thread_rng()).unwrap()
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(GoLPlugins)
        .run();
}
