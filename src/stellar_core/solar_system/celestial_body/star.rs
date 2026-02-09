use bevy::prelude::*;

pub mod luminosity;
pub use luminosity::Luminosity;

use super::CelestialBody;
use crate::procedural_generation::gen_icon::circle_texture as circle_texture;
use crate::procedural_generation::data::StarData;

#[derive(Clone, Component)]
pub struct Star {
    pub spectral_type: String,
}

impl Star {
    pub fn get_bundle(
        star: Self, 
        mass: f64, 
        radius: f64, 
        luminosity: Luminosity, 
        x: f32, y: f32, 
        mut images: &mut ResMut<Assets<Image>>, 
    ) -> (CelestialBody, Self, Luminosity, Sprite, Transform) {
        let tex_size = (radius as u32 * 10).max(32);
        let custom_size = Vec2::splat((radius as f32 * 100.0).max(1000.0));

        (
            CelestialBody { name: "root".into(), mass, radius },
            star,
            luminosity,
            Sprite { 
                image: circle_texture(
                    tex_size, tex_size, &mut images,
                    255, 225, 30, 255
                ),
                custom_size: Some(custom_size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0)
        )
    }

    pub fn get_bundle_from_data(
        data: &StarData,
        x: f32, y: f32, 
        mut images: &mut ResMut<Assets<Image>>, 
    ) -> (CelestialBody, Self, Luminosity, Sprite, Transform) {

        let tex_size = (data.radius as u32 * 100000).min(32);
        let custom_size = Some(Vec2::splat((data.radius as f32 * 100.0).min(256.0)));

        dbg!((tex_size, custom_size));

        (
            CelestialBody { name: "root".into(), mass: data.mass, radius: data.radius },
            Star { spectral_type: data.spectral_type.clone() },
            Luminosity { 0: data.luminosity },
            Sprite { 
                image: circle_texture(
                    tex_size, tex_size, &mut images,
                    255, 225, 30, 255
                ),
                custom_size,
                ..default()
            },
            Transform::from_xyz(x, y, 0.0)
        )
    }
}