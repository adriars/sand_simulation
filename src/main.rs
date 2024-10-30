mod sand_simulation;
mod world_setup;
mod free_camera;
mod bounding_box;
mod collisions;
mod gravity;

use bevy::prelude::*;
use bounding_box::BoundingBoxPlugin;
use collisions::CollisionsPlugin;
use gravity::GravityPlugin;
use sand_simulation::SandSimulation;
use world_setup::WorldSetup;
use free_camera::FreeCamera;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins((
            DefaultPlugins,
            WorldSetup,
            FreeCamera,
            BoundingBoxPlugin,
            CollisionsPlugin,
            GravityPlugin,
            SandSimulation
        ))
        .run();
}
