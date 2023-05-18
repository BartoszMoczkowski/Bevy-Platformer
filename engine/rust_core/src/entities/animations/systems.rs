use bevy::prelude::*;
use std::collections::HashMap;

use crate::entities::animations::components::*;

fn load_animation(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
        texture_atlas_handle : texture_atlas_handle,
        current_frame : first_sprite_index,
        first_frame : first_sprite_index,
        last_frame : last_sprite_index,
        repeating : repeating

    }   
}


pub fn create_animation_controller(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    tile_size : Vec2,
    col : usize,
    rows : usize,
){

}