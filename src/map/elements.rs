use crate::{
    animation::AnimatedSprite,
    map::MapElementHydrated,
    metadata::MapElementMeta,
    physics::{collisions::CollisionWorld, KinematicBody},
    player::{input::PlayerInputs, PlayerIdx, MAX_PLAYERS},
    prelude::*,
};

mod player_spawner;
mod sproinger;

pub struct MapElementsPlugin;

impl Plugin for MapElementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player_spawner::PlayerSpawnerPlugin)
            .add_plugin(sproinger::SproingerPlugin);
    }
}
