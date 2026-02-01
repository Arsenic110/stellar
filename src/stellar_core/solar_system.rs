use bevy::prelude::*;

pub mod orbit;
pub use orbit::Orbit;

pub mod planet;
pub use planet::Planet;

pub mod celestial_body;
pub use celestial_body::{*, star::Luminosity};

pub mod barycenter;
pub use barycenter::Barycenter;

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

    commands.spawn(Star::get_bundle(
        Star {spectral_type: "M".into() }, 
        Mass::new(200.0),
        Radius::new(15.0),
        Luminosity {0: 1.0},
        0.0, 
        0.0, 
        &mut images,
    ));


    // let system = crate::procedural_generation::gen_system::gen_system("eriku");

    // let mut stars = vec![];
    // let mut planets = vec![];

    // for x in &system {
    //     match x {
    //         CelestialBody::Barycenter(_) => {},
    //         CelestialBody::Star(s) => stars.push(s),
    //         CelestialBody::Planet(p) => planets.push(p),
    //     }
    // }

    // for (i, star) in stars.into_iter().enumerate() {
    //     commands.spawn(Star::get_bundle(
    //         star.clone(), 
    //         i as f32 * 1000.0, 
    //         10.0, 
    //         &mut images
    //     ));
    // }

    // for (i, planet) in planets.into_iter().enumerate() {
    //     commands.spawn(Planet::get_bundle(
    //         planet.clone(), 
    //         i as f32 * 1000.0, 
    //         1000.0, 
    //         &mut images
    //     ));
    // }


}

fn update_solar_system(_bodies: Query<&mut Planet>, mut _gizmos: Gizmos) {

}

