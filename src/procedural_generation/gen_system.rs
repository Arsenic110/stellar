use rand::{rngs::SmallRng, SeedableRng, Rng};
use sha2::{Sha256, Digest};
use std::f64::consts::E;

use crate::stellar_core::solar_system::{
    Star,
    Planet,
    CelestialBody, 
    Barycenter
};

use crate::stellar_utils::unit_conversion::*;

use crate::stellar_core::solar_system::Orbit;
use crate::stellar_utils::MTree;

pub fn gen_system(seed: &str) -> MTree<CelestialBody> {
    //init our rng from the seed
    let mut rng: SmallRng = random_gen_from_string(seed);

    let _star_amount = match rng.random_range(0..=100) {
        0..=70 => 1,
        71..=95 => 2,
        96..=99 => 3,
        100 => 4,
        _ => 5
    };

    let star_amount = 1;

    let mut star_vec: Vec<Star> = vec![];
    for _ in 0..star_amount {
        let starmass = imf(&mut rng);
        let age_gy = rng.random_range(0.0..13.8);
        let metallicity = rng.random_range(0.0..=1.3);
        star_vec.push(Star::new(starmass, age_gy, metallicity));
    }

    let mut system_tree = match star_amount {
        1 => MTree::new(CelestialBody::Star(star_vec.pop().unwrap())),
        _ => MTree::new(CelestialBody::Barycenter(Barycenter::default()))
    };

    //todo: add binary barycenter logic here

    //this is the root barycenter of the entire system.
    let mut system_root = system_tree.root_handle();

    let starmass = imf(&mut rng);

    //the planetary system mass pool
    let pmass = sols(starmass) * 0.00001;

    //these parameters are fed into the mass distribution function
    let l_stable= rng.random_range(0.0..20.0);
    let z_unstable = rng.random_range(0.0..1.0);

    //TODO: in meters
    let system_size = rng.random_range(0.0..starmass);

    //running mass total
    let mut current_mass = 0.0;
    let mut current_distance = 0.0;

    while current_mass < pmass && current_distance < system_size * 10.0 {

        //move this planet some distance away from the previous
        current_distance = current_distance + rng.random_range(0.0..system_size);

        //sample the mass from our function and add this value to the running total
        let planet_mass = earths(
            planet_mass_function(current_distance, l_stable, z_unstable, 0.232, 4.313)
        );
        current_mass += planet_mass;

        let orbit = Orbit::new(
            current_distance,
            current_distance * random_eccentricity(&mut rng),
            rng.random_range(0.0..1.0));

        let mut planet_system = MTree::new(
            CelestialBody::Planet(Planet::new(
            to_earth(planet_mass / 1.0),
            rng.random_range(800.0..8000.0),
            solar_flux_function(current_distance, system_size) * 1.0,
            rng.random_range(0.0..1.0),
            orbit
        )));

        let moon_amount = (rng.random_range(0.0..=to_earth(planet_mass)) * 10.0).trunc() as u32;

        for _ in 0..moon_amount {
            let moon_mass = planet_mass * rng.random_range(0.001..0.01);
            current_mass += moon_mass;

            let root_val = planet_system.root_handle();
            let root_planet = match root_val.value() {
                CelestialBody::Planet(planet) => planet,
                _ => panic!()
            };
            
            let root_star = match system_root.value() {
                CelestialBody::Star(star) => star,
                _ => panic!()
            };

            let max_hill = max_moon_orbit(
                &mut rng, 
                root_star.mass, 
                planet_mass,
                root_planet.orbit.semi_major_axis());

            let moon_density = rng.random_range(500.0..8000.0);
            let roche_limit = 
                root_planet.radius * 
                (2.0 * (root_planet.density / moon_density)).powf(0.333);

            let moon_orbit = Orbit::new(
                max_hill.max(roche_limit), 
                max_hill.max(roche_limit) * random_eccentricity(&mut rng), 
                rng.random_range(0.0..1.0));

            planet_system.append(0,
                CelestialBody::Planet(Planet::new(
                to_earth(moon_mass),
                moon_density,
                solar_flux_function(current_distance, system_size) * 1.0,
                rng.random_range(0.0..0.1),
                moon_orbit
            )));
        }

        system_root.merge(planet_system);
    }

    system_tree
}

fn random_gen_from_string<R: SeedableRng<Seed = [u8; 32]>>(s: &str) -> R {

    let hash = Sha256::digest(s.as_bytes());
    let mut seed = [0 as u8; 32];
    seed.copy_from_slice(&hash);

    R::from_seed(seed)
}

///Sample planet masses at a distance from a star.
fn planet_mass_function(distance: f64, l_stable: f64, z_unstable: f64, big_a: f64, exponent: f64) -> f64 {
    let term1 = E.powf((l_stable.powf(distance.sin()) * (1.0 / (E.powf(z_unstable) * z_unstable).cos()) + z_unstable.powf(E).atan()).sin());
    let term2 = l_stable.powf(distance).cos() + E;

    let top = term1 * term2;
    let bottom = 2.0 * E + E.powf(distance);

    //positive result
    let p_res = top / bottom;

    let x = -distance;

    let term1 = E.powf((l_stable.powf(x.sin()) * (1.0 / (E.powf(z_unstable) * z_unstable).cos()) + z_unstable.powf(E).atan()).sin());
    let term2 = l_stable.powf(x).cos() + E;

    let top = term1 * term2;
    let bottom = 2.0 * E + E.powf(x);

    //negative result
    let n_res = top / bottom;

    let raw_res = p_res * n_res;

    //peak amplification (for gas giants etc)
    let middle_term = big_a * (1.0 / (1.0 + E.powf(-10.0 * raw_res)));
    let fixed_peaks = raw_res + middle_term + raw_res.powf(exponent);

    return fixed_peaks;
}

///Sample solar flux at a distance.
fn solar_flux_function(distance: f64, system_size: f64) -> f64 {
    E.powf(system_size * -1.0 * distance)
}

///Integrated Mass Function: IMF
fn imf<R: Rng>(rng: &mut R) -> f64 {
    let present_day = true;

    let seg_integral = |m1: f64, m2: f64, alpha: f64| -> f64 {
            if (alpha - 1.0).abs() < 1e-8 {
                (m2 / m1).ln()
            } 
            else {
                (m2.powf(1.0 - alpha) - m1.powf(1.0 - alpha)) / (1.0 - alpha)
            }
    };

    let sample_powerlaw = |u: f64, m1: f64, m2: f64, alpha: f64| -> f64 {
        if (alpha - 1.0).abs() < 1e-8 {
            m1 * (m2 / m1).powf(u)
        } 
        else {
            let p = 1.0 - alpha;
            ((u * (m2.powf(p) - m1.powf(p)) + m1.powf(p))).powf(1.0 / p)
        }
    };

        //segments: (m_min, m_max, alpha)
    let segments = [
        (0.08, 0.5, 1.3),
        (0.5, 120.0, 2.3),
    ];

    //get weights for each segment
    let weights: Vec<f64> = segments
        .iter()
        .map(|(m1, m2, a)| seg_integral(*m1, *m2, *a))
        .collect();
    let total_weight: f64 = weights.iter().sum();
    let weights: Vec<f64> = weights.iter().map(|w| w / total_weight).collect();

    loop {
        //choose a segment
        let u: f64 = rng.random();
        let mut acc = 0.0;
        let mut idx = 0;
        for (i, w) in weights.iter().enumerate() {
            acc += w;
            if u <= acc {
                idx = i;
                break;
            }
        }

        let (m1, m2, alpha) = segments[idx];
        let u_mass: f64 = rng.random();
        let mass = sample_powerlaw(u_mass, m1, m2, alpha);

        if present_day {
            //main-sequence lifetime in Gyr: t ~ 10 * m^-2.5
            let t_ms = 10.0 * mass.powf(-2.5);
            let survive_prob = (t_ms.min(10.0)) / 10.0;
            let u_life: f64 = rng.random();
            if u_life <= survive_prob {
                return mass;
            } else {
                continue; //resample until survivor
            }
        } else {
            return mass;
        }
    }
}

///Returns the amount of stable star arrangements for this number of stars
fn get_arrangement_num(stars: u32) -> u32 {
    match stars {
        1 => 1,
        2 => 1,
        _ => ((stars / 3) - (stars % 2)) / 2 + 1
    }
}

fn random_eccentricity<R: Rng>(rng: &mut R) -> f64 {
    let sigma = 0.9;
    let x: f64 = rng.sample(rand_distr::Normal::new(0.0, sigma).unwrap());
    let y: f64 = rng.sample(rand_distr::Normal::new(0.0, sigma).unwrap());
    (x.powi(2) + y.powi(2)).sqrt().clamp(0.0, 0.95)
}

///Hill radius in AU. This is the Sphere of Influence around an object
fn hill_radius(star_mass: f64, planet_mass: f64, semi_major: f64) -> f64 {
    semi_major * (planet_mass / (3.0 * star_mass)).powf(1.0 / 3.0)
}

///Max safe moon orbit in AU. Up to 50% of the Hill sphere radius/
fn max_moon_orbit<R: Rng>(rng: &mut R, star_mass: f64, planet_mass: f64, semi_major: f64) -> f64 {
    let hill = hill_radius(star_mass, planet_mass, semi_major);
    hill * rng.random_range(0.05..0.5)
}