use crate::stellar_core::solar_system::{Star, Planet, Barycenter};

#[derive(Debug)]
pub enum CelestialBody {
    Star(Star),
    Planet(Planet),
    Barycenter(Barycenter)
}

