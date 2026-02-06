use super::*;
use crate::stellar_core::solar_system::Barycenter;

pub enum GeneratorData {
    StarData(StarData),
    PlanetData(PlanetData),
    Barycenter(Barycenter)
}

impl GeneratorData {
    pub fn print(self: &Self) {
        match self {
            GeneratorData::StarData(_) => println!("Star\n"),
            GeneratorData::PlanetData(_) => println!("Planet\n"),
            GeneratorData::Barycenter(_) => println!("Barycenter\n"),
        }
    }
}