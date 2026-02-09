use bevy::prelude::*;

use super::CelestialBody;
use crate::procedural_generation::gen_icon::circle_texture as circle_texture;
use crate::procedural_generation::data::PlanetData;

#[derive(Component)]
pub struct Planet {
}

impl Default for Planet {
    fn default() -> Self {
        Planet { 
        }
    }
}

impl Planet {
    pub fn get_bundle(
        planet: Self, 
        mass: f64, 
        radius: f64, 
        x: f32, y: f32, 
        mut images: &mut ResMut<Assets<Image>>
    ) -> (CelestialBody, Self, Sprite, Transform) {
        let pixel_size = radius as u32 * 10;
        let custom_size = Some(Vec2::splat(radius as f32 * 50.0));

        (
            CelestialBody { name: "planet".into(), mass, radius },
            planet,
            Sprite { 
                image: circle_texture(
                    pixel_size, pixel_size, &mut images,
                    0, 225, 255, 255
                ),
                custom_size,
                ..default()
            },
            Transform::from_xyz(x, y, 0.0)
        )
    }

    pub fn get_bundle_from_data(
        data: &PlanetData,
        x: f32, y: f32, 
        mut images: &mut ResMut<Assets<Image>>,
    ) -> (CelestialBody, Self, Sprite, Transform) {
        let pixel_size = (data.radius as u32 / 100).min(16);
        let custom_size = 
            Some(Vec2::splat((data.radius as f32 / 100.0).min(64.0)));

        dbg!((pixel_size, custom_size));

        (
            CelestialBody { name: "planet".into(), mass: data.mass, radius: data.radius },
            Planet {},
            Sprite { 
                image: circle_texture(
                    pixel_size, pixel_size, &mut images,
                    0, 225, 255, 255
                ),
                custom_size,
                ..default()
            },
            Transform::from_xyz(x, y, 0.0)
        )
    }
}