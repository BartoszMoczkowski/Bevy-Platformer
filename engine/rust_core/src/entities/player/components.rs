use bevy::{prelude::*};
use bevy_ecs_ldtk::prelude::*;
use std::{collections::HashMap, fs::File};
use crate::{entities::{collisions::components::{ColliderBundle, GroundDetection}, animations::{components::{AnimationController}, systems::load_animation}}, EntityAnimationData, generate_animation_components};

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
        

        let (ssb, anim_contr) = generate_animation_components(
            "assets/sprites/player_anims.yaml".to_string(), 
            asset_server, 
            texture_atlases
        );
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