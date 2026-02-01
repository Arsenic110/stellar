use bevy::prelude::*;

pub mod luminosity;
pub use luminosity::Luminosity;

use super::{Mass, Radius};

use crate::stellar_core::solar_system::Orbit;
use crate::procedural_generation::{self, gen_star as gen};

#[derive(Clone, Component)]
pub struct Star {
    pub spectral_type: String,
}

impl Star {
    pub fn get_bundle(
        star: Self, 
        mass: Mass, 
        radius: Radius, 
        luminosity: Luminosity, 
        x: f32, y: f32, 
        mut images: &mut ResMut<Assets<Image>>, 
) -> (Self, Mass, Radius, Luminosity, Sprite, Transform) {
        let tex_size = (*radius as u32 * 10).max(32);
        let custom_size = Vec2::splat((*radius as f32 * 100.0).max(1000.0));

        (
            star,
            mass,
            radius,
            luminosity,
            Sprite { 
                image: procedural_generation::gen_icon::circle_texture(
                    tex_size, tex_size, &mut images,
                    255, 225, 30, 255
                ),
                custom_size: Some(custom_size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0)
        )

    }
}