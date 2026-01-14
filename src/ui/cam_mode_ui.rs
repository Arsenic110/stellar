use bevy::prelude::*;
use crate::stellar_core::camera::{CamChase, };

//plugin to wrap bevy functionality
pub struct CamUIPlugin;
impl Plugin for CamUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, update)
        ;
    }
}

//marker struct to identify the span
#[derive(Component)]
struct CamModeUIMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font = TextFont {
        font: asset_server.load("fonts/vcr_osd_mono.ttf"),
        font_size: 30.0,
        ..default()
    };

    commands.spawn((
        Text::new("Cam (spacebar): "),
        font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(5.0),
            ..default()
        },
    ))
    .with_child((
        TextSpan::default(),
        font,
        CamModeUIMarker
    ))
    ;

}

fn update(
    mut query: Query<&mut TextSpan, With<CamModeUIMarker>>,
    mode_query: Query<Option<&CamChase>, With<Camera2d>>
) {
    for mut span in &mut query {
        for mode in &mode_query {
            **span = format!("{}", match mode {Some(_) => "Chase", None => "Free"});
        }
    }
}
