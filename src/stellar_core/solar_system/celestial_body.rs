use bevy::prelude::*;

pub mod mass;
pub use mass::Mass;

pub mod star;
pub use star::Star;

#[derive(Component)]
pub struct CelestialBody;