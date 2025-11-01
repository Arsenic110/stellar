use crate::stellar_core::solar_system::Orbit;

#[derive(Debug)]
pub struct Barycenter {
    orbit: Orbit
}

impl Default for Barycenter {
    fn default() -> Self {
        Barycenter { orbit: Orbit::default() }
    }
}