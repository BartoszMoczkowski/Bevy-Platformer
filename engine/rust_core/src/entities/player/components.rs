use bevy::{prelude::*};
use bevy_ecs_ldtk::prelude::*;
use std::{collections::HashMap, fs::File};
use crate::{entities::{collisions::components::{ColliderBundle, GroundDetection}, animations::{components::{AnimationController}, systems::load_animation}}, EntityAnimationData};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player{}

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_bundle : SpriteSheetBundle,
    pub animation_controller : AnimationController,
    #[bundle]
    pub collider_bundle : ColliderBundle,
    pub player : Player,
    pub wordly : Worldly,
    pub ground_detection : GroundDetection,
    entity_instance : EntityInstance

}

impl LdtkEntity for PlayerBundle{
    fn bundle_entity(
            entity_instance: &EntityInstance,
            _layer_instance: &LayerInstance,
            _tileset: Option<&Handle<Image>>,
            _tileset_definition: Option<&TilesetDefinition>,
            asset_server: &AssetServer,
            texture_atlases: &mut Assets<TextureAtlas>,
        ) -> Self {
        
        let f = File::open("assets/sprites/player_anims.yaml").expect("Cannot open the player anim data file");
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


        PlayerBundle { 
            sprite_bundle: ssb,
            collider_bundle: ColliderBundle::from(entity_instance),
            player: Player {},
            animation_controller : anim_contr,
            wordly: Worldly::from_entity_info(entity_instance),
            ground_detection: GroundDetection { on_ground: true },
            entity_instance: EntityInstance::from(entity_instance)
        }
    }
}