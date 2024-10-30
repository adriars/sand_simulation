use bevy::prelude::*;

use crate::sand_simulation::SandParticle;

const GRAVITY: f32 = 9.81;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}

// Apply gravity force to sand particles
fn apply_gravity(time: Res<Time>, mut sand_particles: Query<(&mut Transform, &mut SandParticle), With<SandParticle>>) {

    for (mut sand_particle_transform, mut sand_particle) in &mut sand_particles {
        sand_particle.velocity += Vec3::new(0.0, -1.0, 0.0) * GRAVITY * time.delta_seconds();
        sand_particle_transform.translation += sand_particle.velocity * time.delta_seconds();
    }

}