mod ui;
mod input;
mod simulation;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use crate::input::InputPlugin;
use crate::simulation::SimulationPlugin;
use crate::ui::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1024.,
                height: 1099.,
                title: format!("Game of Life"),
                ..default()
            },
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.insert_resource(WinitSettings::desktop_app())
        .add_plugin(MainMenuPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SimulationPlugin)
        //.add_plugin(GameOfLife)
        .run();
}
