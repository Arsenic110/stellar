use bevy::prelude::*;

use crate::stellar_core::solar_system::Orbit;

use crate::procedural_generation::{self, gen_star as gen};

#[derive(Clone, Component)]
pub struct Star {
    pub mass: f64,
    pub radius: f64,
    pub luminosity: f64,
    pub temperature: f64,
    pub lifespan: f64,
    pub spectral_type: String,
    pub orbit: Orbit,
}

impl Default for Star {
    fn default() -> Self {
        Star {
            mass: 1.0, radius: 1.0, luminosity: 1.0, temperature: 1.0, lifespan: 1.0, 
            spectral_type: ".".to_string(), orbit: Orbit::default(),
        }
    }
}

impl std::fmt::Debug for Star {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!("{} class | M: {:.5} R: {:.5} L: {:.5} T: {:.5}", 
            self.spectral_type,
            self.mass, 
            self.radius, 
            self.luminosity,
            self.temperature
        ).as_str())
    }
}

impl Star {
    pub fn new(mass: f64, age: f64, metallicity: f64) -> Star {
        gen::generate_star(mass, age, metallicity)
    }

    pub fn get_bundle(
        star: Self, x: f32, y: f32, mut images: &mut ResMut<Assets<Image>>
    ) -> (Self, Sprite, Transform) {
        let radius = star.radius;
        let tex_size = (radius as u32 * 100).max(16);

        dbg!((tex_size, radius));

        (
            star,
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