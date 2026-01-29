use bevy::prelude::*;
use crate::stellar_core;

mod thruster;
mod path;

use core::f32::consts::PI as PI;
use thruster::EngineFlame as EngineFlame;
use path::ShipPath as ShipPath;

use stellar_core::solar_system::Mass;

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                thruster::ThrusterPlugin,
                ShipPath,
            ))
            .add_systems(Startup, setup_ship)
            .add_systems(Update, update_ship)
            .add_systems(Update, ship_controls)
            ;
    }
}

#[derive(Component, Debug)]
pub struct Ship {
    pub velocity: Vec2,
    pub angular: f32,
    pub future_path: Vec<Vec2>,
}

impl Ship {
    pub fn new() -> Self {
        Ship { 
            velocity: Vec2 {x: 0.0, y: -15.1 }, 
            angular: 0.0,
            future_path: Vec::new()
        }
    }
}

fn setup_ship(mut commands: Commands, asset_server : Res<AssetServer>) {
    //load textures
    let ship_image: Handle<Image> = asset_server.load("ship2.png");
    let engine_flame_image: Handle<Image> = asset_server.load("engine_flame.png");

    //assemble the ship entity
    let _ship = commands.spawn((
        Ship::new(),
        Sprite { image: ship_image, custom_size: Some(Vec2::splat(10.)), ..default() },
        Transform::from_xyz(1000.0, 100.0, 1.0),
    ))
    .with_child( //the engine flame is a child because it allows custom placement of the plume
        EngineFlame::get_bundle(&engine_flame_image, 0, //main engine
            Transform::from_xyz(-6.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(1.5 * PI))
            .with_scale(Vec3 { x: 0.8, y: 4.0, z: 1.0 })
    ))
    .with_child(
        EngineFlame::get_bundle(&engine_flame_image, 1, //port 
            Transform::from_xyz(2.0, 3.0, 0.0)
            .with_rotation(Quat::from_rotation_z(0.5 * PI + 0.3))
            .with_scale(Vec3 { x: 0.4, y: 0.8, z: 1.0 })
    ))
    .with_child(
        EngineFlame::get_bundle(&engine_flame_image, 2, //starboard
            Transform::from_xyz(2.0, -3.0, 0.0)
            .with_rotation(Quat::from_rotation_z(0.5 * PI - 0.3))
            .with_scale(Vec3 { x: 0.4, y: 0.8, z: 1.0 })
    ))
    
    .id()
    ;

    //commands.entity(ship).insert((ShipPath::new(5, &path_pip)));

}

fn update_ship_position(mut ship_query: Query<(&mut Ship, &mut Transform)>) {
    let Ok((ship, mut transform)) = ship_query.get_single_mut()
        else {return};

    transform.translation = 
        Vec3 {
            x: transform.translation.x + ship.velocity.x, 
            y: transform.translation.y + ship.velocity.y,
            z: transform.translation.z,
        };
}

//process gravity for the ship
fn update_ship(
    mut ship_query: Query<(&mut stellar_core::ship::Ship, &mut Transform)>, 
    bodies_query: Query<(&stellar_core::solar_system::planet::Planet, &Transform), Without<stellar_core::ship::Ship>>,
    stars_query: Query<(&stellar_core::solar_system::celestial_body::Star, &Transform), Without<stellar_core::ship::Ship>>,
    bodies: Query<(&Mass, &Transform)>
) {
    //unpack and error handle the tuple
    let Ok((mut ship, mut transform)) = ship_query.get_single_mut() 
        else {return};

    let mut points: Vec<Vec2> = Vec::new();
    let mut current_point = transform.translation.xy();
    let mut current_velocity = ship.velocity;

    let path_length = 200;

    for i in 0..path_length {
        // Calculate the new velocity based on gravitational attraction
        let new_velocity = 
            stellar_core::navigation::calculate_acceleration(
                &current_point, &bodies_query.iter().collect(), &stars_query.iter().collect()
            )
            + current_velocity; // Add it to the current velocity

        //on the first run, update the ship values.
        if i == 0 {
            transform.translation.x += new_velocity.x;
            transform.translation.y += new_velocity.y;

            ship.velocity = new_velocity;
        }

        //update velocity and points
        current_velocity = new_velocity;
        current_point += new_velocity;
        //push the position to the vec
        points.push(current_point);
    }
    transform.rotation *= Quat::from_rotation_z(ship.angular);
    ship.future_path = points;
}

fn ship_controls(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ship_query: Query<(&mut Ship, &mut Transform)>,
    mut engines: Query<&mut EngineFlame, Without<Ship>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>
) {
    let Ok((mut ship, transform)) = ship_query.get_single_mut() else { return };
    let Ok(window) = q_windows.get_single() else { return };

    let mut button_pressed = false;
    let mut toggle_engine = |id: i32, state| {
        button_pressed = true;
        for mut e in engines.iter_mut() {
            if e.id == id {
                e.active = state;
            }
        }
    };

    if mouse_buttons.pressed(MouseButton::Right) {
        toggle_engine(1, true);
        toggle_engine(2, true);

        ship.angular *= 0.95;
        ship.velocity *= 0.95;
    }

    if mouse_buttons.pressed(MouseButton::Left) {

        //unpack this 'safely' incase user does something like click then drag mouse outside of the window.
        let Some(cursor_pos) = window.cursor_position() else { return };

        toggle_engine(0, true);

        let world_pos = 
            transform.translation.xy() + cursor_pos - window.size() / 2.0;

        let velocity_modifier = Vec2 { 
            x: (world_pos.x - transform.translation.x), 
            y: -(world_pos.y - transform.translation.y) 
        } / 1.0e3;

        ship.velocity += velocity_modifier;

        if ship.velocity.length_squared() > 0.0 {
            let nrm = velocity_modifier.normalize();
            let target_angle = nrm.y.atan2(nrm.x); // x and y swapped for standard atan2
        
            // Extract the current angle from the transform's rotation
            let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        
            // Compute shortest angle difference
            let mut angle_diff = target_angle - current_angle;

            if angle_diff > PI {
                angle_diff -= 2.0 * PI;
            } 
            else if angle_diff < -PI {
                angle_diff += 2.0 * PI;
            }
        
            // Apply damped turning speed
            ship.angular = angle_diff * 0.1;
        }
    }

    if mouse_buttons.just_pressed(MouseButton::Middle) {
        dbg!(transform.translation);
    }

    if keyboard.pressed(KeyCode::KeyQ) {
        ship.angular += 0.002;

        toggle_engine(1, true);
    }

    if keyboard.pressed(KeyCode::KeyE) {
        ship.angular -= 0.002;

        toggle_engine(2, true);
    }

    if !button_pressed {
        for mut e in engines.iter_mut() {
                e.active = false;
        }
    }


}