use bevy::prelude::*;

use crate::{StateChangeEvent, AnimationController};
use crate::entities::collisions::components::GroundDetection;
use crate::entities::player::components::Player;
use bevy_rapier2d::prelude::*;


pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection, &mut AnimationController, Entity), With<Player>>,
    mut state_change : EventWriter<StateChangeEvent>
) {
    for (mut velocity, ground_detection, mut anim, entity) in &mut query {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;


        
        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground) {
            velocity.linvel.y = 500.;
            anim.switch_state(1);
            state_change.send(StateChangeEvent(entity));

        }
    }
}

pub fn landed(
    mut query: Query<(Changed<GroundDetection>,&GroundDetection, &mut AnimationController, Entity), With<Player>>,
    mut state_change : EventWriter<StateChangeEvent>
){
    for (landed,ground, mut anim, entity) in &mut query {
        if landed && anim.state != 0 && ground.on_ground{
            anim.switch_state(0);
            state_change.send(StateChangeEvent(entity));
        }
    }
}