use core::f64;

use bevy::prelude::*;
use super::solar_system::star::Star;
use super::solar_system::planet::Planet;
use crate::stellar_utils::unit_conversion::*;

pub const G: f64 = 6.6743015e-11;
const SOL_MASS: f64 = 2e14;

//calculate the total acceleration at a given position from a vec of celestial bodies.
pub fn calculate_acceleration(
    position: &Vec2, bodies: &Vec<(&Planet, &bevy::prelude::Transform)>, 
    stars: &Vec<(&Star, &bevy::prelude::Transform)>
) -> Vec2 {

    //start with zero accel
    let mut accel = Vec2::new(0.0,0.0);

    //iterate through each body, adding the acceleration together.
    for body in bodies {
        accel += acceleration(
            &body.1.translation.xy(), 
            position, 
            earths(body.0.mass) as f64, 
            0.5 * body.0.radius as f64);
    }

    for star in stars {
        accel += acceleration(
            &star.1.translation.xy(), 
            position, 
            earths(star.0.mass) as f64, 
            2.5 * star.0.radius as f64)
    }

    accel
}

//modified newton's. Ignores mass of one of the objects, and adds a repulsive force when close by
pub fn acceleration(pos1: &Vec2, pos2: &Vec2, mass: f64, radius: f64) -> Vec2 {
    let delta_pos = pos1 - pos2;
    let mass = mass * 1e-10;

    //true distance is split into two calculations  since we want to check for zero
    let distance_squared = delta_pos.length_squared() as f64;
    if distance_squared == 0.0 { 
        return Vec2::splat(0.0); //avoid division by zero
    }

    let distance = distance_squared.sqrt() as f64;
    //calculate magnitude + incorporate repulsive force calculation
    let soft = radius * 0.1;
    let acceleration_magnitude = G * mass * (distance_squared - soft * soft) / (distance_squared * distance_squared);

    let direction = delta_pos / distance as f32;
    //cap it to at minimum -0.1
    let acceleration = direction * f64::max(-0.1, acceleration_magnitude) as f32;

    return acceleration;
}