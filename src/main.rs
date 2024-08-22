#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

#[derive(Component)]
struct MainCamera;

fn main() {
    let window = Window {
        title: "Template".into(),
        name: Some("template.app".into()),
        resolution: (800., 500.).into(),
        ..default()
    };

    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
                ..default()
            },
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                }),
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
