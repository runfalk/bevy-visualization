use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::system::exit_on_esc_system,
    prelude::*,
};

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
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(fps_display_init.system())
        .add_system(exit_on_esc_system.system())
        .add_system(fps_display_update.system())
        .run();
}

fn fps_display_init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraMono-Medium.ttf");

    commands
        .spawn(UiCameraBundle::default())
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "FPS: <unknown>",
                TextStyle {
                    font,
                    font_size: 18.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .with(FPSDisplay);
}

struct FPSDisplay;

fn fps_display_update(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FPSDisplay>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps_diag) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diag.average() {
                text.sections[0].value = format!("FPS: {:.1}", fps_avg);
            }
        }
    }
}
