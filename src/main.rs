use bevy::{input::system::exit_on_esc_system, prelude::*};

mod bots;
mod debug;
mod grid;
mod rng;

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
        .add_plugin(rng::RngPlugin::from(1984))
        .add_plugin(debug::FpsDisplay::default())
        .add_plugin(grid::GridPlugin::new(100, 100))
        .add_plugin(bots::BotPlugin::new(1_000))
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, grid: Res<grid::GridProperties>) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_xyz(-4.0, 8.0, 4.0),
            ..Default::default()
        })
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-10.0, 35.0, -20.0)
                .looking_at(grid.coord_to_vec3(&grid::Coordinate::new(30, 30)), Vec3::Y),
            ..Default::default()
        });
}
