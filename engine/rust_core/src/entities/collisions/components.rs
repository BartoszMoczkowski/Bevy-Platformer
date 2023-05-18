use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use std::collections::HashSet;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider : Collider,
    pub rigid_body : RigidBody,
    pub velocity : Velocity,
    pub rotation_constraints : LockedAxes,
    pub gravity_scale : GravityScale,
    pub friction : Friction,
    pub density : ColliderMassProperties
}

impl From<&EntityInstance> for ColliderBundle{
    fn from(value: &EntityInstance) -> Self {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match value.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(8., 14.),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default()
        }
        
    }
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}