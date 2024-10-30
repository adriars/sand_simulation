use bevy::prelude::*;
use rand::Rng;

use crate::bounding_box::BoundingBox;

const SAND_PARTICLE_SIZE: f32 = 1.0;

#[derive(Component)]
pub struct SandParticle {
    pub velocity: Vec3
}

pub struct SandSimulation;

impl Plugin for SandSimulation {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_sand_particles);
    }
}

// Spawn sand particles
fn spawn_sand_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    bounding_box_query: Query<&BoundingBox>
) {
    
    let mut rng = rand::thread_rng();

    let bounding_box = bounding_box_query.get_single().unwrap();

    for _ in 0..100 {
        commands.spawn((
            SandParticle {
                velocity: Vec3::new(0.0, 0.0, 0.0)},
                PbrBundle {
                    mesh: meshes.add(Sphere::new(SAND_PARTICLE_SIZE)),
                    material: materials.add(Color::from(Hsla::hsl(0.0, 1.0, 1.0))),
                    transform: Transform::from_translation(Vec3::new(rng.gen_range(-bounding_box.size / 2.0 .. bounding_box.size / 2.0), rng.gen_range(-bounding_box.size / 2.0 .. bounding_box.size / 2.0), rng.gen_range(-bounding_box.size / 2.0 .. bounding_box.size / 2.0))),
                    ..Default::default()
                },
        ));
        };
    }