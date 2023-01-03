use bevy::diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use crate::input::MainCamera;

const BUTTON_ACTIVE: Color = Color::rgb(0.8, 0.8, 0.8);
const BUTTON_HOVER: Color = Color::rgb(0.4, 0.8, 0.8);
const BUTTON_DOWN: Color = Color::rgb(0.4, 1.0, 1.0);

pub struct GameExitEvent;

pub struct StartSimulationEvent;

pub struct StopSimulationEvent;

pub struct ResetSimulationEvent;

pub struct MarkCellForDeathEvent;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct StatsText;

#[derive(Component)]
pub struct ClassicButton(ButtonType);

#[derive(PartialEq, Copy, Clone)]
enum ButtonType {
    Start,
    Stop,
    Exit,
    Reset,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameExitEvent>()
            .add_event::<StartSimulationEvent>()
            .add_event::<StopSimulationEvent>()
            .add_event::<ResetSimulationEvent>()
            .add_startup_system(setup)
            .add_system(button_system)
            .add_system(fps_update_system)
            .add_system(stats_update_system);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/minecraft_font.ttf"),
                    font_size: 15.0,
                    color: Color::ANTIQUE_WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/minecraft_font.ttf"),
                font_size: 15.0,
                color: Color::GOLD,
            })
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            })
        ,
        FpsText
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Zoom: ",
                TextStyle {
                    font: asset_server.load("fonts/minecraft_font.ttf"),
                    font_size: 15.0,
                    color: Color::ANTIQUE_WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/minecraft_font.ttf"),
                font_size: 15.0,
                color: Color::GOLD,
            }),
            TextSection::new(
                " Camera: ",
                TextStyle {
                    font: asset_server.load("fonts/minecraft_font.ttf"),
                    font_size: 15.0,
                    color: Color::ANTIQUE_WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/minecraft_font.ttf"),
                font_size: 15.0,
                color: Color::GOLD,
            })
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(25.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            })
        ,
        StatsText
    ));

    commands.spawn(
        NodeBundle { // Root
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                position: UiRect::bottom(Val::Percent(0.0)),
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| { // Button group
            parent
                .spawn(
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Px(75.0)),
                            border: UiRect::all(Val::Px(3.0)),
                            ..Default::default()
                        },
                        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                        ..Default::default()
                    })
                .with_children(|parent| {
                    parent // Button background filler
                        .spawn(
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    align_items: AlignItems::FlexEnd,
                                    ..Default::default()
                                },
                                background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                                ..Default::default()
                            }
                        )
                        .with_children(|parent| {
                            parent
                                .spawn(build_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn(build_text("Play", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Start));

                            parent
                                .spawn(build_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn(build_text("Stop", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Stop));

                            parent
                                .spawn(build_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn(build_text("Reset", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Reset));

                            parent
                                .spawn(build_button(&asset_server))
                                .with_children(|parent| {
                                    parent.spawn(build_text("QUIT", &asset_server));
                                })
                                .insert(ClassicButton(ButtonType::Exit));
                        });
                });
        });
}

fn build_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn build_text(value: &str, assets: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            value,
            TextStyle {
                font: assets.load("fonts/minecraft_font.ttf"),
                font_size: 30.0,
                color: Default::default(),
            },
        ),
        ..Default::default()
    }
}

fn button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ClassicButton), (Changed<Interaction>, With<Button>)>,
    mut start_writer: EventWriter<StartSimulationEvent>,
    mut stop_writer: EventWriter<StopSimulationEvent>,
    mut exit_writer: EventWriter<GameExitEvent>,
    mut reset_writer: EventWriter<ResetSimulationEvent>,
) {
    for (i, mut bc, cb) in query.iter_mut() {
        match *i {
            Interaction::Clicked => {
                *bc = BUTTON_DOWN.into();
                match cb.0 {
                    ButtonType::Start => {
                        start_writer.send(StartSimulationEvent);
                    }
                    ButtonType::Stop => {
                        stop_writer.send(StopSimulationEvent);
                    }
                    ButtonType::Exit => {
                        exit_writer.send(GameExitEvent);
                    }
                    ButtonType::Reset => {
                        reset_writer.send(ResetSimulationEvent);
                    }
                }
            }
            Interaction::Hovered => {
                *bc = BUTTON_HOVER.into();
            }
            Interaction::None => {
                *bc = BUTTON_ACTIVE.into();
            }
        }
    }
}

fn fps_update_system(
    diag: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diag.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(v) = fps.smoothed() {
                text.sections[1].value = format!("{v:.2}");
            }
        }
    }
}

fn stats_update_system(
    diag: Res<Diagnostics>,
    mut query: Query<(&mut Text), With<StatsText>>,
    camera: Query<(&Transform, &OrthographicProjection), With<MainCamera>>,
) {
    for mut text in &mut query {
        let (trans, cam) = camera.single();
        text.sections[1].value = format!("{:.2}", cam.scale);
        text.sections[3].value = format!("x:{:.2} y:{:.2}", trans.translation.x, trans.translation.y);
    }
}