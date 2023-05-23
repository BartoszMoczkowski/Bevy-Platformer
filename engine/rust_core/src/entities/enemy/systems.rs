use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;

use crate::components::{Enemy, EnemyTextureData};
use crate::{StateChangeEvent, AnimationController};
use crate::entities::collisions::components::GroundDetection;
use crate::entities::player::components::Player;
use bevy_rapier2d::prelude::*;


pub fn spawn_enemy(
    mut query : Query<(Added<EntityInstance>,&mut AnimationController,&mut Handle<TextureAtlas>,&mut TextureAtlasSprite), With<Enemy>>,
    data : Res<EnemyTextureData>
){

    for (added, mut anim_contr, mut ta_handle, mut tas) in &mut query{
        if added{
            anim_contr.state = data.anim_contr.state.clone();
            anim_contr.frame = data.anim_contr.frame.clone();
            anim_contr.possible_animations = data.anim_contr.possible_animations.clone();
            anim_contr.state_transitions = data.anim_contr.state_transitions.clone();

            *ta_handle = data.ssb.texture_atlas.clone();
            *tas = TextureAtlasSprite::new(0);
        }
    }
}