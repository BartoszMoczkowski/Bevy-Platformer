use bevy::{prelude::*};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
#[derive(Component, Clone, Default, PartialEq)]
pub struct Animation{
    pub name : String,
    pub texture_atlas_handle : Handle<TextureAtlas>,
    pub first_frame : usize,
    pub last_frame : usize,
    pub repeating : bool
}
#[derive(Serialize, Deserialize)]
pub struct AnimationData{
    pub name : String,
    pub path : String,
    pub tile_size : Vec2,
    pub columns : usize,
    pub rows : usize,
    pub first_index : usize,
    pub last_index : usize,
    pub repeating : bool
}

#[derive(Serialize, Deserialize)]
pub struct EntityAnimationData{
    pub animations : HashMap<usize,AnimationData>,
    pub transitions : HashMap<usize,Vec<usize>>,
    pub default_state : usize
}


type State = usize;

#[derive(Component, Clone, Default)]
pub struct AnimationController{
    pub state : State,
    pub state_transitions : HashMap<State, Vec<State>>,
    pub possible_animations : HashMap<State, Animation>,
    pub frame : usize
}
#[derive (Resource)]
pub struct AnimationTimer(pub Timer);


impl AnimationController {
    pub fn current_animation(&self) -> &Animation{
        &self.possible_animations[&self.state]
    }

    pub fn switch_state(&mut self, state : State){
        if !self.state_transitions[&self.state].contains(&state){
            return;
        }
        self.state = state;
        self.frame = 0;
    }
}