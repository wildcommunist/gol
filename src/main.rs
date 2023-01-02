mod cell;
mod ui;
mod input;

use bevy::prelude::*;
use bevy::winit::WinitSettings;
use crate::cell::GameOfLife;
use crate::ui::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.,
                height: 600.,
                title: format!("Game of Life"),
                ..default()
            },
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(MainMenuPlugin)
        //.add_plugin(GameOfLife)
        .run();
}
