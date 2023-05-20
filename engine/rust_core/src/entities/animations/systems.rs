use bevy::prelude::{*};
use bevy_rapier2d::prelude::*;

use crate::{entities::animations::components::*};

pub fn load_animation(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    path : &str,
    tile_size : Vec2,
    col : usize,
    rows : usize,
    first_sprite_index : usize,
    last_sprite_index : usize,
    repeating : bool
) -> Animation
{
    let textutre_handle = asset_server.load(path);
    let texture_atlas = TextureAtlas::from_grid(
        textutre_handle,
        tile_size, 
        col,
        rows,
        None,
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    Animation{
        name : path.to_string(),
        texture_atlas_handle : texture_atlas_handle,
        first_frame : first_sprite_index,
        last_frame : last_sprite_index,
        repeating : repeating

    }   
}
pub fn animate(
    mut query : Query<(&mut AnimationController, &mut TextureAtlasSprite)>,
    time : Res<Time>,
    mut anim_timer : ResMut<AnimationTimer>
){
    
    if anim_timer.0.tick(time.delta()).just_finished(){
        for (mut anim, mut tas) in &mut query{
            anim.frame = if anim.frame == anim.current_animation().last_frame{
                anim.current_animation().first_frame
            } else {
                anim.frame + 1
            };
            tas.index = anim.frame;
        }
    }
}
pub struct StateChangeEvent(pub Entity);

pub fn switch_animation(
    mut query : Query<(&AnimationController, &mut Handle<TextureAtlas>)>,
    ev_state_change : EventReader<StateChangeEvent>
){
    if ev_state_change.len() !=0{
        for (anim, mut tas_handle) in &mut query{
            let new_handle = anim.current_animation().texture_atlas_handle.clone_weak();
            *tas_handle = new_handle;
        }
    }
}
pub fn animation_flip_axis(
    mut query : Query<(&Velocity, &mut TextureAtlasSprite)>
){
    for (vel, mut tas) in &mut query{
        if vel.linvel.x > 0. {
            tas.flip_x = true;
        }else if vel.linvel.x < 0. {
            tas.flip_x = false;
        }
    }
}

