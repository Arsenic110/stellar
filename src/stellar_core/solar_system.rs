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

    let mut i = 0;
    for data in &system {

        if i > 2 && false {
            break;
        }

        match data {
            GeneratorData::Barycenter(_) => {},
            GeneratorData::StarData(data) => {
                commands.spawn(Star::get_bundle_from_data(
                    data, 0.0, 0.0, &mut images
                ));
            },
            GeneratorData::PlanetData(data) => {
                commands.spawn(Planet::get_bundle_from_data(
                    data, i as f32 * 100.0, 100.0, &mut images
                ));
            },
        }

        i += 1;
    }

    println!("After loop print");
}

fn update_solar_system(_bodies: Query<&mut Planet>, mut _gizmos: Gizmos) {

}

