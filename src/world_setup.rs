use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use std::f32::consts::PI;

pub struct WorldSetup;

impl Plugin for WorldSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_setup);
    }
}

fn world_setup(
    mut commands: Commands,
) {

// Light
commands.spawn(DirectionalLightBundle {
    transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
    directional_light: DirectionalLight {
        shadows_enabled: true,
        ..default()
    },
    cascade_shadow_config: CascadeShadowConfigBuilder {
        first_cascade_far_bound: 200.0,
        maximum_distance: 400.0,
        ..default()
    }
    .into(),
    ..default()
});
}