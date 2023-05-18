use bevy::prelude::*;
use std::collections::HashMap;
use bevy_ecs_ldtk::prelude::*;
#[derive(Component)]
pub struct Animation{
    pub texture_atlas_handle : Handle<TextureAtlas>,
    pub current_frame : usize,
    pub first_frame : usize,
    pub last_frame : usize,
    pub repeating : bool
}


#[derive(Component)]
pub struct AnimationController{
    possible_animations : Vec<Animation>,
    current_animation : usize,
    animation_rules : HashMap<usize,Vec<usize>>
}

