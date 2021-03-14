use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Debug, Default)]
pub struct FpsDisplay;

impl Plugin for FpsDisplay {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(fps_display_init.system())
            .add_system(fps_display_update.system());
    }
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
        if let Some(fps_avg) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps_diag| fps_diag.average())
        {
            text.sections[0].value = format!("FPS: {:.1}", fps_avg);
        }
    }
}
