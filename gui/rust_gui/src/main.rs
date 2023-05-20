use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;



use rust_core::*;
fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(LdtkPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -1500.0),
            ..Default::default()
        })
    .insert_resource(LevelSelection::Uid(0))
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        set_clear_color: SetClearColor::FromLevelBackground,
        ..Default::default()
    })
    .insert_resource(AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
    .add_event::<StateChangeEvent>()
    .add_startup_systems((
        setup,
    ))
    .add_systems((
        spawn_wall_collision,
        spawn_ground_sensor,
        camera_fit_inside_current_level,
        ground_detection,
        update_on_ground.after(ground_detection),
        movement.after(update_on_ground),
        update_level_selection,
        animate,
        landed,
        animation_flip_axis,
        switch_animation.after(movement)
    ))
    .register_ldtk_entity::<PlayerBundle>("Player")
    .register_ldtk_int_cell::<WallBundle>(1)
    .register_ldtk_int_cell::<WallBundle>(3)
    .run();
}
