use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalForce;

use crate::components::PlayerLeaf;

pub fn wind_blow(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    mut leaf_query: Query<(&mut Transform, &mut ExternalForce), With<PlayerLeaf>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.get_primary_mut().unwrap();
    let (camera, camera_transform) = camera_query.single();
    let (leaf_transform, mut leaf_force) = leaf_query.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            let difference = leaf_transform.translation.truncate() - world_pos;
            leaf_force.force += difference.normalize() * 1600.0;
        } else {
            // cursor is not inside the window
        }
    }
}

pub fn leaf_force_decay(
    time: Res<Time>,
    mut leaf_query: Query<&mut ExternalForce, With<PlayerLeaf>>,
) {
    let mut leaf_force = leaf_query.single_mut();
    leaf_force.force = leaf_force
        .force
        .lerp(Vec2 { x: 0.0, y: 0.0 }, 10.0 * time.delta_seconds());
}

pub fn leaf_height_updater(
    mut height_widget_query: Query<&mut Text>,
    leaf_query: Query<&mut Transform, With<PlayerLeaf>>,
) {
    let mut height_widget_text = height_widget_query.single_mut();
    let leaf_transform = leaf_query.single();
    height_widget_text.sections[0].value = format!(
        "{} Current Fall",
        (leaf_transform.translation.y / 20.0).round().abs()
    );
}
