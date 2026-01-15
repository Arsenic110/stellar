//use bevy::prelude::*;

pub mod ship;
pub mod navigation;
pub mod solar_system;
pub mod camera;

//im not sure if refactoring the utilities into the mod here is right,
//but here they are!

//linear interpolation
pub fn lerp(start: f32, end: f32, speed: f32) -> f32 {
    (end - start) / speed + start
}

//sigmoid
pub fn sigmoid(z: f32) -> f32 {
    return 1.0 / (1.0 + (-z).exp());
}

//sigmoid derivative
pub fn sigmoid_derivative(z: f32) -> f32 {
    let sigmoid_x = 1.0 / (1.0 + (-z).exp());
    return 0.25 - (0.25 - sigmoid_x) * (0.25 - sigmoid_x);
}