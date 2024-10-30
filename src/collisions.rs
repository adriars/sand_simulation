use bevy::prelude::*;

use crate::{bounding_box::BoundingBox, sand_simulation::SandParticle};

const DAMPENING: f32 = 0.8;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, check_bounding_box_collision);
    }
}

// Check bounding box collision
fn check_bounding_box_collision(
    mut sand_particles: Query<(&mut Transform, &mut SandParticle), With<SandParticle>>,
    bounding_box: Query<&BoundingBox>
) {
    let bounding_box = bounding_box.get_single().unwrap();

    let half_bounds_size = (bounding_box.size / 2.0) as f32 - Vec3::new(1.0, 1.0, 1.0);

    for (mut sand_particle_transform, mut sand_particle) in &mut sand_particles {
        if sand_particle_transform.translation.x.abs() > half_bounds_size.x {
            sand_particle_transform.translation.x = half_bounds_size.x * sand_particle_transform.translation.x.signum();
            sand_particle.velocity *= -1.0 * DAMPENING;
        }

        if sand_particle_transform.translation.y.abs() > half_bounds_size.y {
            sand_particle_transform.translation.y = half_bounds_size.y * sand_particle_transform.translation.y.signum();
            sand_particle.velocity *= -1.0 * DAMPENING;
        }

        if sand_particle_transform.translation.z.abs() > half_bounds_size.z {
            sand_particle_transform.translation.z = half_bounds_size.z * sand_particle_transform.translation.z.signum();
            sand_particle.velocity *= -1.0 * DAMPENING;
        }
    }
}