use bevy::prelude::*;

#[derive(Component)]
pub struct Radius(pub f64);

impl Radius {
    pub fn new(radius: f64) -> Radius {
        Radius {0: radius}
    }
}

//impl deref so you can refer to it as *radius instead of radius.0
impl std::ops::Deref for Radius {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Radius {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}