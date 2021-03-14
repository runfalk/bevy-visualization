use bevy::{input::system::exit_on_esc_system, prelude::*};

mod debug;

fn main() {
    App::build()
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Visualizer".to_string(),
            width: 1280.,
            height: 720.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(debug::FpsDisplay::default())
        .add_system(exit_on_esc_system.system())
        .run();
}
