use std::time::Duration;

use bevy::prelude::*;
use bevy_toast::{ShowToast, ToastPlugin};
use bevy_tweening::TweeningPlugin;

use bevy::app::App;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TweeningPlugin)
        .add_plugins(ToastPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, key_handler)
        .run();
}

/// Adding an UI camera and Helper text
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            background_color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Press 'E' to show toast",
                    TextStyle {
                        font: asset_server.load("Roboto-Regular.ttf"),
                        font_size: 48.,
                        color: Color::WHITE.into(),
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..Default::default()
            });
        });
}

/// handler for keyboard key presses
fn key_handler(mut toast_evt: EventWriter<ShowToast>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::E) {
        println!("TOAST");
        toast_evt.send(ShowToast {
            title: "Achievement reached!".to_string(),
            subtitle: "Hello, World".to_string(),
            duration: Duration::from_secs(2),
        });
    }
}
