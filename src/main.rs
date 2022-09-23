mod components;
mod systems;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use components::PlayerLeaf;
use systems::{camera_follower, leaf_force_decay, leaf_height_updater, wind_blow};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.58, 0.769, 0.945)))
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        //    .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(create_leaf)
        .add_startup_system(create_height_tracker)
        .add_system(wind_blow)
        .add_system(leaf_force_decay)
        .add_system(leaf_height_updater)
        .add_system(camera_follower)
        .run();
}

fn create_leaf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("leaf_static.png"),
            transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(3.)),
            ..Default::default()
        })
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(PlayerLeaf);
}

fn create_height_tracker(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Test Height",
            TextStyle {
                font: asset_server.load("fonts/Milky Coffee.otf"),
                font_size: 23.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    );
}
