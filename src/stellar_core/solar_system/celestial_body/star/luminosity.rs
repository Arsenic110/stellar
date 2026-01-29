use bevy::prelude::*;

//to be used by lighting system & for calculating heat/radiation
#[derive(Component)]
pub struct Luminosity(pub f64);

impl std::ops::Deref for Luminosity {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Luminosity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}