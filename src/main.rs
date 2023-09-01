use bevy::prelude::*;
use my_game::is_debug;

fn main() {
    run()
}

fn run() {
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
    if is_debug() {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::new());
    };
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
