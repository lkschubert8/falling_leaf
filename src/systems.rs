use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Mug, PlayerLeaf};

pub fn wind_blow(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    mut leaf_query: Query<(&mut Transform, &mut ExternalForce, &mut PlayerLeaf), With<PlayerLeaf>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.get_primary_mut().unwrap();
    let (camera, camera_transform) = camera_query.single();
    let leaf_query_res = leaf_query.get_single_mut();
    if leaf_query_res.is_err() {
        return;
    }

    let (leaf_transform, mut leaf_force, mut player_leaf) = leaf_query_res.unwrap();

    if btn.just_pressed(MouseButton::Left) && player_leaf.gusts_left > 0 {
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
            leaf_force.force += difference.normalize() * 600.0;
            player_leaf.gusts_left -= 1;
        } else {
            // cursor is not inside the window
        }
    }
}

pub fn leaf_force_decay(
    time: Res<Time>,
    mut leaf_query: Query<&mut ExternalForce, With<PlayerLeaf>>,
) {
    let leaf_force_res = leaf_query.get_single_mut();
    if leaf_force_res.is_err() {
        return;
    }
    let mut leaf_force = leaf_force_res.unwrap();
    leaf_force.force = leaf_force
        .force
        .lerp(Vec2 { x: 0.0, y: 0.0 }, 10.0 * time.delta_seconds());
}

pub fn leaf_blow_updater(
    mut height_widget_query: Query<&mut Text>,
    leaf_query: Query<&PlayerLeaf>,
) {
    let mut height_widget_text = height_widget_query.single_mut();
    let player_leaf_res = leaf_query.get_single();
    if player_leaf_res.is_err() {
        return;
    }
    let player_leaf = player_leaf_res.unwrap();
    height_widget_text.sections[0].value = format!("{} Blows Left", player_leaf.gusts_left);
}

pub fn camera_follower(
    time: Res<Time>,
    leaf_query: Query<&mut Transform, With<PlayerLeaf>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<PlayerLeaf>)>,
) {
    let leaf_transform_res = leaf_query.get_single();
    if leaf_transform_res.is_err() {
        return;
    }
    let leaf_transform = leaf_transform_res.unwrap();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = camera_transform.translation.lerp(
        leaf_transform.translation.truncate().extend(15.0),
        5.0 * time.delta_seconds(),
    );
    // println!(
    //     "Player Translation {},{},{}",
    //     leaf_transform.translation.x, leaf_transform.translation.y, leaf_transform.translation.z
    // );
    // println!(
    //     "Camera Translation {},{},{}",
    //     camera_transform.translation.x,
    //     camera_transform.translation.y,
    //     camera_transform.translation.z
    // );
}

pub fn god_mode_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let speed = 500.;
    if keys.pressed(KeyCode::Up) {
        camera_transform.translation.y += speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Down) {
        camera_transform.translation.y -= speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Left) {
        camera_transform.translation.x -= speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Right) {
        camera_transform.translation.x += speed * time.delta_seconds();
    }
}

pub fn god_mode_mouse_location(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.get_primary_mut().unwrap();
    let (camera, camera_transform) = camera_query.single();

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
            println!("World position = ({}, {})", world_pos.x, world_pos.y);
        }
    }
}

pub fn tea_in_mug_system(
    mut commands: Commands,

    rapier_context: Res<RapierContext>,
    query_player_leaf: Query<Entity, With<PlayerLeaf>>,
    mut query_mug: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut Mug,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let player_res = query_player_leaf.get_single();
    if player_res.is_err() {
        return;
    }
    let player = player_res.unwrap();
    let (mug, mut sprite, texture_atlas_handle, mut mug_component) = query_mug.single_mut();
    if rapier_context.intersection_pair(player, mug) == Some(true) {
        mug_component.leafs_in_mug += 1;
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        commands.entity(player).despawn();
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("leaf_static.png"),
                transform: Transform::from_xyz(-1394.0, -450.0, 10.),
                ..Default::default()
            })
            .insert(ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(10.0, 10.0))
            .insert(PlayerLeaf { gusts_left: 1000 });
    }
}
