use bevy::prelude::*;

pub mod star;
pub use star::Star;

pub mod planet;
pub use planet::Planet;

#[derive(Component)]
pub struct CelestialBody {
    pub name: String,
    pub mass: f64,
    pub radius: f64
}