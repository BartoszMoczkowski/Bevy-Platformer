use bevy::prelude::{*};
use bevy_rapier2d::prelude::*;

use std::{fs::File, collections::HashMap};

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

pub fn generate_animation_components(path : String, asset_server: &AssetServer, texture_atlases: &mut Assets<TextureAtlas>) -> (SpriteSheetBundle, AnimationController){
    let f = File::open(path).expect("Cannot open the player anim data file");
    let player_anim_data : EntityAnimationData = serde_yaml::from_reader(f).expect("Cannot deserialize player anim data");

    let mut anims = HashMap::new();
    for (state,anim_data) in player_anim_data.animations{
        let anim = load_animation(
            asset_server, 
            texture_atlases,
            &anim_data.path, 
            anim_data.tile_size, 
            anim_data.columns, 
            anim_data.rows, 
            anim_data.first_index, 
            anim_data.last_index, 
            anim_data.repeating
        );

        anims.insert(state, anim);
    }

    let texture_atlas_handle = anims[&0].texture_atlas_handle.clone();
    let anim_contr = AnimationController{
        state : 0,
        state_transitions : player_anim_data.transitions,
        possible_animations : anims,
        frame : 0
    };

    let ssb = SpriteSheetBundle{
        texture_atlas : texture_atlas_handle,
        sprite : TextureAtlasSprite::new(0),
        ..Default::default()
    };

    (ssb, anim_contr)

}




//systems
pub fn animate(
    mut query : Query<(&mut AnimationController, &mut TextureAtlasSprite)>,
    time : Res<Time>,
    mut anim_timer : ResMut<AnimationTimer>
){
    
    if anim_timer.0.tick(time.delta()).just_finished(){
        for (mut anim, mut tas) in &mut query{
            if anim.possible_animations.len() == 0{
                return;
            }
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

