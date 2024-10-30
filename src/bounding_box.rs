use bevy::prelude::*;

pub struct BoundingBoxPlugin;

impl Plugin for BoundingBoxPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_bounding_box)
        .add_systems(Update, (render_bounding_box, control_bounding_box));
    }
}

#[derive(Component)]
pub struct BoundingBox {
    pub size: f32,
    min_size: f32,
    max_size: f32
}

fn setup_bounding_box(
    mut commands: Commands
) {
    commands.spawn(BoundingBox {size: 25.0, min_size: 5.0, max_size: 45.0});
}

// Render Bounding Box using gizmos
fn render_bounding_box(
    mut gizmos: Gizmos,
    query: Query<&BoundingBox>
) {

    let bounding_box = query.get_single().unwrap();

    gizmos.cuboid(
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(bounding_box.size)),
        Color::WHITE,
    );
}

fn control_bounding_box(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut BoundingBox>
) {

    let mut bounding_box = query.get_single_mut().unwrap();

    if keys.pressed(KeyCode::ArrowUp) && bounding_box.size <= bounding_box.max_size {
        bounding_box.size += 0.1;

    }

    if keys.pressed(KeyCode::ArrowDown) && bounding_box.size >= bounding_box.min_size {
        bounding_box.size -= 0.1;
    }
}