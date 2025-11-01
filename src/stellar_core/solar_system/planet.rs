use bevy::prelude::*;

use crate::stellar_core::solar_system::Orbit;

use crate::procedural_generation::{self, gen_planet as gen};

#[derive(Clone, Component)]
pub struct Planet {
    pub mass: f64,
    pub density: f64,
    pub radius: f64,
    pub surface_gravity: f64,
    pub atmos_pressure: f64,
    pub surface_temperature: f64,
    pub atmosphere_composition: Vec<(String, f64)>,
    pub magnetic_field_strength: f64,
    pub tectonic_activity: String,
    pub habitability: f64,
    pub orbit: Orbit,
}

impl Default for Planet {
    fn default() -> Self {
        Planet { 
            mass: 0.0, 
            density: 0.0, 
            radius: 0.0, 
            surface_gravity: 0.0, 
            atmos_pressure: 0.0, 
            surface_temperature: 0.0, 
            atmosphere_composition: vec![], 
            magnetic_field_strength: 0.0, 
            tectonic_activity: "".to_string(), 
            habitability: 0.0,
            orbit: Orbit::default()
        }
    }
}

impl std::fmt::Debug for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!("{:?} M(e): {:.3} D: {:.4}g/cm^3 R: {:.2}km Temp: {:.2}K", 
            self.orbit, 
            self.mass, 
            self.density / 1000.0, 
            self.radius, 
            self.surface_temperature,
        ).as_str())
    }
}

impl Planet {
    pub fn new(earth_mass: f64, density: f64, solar_flux: f64, magnetic_field: f64, orbit: Orbit) -> Planet {
        gen::generate_planet(earth_mass, density, solar_flux, magnetic_field, orbit)
    }

    pub fn get_bundle(
        planet: Self, x: f32, y: f32, images: &mut ResMut<Assets<Image>>
    ) -> (Self, Sprite, Transform) {
        let radius = planet.radius;
        let tex_size = (radius as u32 / 100).max(1);

        let sprite = Sprite {
            image: procedural_generation::gen_icon::image_to_handle(
                procedural_generation::gen_icon::render_icon(&planet, tex_size), 
                images
            ),
            custom_size: Some(Vec2::splat(radius as f32)),
            ..default()
        };

        (
            planet,
            sprite,
            Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(0.05))
        )
    }
}