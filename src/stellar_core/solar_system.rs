use bevy::prelude::*;

pub mod orbit;
pub use orbit::Orbit;

pub mod barycenter;
pub use barycenter::Barycenter;

pub mod celestial_body;
pub use celestial_body::{Star, Planet, star::Luminosity, CelestialBody};

use crate::procedural_generation::data::GeneratorData;

pub struct SolarSystemPlugin;
impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_solar_system)
            .add_systems(Update, update_solar_system);
    }
}


fn setup_solar_system(
    mut commands: Commands, 
    mut images: ResMut<Assets<Image>>,
) {

    // commands.spawn(Star::get_bundle(
    //     Star {spectral_type: "M".into() }, 
    //     200.0,
    //     15.0,
    //     Luminosity {0: 1.0},
    //     0.0, 0.0, 
    //     &mut images,
    // ));

    // commands.spawn(Planet::get_bundle(
    //     Planet {}, 
    //     10.0, 
    //     1.0, 
    //     1000.0, -1000.0, 
    //     &mut images
    // ));


    let system = 
        crate::procedural_generation::gen_system::gen_system("eriku");

    for data in &system {
        
    }

    return;

    let mut stars = vec![];
    let mut planets = vec![];

    for x in &system {
        match x {
            GeneratorData::Barycenter(_) => {},
            GeneratorData::StarData(s) => stars.push(s),
            GeneratorData::PlanetData(p) => planets.push(p),
        }
    }

    for (_, star) in stars.into_iter().enumerate() {
        commands.spawn(Star::get_bundle(
            Star { spectral_type: star.spectral_type.clone().into() },
            star.mass,
            star.radius,
            Luminosity { 0: star.luminosity },
            0.0,
            0.0,
            &mut images
        ));
    }

    for (i, planet) in planets.into_iter().enumerate() {
        commands.spawn(Planet::get_bundle(
            Planet {}, 
            planet.mass, 
            planet.radius, 
            i as f32 * 1000.0, 
            1000.0, 
            &mut images
        ));
    }


}

fn update_solar_system(_bodies: Query<&mut Planet>, mut _gizmos: Gizmos) {

}

