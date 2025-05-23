use bevy::prelude::*;
use crate::stellar_core;

#[derive(Component)]
pub struct EngineFlame {
    pub id: i32,
    pub active: bool,
}

impl EngineFlame {
    pub fn new(engine_flame: &Handle<Image>, id: i32, transform: Transform) -> (EngineFlame, Sprite, Transform) {
        (
            EngineFlame { id, active: false },
            Sprite { image: engine_flame.clone(), custom_size: Some(Vec2::splat(6.0)), ..default() },
            transform
        )
    }

    pub fn update_thrusters(mut thruster_query: Query<(&mut Transform, &EngineFlame)>) {
        for (_, (mut transform, engine_flame)) in thruster_query.iter_mut().enumerate() {
            if engine_flame.active == true {
                transform.scale.y = stellar_core::lerp(transform.scale.y, 4.0 + rand::random_range(-1.0..1.0), 4.0);
            }
            else {
                transform.scale.y = stellar_core::lerp(transform.scale.y, 0.0, 4.0);
            }
        }
    }
}

