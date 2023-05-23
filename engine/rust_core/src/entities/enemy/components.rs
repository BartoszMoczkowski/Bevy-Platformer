use bevy::{prelude::*, ecs::system::SystemState};
use bevy_ecs_ldtk::prelude::*;
use std::{collections::HashMap, fs::File};
use crate::{entities::{collisions::components::{ColliderBundle, GroundDetection}, animations::{components::{AnimationController}, systems::load_animation}}, EntityAnimationData, generate_animation_components};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;

#[derive(Clone, Default, Bundle)]
pub struct EnemyBundle {
    #[bundle]
    pub sprite_bundle : SpriteSheetBundle,
    pub animation_controller : AnimationController,
    #[bundle]
    pub collider_bundle : ColliderBundle,
    pub enemy : Enemy,
    pub ground_detection : GroundDetection,
    entity_instance : EntityInstance

}

#[derive(Resource,Clone)]
pub struct EnemyTextureData{
    pub ssb : SpriteSheetBundle,
    pub anim_contr : AnimationController
}
impl FromWorld for EnemyTextureData {
    fn from_world(world: &mut World) -> Self {
        
        let mut sys_state : SystemState<(
            Option<Res<AssetServer>>,
            Option<ResMut<Assets<TextureAtlas>>>
        )> = SystemState::new(world);

        let (op_asset_server, op_texture_atlases) = sys_state.get_mut(world);

        let asset_server = &op_asset_server.unwrap();
        let texture_atlases = &mut op_texture_atlases.unwrap();
        let (ssb, anim_contr) = generate_animation_components(
            "assets/sprites/enemy_anims.yaml".to_string(), 
            asset_server, 
            texture_atlases
        );

        EnemyTextureData { ssb: ssb, anim_contr: anim_contr }
    
    }
}

impl LdtkEntity for EnemyBundle{
    fn bundle_entity(
            entity_instance: &EntityInstance,
            _layer_instance: &LayerInstance,
            _tileset: Option<&Handle<Image>>,
            _tileset_definition: Option<&TilesetDefinition>,
            asset_server: &AssetServer,
            texture_atlases: &mut Assets<TextureAtlas>,
        ) -> Self {
        

        
        EnemyBundle { 
            collider_bundle: ColliderBundle::from(entity_instance),
            enemy : Enemy {},
            ground_detection: GroundDetection { on_ground: true },
            entity_instance: EntityInstance::from(entity_instance),
            ..Default::default()
        }
    }
}