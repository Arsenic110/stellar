use core::f32;

use bevy::prelude::*;
use super::celestial_body::CelestialBody;

pub const G: f32 = 6.6743015e-11;

//calculate the total acceleration at a given position from a vec of celestial bodies.
pub fn calculate_acceleration(position: &Vec2, bodies: &Vec<(&CelestialBody, &bevy::prelude::Transform)>) -> Vec2 {

    //start with zero accel
    let mut accel = Vec2::new(0.0,0.0);

    //iterate through each body, adding the acceleration togethher.
    for body in bodies {
        accel += acceleration(
            &body.1.translation.xy(), 
            position, 
            1.0 * body.0.mass as f32, 
            0.5 * body.0.radius as f32);
    }

    return accel;
}

//modified newton's. Ignores mass of one of the objects, and adds a repulsive force 
pub fn acceleration(pos1: &Vec2, pos2: &Vec2, mass: f32, radius: f32) -> Vec2 {
    let delta_pos = pos1 - pos2;

    //true distance is split into two calculations  since we want to check for zero
    let distance_squared = delta_pos.length_squared();
    if distance_squared == 0.0 { 
        return Vec2::splat(0.0); //avoid division by zero
    }

    let distance = distance_squared.sqrt();
    //calculate magnitude + incorporate repulsive force calculation
    let acceleration_magnitude = G * mass * (distance_squared - radius * radius) / (distance_squared * distance_squared);

    let direction = delta_pos / distance;
    //cap it to at minimum -0.1
    let acceleration = direction * f32::max(-0.1, acceleration_magnitude);

    return acceleration;
}