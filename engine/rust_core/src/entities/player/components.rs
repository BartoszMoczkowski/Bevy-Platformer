use bevy::{prelude::*, ecs::bundle};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::collisions::components::{ColliderBundle, GroundDetection};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player{}

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_bundle : SpriteSheetBundle,
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
            layer_instance: &LayerInstance,
            tileset: Option<&Handle<Image>>,
            tileset_definition: Option<&TilesetDefinition>,
            asset_server: &AssetServer,
            texture_atlases: &mut Assets<TextureAtlas>,
        ) -> Self {
        
        let texture_handle = asset_server.load("sprites/player1.png");
        let texture_atlas = 
            TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(24.0, 24.0),
                1,
                1,
                None,
                None
            );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let ssb = SpriteSheetBundle{
            texture_atlas : texture_atlas_handle,
            sprite : TextureAtlasSprite::new(0),
            ..Default::default()
        };


        PlayerBundle { 
            sprite_bundle: ssb,
            collider_bundle: ColliderBundle::from(entity_instance),
            player: Player {},
            wordly: Worldly::from_entity_info(entity_instance),
            ground_detection: GroundDetection { on_ground: true },
            entity_instance: EntityInstance::from(entity_instance)
        }
    }
}