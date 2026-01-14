use bevy::{input::mouse::*, prelude::*};
use crate::stellar_core;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        //build plugin & add systems
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, update_chase_camera.run_if(any_with_component::<CamChase>))
            .add_systems(Update, update_free_camera.run_if(any_with_component::<CamFree>))
            .add_systems(Update, update_cam_type)
            .add_systems(Update, update_cam_zoom)
            ;
    }
}

//todo: overhaul with 3D cameras for spicy planet renders?
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d, 
        Camera { ..default() },
        CamChase
    ));
}

//chase camera
fn update_chase_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, With<CamChase>)>,
    ship_query: Query<&Transform, (With<stellar_core::ship::Ship>, Without<Camera2d>)>
) {
    let Ok(mut transform) =  camera_query.get_single_mut() else { return };
    let Ok(ship) = ship_query.get_single() else { return };

    //set camera position to ship position.
    transform.translation = ship.translation;

    //set ship scale to something relative so that it scales with zoom
    //ship_query.single_mut().scale = (transform.scale + 1.0) * 3.0;
}

//freecam function
fn update_free_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, With<CamFree>)>, 
    input: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>
) {
    let Ok(mut transform) = camera_query.get_single_mut() else { return };

    let mut direction = Vec3::ZERO;
    let zoom: f32 = transform.scale.y;

    if input.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }
    if input.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }
    if input.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
    if input.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }

    //mouse drag to move camera. cheap & easy solution - not always pixel-perfect
    for ev in evr_motion.read() {
        if buttons.pressed(MouseButton::Left) {
            direction -= Vec3 {x: ev.delta.x / 8.0, y: -ev.delta.y / 8.0, z: direction.z };
        }
    }

    //apply translation and scale
    transform.translation += direction * zoom * 5.0;
}

fn update_cam_type(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, Option<&CamChase>, Option<&CamFree>), With<Camera2d>>
) {
    if !input.just_released(KeyCode::Space) {
        return;
    }

    for (entity, c, _) in query.iter() {
        let mut ec = commands.entity(entity);
        match c {
            Some(_) => {ec.remove::<CamChase>().insert(CamFree);},
            None => {ec.remove::<CamFree>().insert(CamChase);}
        }   
    }
}

fn update_cam_zoom(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    let Ok(mut transform) = camera_query.get_single_mut() else {return};
    let mut zoom: f32 = transform.scale.y;

    let scroll: f32 = evr_scroll.read().map(|ev| ev.y).sum();

    if scroll != 0.0 {
        zoom = (zoom * (1.0 - scroll / 10.0)).clamp(0.1, 10.0);
    }

    //apply the scale to the camera
    transform.scale = Vec3 { x: zoom, y: zoom, z: zoom };
}

#[derive(Component)]
pub struct CamChase;

#[derive(Component)]
pub struct CamFree;