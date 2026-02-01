use bevy::prelude::*;

pub mod mass;
pub use mass::Mass;

pub mod radius;
pub use radius::Radius;

pub mod star;
pub use star::Star;

#[derive(Component)]
pub struct CelestialBody;