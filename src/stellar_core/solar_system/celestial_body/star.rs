use bevy::prelude::*;

pub mod luminosity;
pub use luminosity::Luminosity;

use super::Mass;

use crate::stellar_core::solar_system::Orbit;
use crate::procedural_generation::{self, gen_star as gen};

#[derive(Clone, Component)]
pub struct Star {
    pub radius: f64,
    pub spectral_type: String,
}

impl Star {
    pub fn get_bundle(
        star: Self, mass: Mass, luminosity: Luminosity, 
        x: f32, y: f32, mut images: &mut ResMut<Assets<Image>>, 
    ) -> (Self, Mass, Luminosity, Sprite, Transform) {
        let radius = star.radius;
        let tex_size = (radius as u32 * 100).max(16);

        (
            star,
            mass,
            luminosity,
            Sprite { 
                image: procedural_generation::gen_icon::circle_texture(
                    tex_size, tex_size, &mut images,
                    255, 225, 30, 255
                ),
                custom_size: Some(Vec2::splat((radius as f32 * 500.0).max(1000.0))),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0)
        )

    }
}