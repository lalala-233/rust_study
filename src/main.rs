use std::env;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mine Sweeper!".to_string(),
            resolution: (1920., 1080.).into(),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, camera_setup);
    if_debug(&mut app);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn if_debug(app: &mut App) {
    let mut args = env::args();
    args.next();
    if args.next() == Some("debug".to_string()) {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::new());
    }
}
