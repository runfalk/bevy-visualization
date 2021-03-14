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
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_xyz(-4.0, -4.0, 8.0),
            ..Default::default()
        })
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_xyz(2.0, -5.0, 2.5).looking_at(Vec3::ZERO, Vec3::Z),
            ..Default::default()
        });
}
