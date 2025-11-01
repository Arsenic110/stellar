/*
    Note to self - these are the kinds of things this generator should be able to make:

    "Earth-Like-World / Land Rivers - A terran like planet with land, rivers and clouds. Earth",
    "Ice World - Ice planet, with some water lakes, wind and clouds. Hoth, MicroTech",
    "Terran Dry - A mars-like rocky planet, close to its star, dried out of any water. Mars",
    "Islands - Water planets covered in islands. Scarif",
    "No atmosphere - Moons or planets not protected by atmosphere. Mercury, Pluto",
    "Gas Giant I - A cold planet, outside the frost line. Jupiter/Saturn",
    "Gas Giant II - A cold planet, outside the frost line, variation. Uranus/Neptune",
    "Lava World - A protoplanet, perhaps too close to a star. Vulcan, Janssen/55 Cancri E",
    "Sulfur World - A world more similar to hell than reality. Venus"
*/

use crate::stellar_core::solar_system::Orbit;
use crate::stellar_core::solar_system::Planet;

use rand_distr::{Distribution, Normal};

pub const EARTH_RADIUS: f64 = 6.371e6;
pub const EARTH_MASS: f64 = 5.972e24;
pub const EARTH_GRAVITY: f64 = 9.7803267715;
pub const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;
pub const G: f64 = 6.6743015e-11;


pub fn generate_planet(earth_mass: f64, density: f64, solar_flux: f64, magnetic_field: f64,
    orbit: Orbit) -> Planet {

    //in meters
    let radius = f64::powf((3.0 * earth_mass * EARTH_MASS) / (4.0 * core::f64::consts::PI * density), 1.0 / 3.0);

    //in Gs
    let surface_gravity = (G as f64 * earth_mass * EARTH_MASS) / radius.powf(2.0) / EARTH_GRAVITY;

    //in m/s
    let escape_velocity = ((2.0 * G as f64 * earth_mass * EARTH_MASS) / radius).sqrt();

    let temp_base = 278.0 * solar_flux.sqrt();
    let mean_mol_weight = 28.97;
    let gas_retention_factor = 
        escape_velocity / ((3.0 * 1.380649e-23 * temp_base / (mean_mol_weight * 1.66053906660e-27)).sqrt());

    //we wanna distribute the possible atmospheres normally, to get some more interesting generation
    let nrm = Normal::new(earth_mass, 2.1).unwrap();
    let v: f64 = nrm.sample(&mut rand::rng()).abs();

    //use gas retention to modulate atmosphere retention (0.0 to 1.0 scale)
    let retention_efficiency = (gas_retention_factor / 10.0).clamp(0.0, 1.0);

    //solar flux and magnetic field affect atmospheric erosion or shielding
    let erosion_factor = (solar_flux - magnetic_field).max(0.0);
    let erosion_penalty = erosion_factor * (1.0 - retention_efficiency);

    //atmos modifier represents net gain/loss potential for atmosphere
    let atmos_modifier = ((v * retention_efficiency) - erosion_penalty).clamp(0.0, 5.0);

    //final atmospheric pressure in atm
    let atmos_pressure = atmos_modifier;

    //temperature estimate
    let greenhouse_effect = atmos_modifier * 33.0; //33K is Earth's greenhouse contribution
    let temp = temp_base + greenhouse_effect;

    //albedo: depends on clouds, surface type, etc.
    let albedo = (0.1 + 0.2 * (1.0 - magnetic_field).clamp(0.0, 1.0)) * (1.0 - 0.2 * atmos_pressure.clamp(0.0, 5.0));
    let _equilibrium_temp = ((solar_flux * (1.0 - albedo)) / (4.0 * STEFAN_BOLTZMANN)).powf(0.25);

    //solar day length: just increase with size
    //let solar_day_length = 24.0 * (radius).sqrt();

    //magnetic field strength vs core and rotation
    //let rotation_speed = 1.0 / solar_day_length;

    let tectonic_activity = match magnetic_field + normalize(temp, 200.0, 500.0) {
        -1.0..0.2 => (1, "Dormant"),
        0.2..0.4 => (2, "Barely Active"),
        0.4..0.6 => (3, "Weakly Active"),
        0.6..=1.0 => (4, "Moderately Active"),
        1.0..1.2 => (5, "Strongly Active"),
        1.2..1.4 => (6, "Unstable"),
        1.4..10.0 => (7, "Permanent Resurfacing"),
        _ => (0, "None")
    };

    //habitability score: crude, composite metric
    let habitability = {
        // Pressure score – values above 2 atm get capped.
        let pressure_score = (atmos_pressure / 2.0).min(1.0);

        // Temperature score – a linear fall‑off of 100 K from Earth’s mean.
        let temp_diff = (temp - 288.0).abs();
        let temp_score = (1.0 - (temp_diff / 100.0).min(1.0)).max(0.0);

        // Magnetic field – anything > 1 is still 1.
        let magnet_score = magnetic_field.min(1.0).max(0.0);

        // Tectonic activity – hard‑coded weights.
        let tectonics_score = match tectonic_activity {
            (x, _) => 1.0 - (x - 4).max(0) as f64 / 7.0
        };

        // The final habitability metric is the mean of the four scores.
        (pressure_score + temp_score + magnet_score + tectonics_score) / 4.0
    };

    //composition estimation (based on density)
    let composition = match density {
        d if d < 3000.0 && earth_mass >= 10.0 => "gas giant",
        d if d < 3000.0 && earth_mass < 10.0 => "ice world",
        d if d < 5500.0 => "rocky with volatile-rich crust",
        _ => "rocky with metallic core",
    };

    let p = Planet { 
        mass: earth_mass, 
        density: density, 
        radius: radius / 1000.0, 
        surface_gravity: surface_gravity, 
        atmos_pressure: atmos_pressure, 
        surface_temperature: temp, 
        atmosphere_composition: vec![(composition.to_string(), 1.0)], 
        magnetic_field_strength: magnetic_field, 
        tectonic_activity: tectonic_activity.1.to_string(), 
        habitability: habitability,
        orbit
    };

    p
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    //clamp to 0–1
    let v = (value - min).max(0.0).min(max - min);
    v / (max - min)
}