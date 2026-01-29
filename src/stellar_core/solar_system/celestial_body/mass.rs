use bevy::prelude::*;

#[derive(Component)]
pub struct Mass(pub f64);

impl Mass {
    fn new(mass: f64) -> Mass {
        Mass {0: mass}
    }
}

//impl deref so you can refer to it as *mass instead of mass.0
impl std::ops::Deref for Mass {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Mass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}