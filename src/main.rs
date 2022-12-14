mod components;
mod systems;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::entity,
    prelude::*,
};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use components::{GustText, Mug, PlayerLeaf};
use systems::{
    camera_follower, god_mode_mouse_location, god_mode_movement, leaf_force_decay,
    leaf_gust_updater, tea_in_mug_system, wind_blow,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    TitleScreen,
    Game,
    BrewedTea,
    GodMode,
}

fn main() {
    // std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap()).unwrap();
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.203, 0.328, 0.320))) // 52, 84, 82
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy game".to_string(), // ToDo
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(DebugLinesPlugin::default())
        .add_state(GameState::TitleScreen)
        //Title State
        .add_system_set(SystemSet::on_enter(GameState::TitleScreen).with_system(title_screen_setup))
        .add_system_set(SystemSet::on_update(GameState::TitleScreen).with_system(space_start_game))
        .add_system_set(
            SystemSet::on_exit(GameState::TitleScreen).with_system(title_screen_teardown),
        )
        // Game States
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(create_leaf)
                .with_system(create_world)
                .with_system(create_height_tracker),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(wind_blow)
                .with_system(leaf_force_decay)
                .with_system(leaf_gust_updater)
                .with_system(tea_in_mug_system)
                .with_system(camera_follower)
                .with_system(god_mode_toggle),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GodMode)
                .with_system(god_mode_movement)
                .with_system(god_mode_toggle)
                .with_system(god_mode_mouse_location),
        )
        .run();
}

pub fn space_start_game(keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        app_state.set(GameState::Game).unwrap()
    }
}

pub fn title_screen_teardown(
    mut commands: Commands,
    everything_but_camera: Query<Entity, Without<Camera>>,
) {
    for entity in everything_but_camera.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn title_screen_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Brewing",
            TextStyle {
                font: asset_server.load("./fonts/Milky Coffee.otf"),
                font_size: 45.0,
                color: Color::rgb_u8(236, 111, 28),
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::CENTER), // Set the style of the TextBundle itself.
    );
    commands.spawn_bundle(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "press space to begin",
            TextStyle {
                font: asset_server.load("./fonts/Milky Coffee.otf"),
                font_size: 20.0,
                color: Color::rgb_u8(236, 111, 28),
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::CENTER), // Set the style of the TextBundle itself.
    );
}

pub fn god_mode_toggle(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::Grave) {
        match app_state.current() {
            GameState::Game => app_state.push(GameState::GodMode).unwrap(),
            GameState::GodMode => app_state.pop().unwrap(),
            _ => todo!(),
        }
        keys.reset(KeyCode::Grave);
    }
}
fn create_leaf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("./leaf_static.png"),
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

fn create_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("./bg2.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    });

    commands
        .spawn()
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -1400.0, -505.0, 2.0,
        )))
        .insert(Collider::cuboid(60.0, 15.0));

    // Creating mug
    let texture_handle = asset_server.load("./mug.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(1151.8059, -464.01813, 2.),
            ..Default::default()
        })
        .insert(Collider::cuboid(20.0, 30.0))
        .insert(Sensor)
        .insert(Mug {
            leafs_in_mug: 0,
            full_brew: 4,
        });
}

fn create_height_tracker(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Gusts Left",
                TextStyle {
                    font: asset_server.load("./fonts/Milky Coffee.otf"),
                    font_size: 45.0,
                    color: Color::rgb_u8(236, 111, 28),
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
        )
        .insert(GustText {});
}
