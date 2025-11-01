use bevy::prelude::*;

use crate::stellar_core::{camera, ship, solar_system};
use crate::ui;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                    camera::CameraPlugin,
                    ship::ShipPlugin,
                    solar_system::SolarSystemPlugin,
                    ui::info_ui::UIPlugin,
                )
            );
    }
}