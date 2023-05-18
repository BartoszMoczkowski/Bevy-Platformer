use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::{HashMap, HashSet};
use crate::entities::collisions::components::GroundDetection;
use crate::entities::player::components::Player;
use bevy_rapier2d::prelude::*;


pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection), With<Player>>,
) {
    for (mut velocity, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;



        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground) {
            velocity.linvel.y = 500.;
        }
    }
}
