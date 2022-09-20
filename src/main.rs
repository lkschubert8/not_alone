//! Renders a 2D scene containing a single, moving sprite.
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod components;
mod generation;
mod systems;

use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::parry::shape::Cuboid;
use bevy_rapier2d::prelude::{
    Collider, GravityScale, NoUserData, RapierConfiguration, RapierDebugRenderPlugin,
    RapierPhysicsPlugin, Restitution, RigidBody, Velocity,
};
use components::*;
use generation::{generate_bystander, get_buildings, player_init, Building};
use rand::Rng;
use systems::{
    bystander_movement, camera_tracker, follower_system, handle_player_arrival_at_destination,
    sprite_movement,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    Game,
    Win,
    Lose,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DebugLinesPlugin::default())
        .add_state(AppState::Menu)
        //Main Screen Systems
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(main_menu_setup))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(main_menu_space_to_start))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(main_menu_cleanup))
        // Game Systems
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(sprite_movement)
                .with_system(bystander_movement)
                .with_system(camera_tracker)
                .with_system(follower_system)
                .with_system(handle_player_arrival_at_destination),
        )
        .run();
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("main_menu.png"),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(0.7, 0.7, 0.7)),
        ..default()
    });
}

fn main_menu_space_to_start(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.pressed(KeyCode::Space) {
        app_state.set(AppState::Game).unwrap();
    }
}

fn main_menu_cleanup(mut commands: Commands, mut entity: Query<Entity>) {
    for entity in entity.iter_mut() {
        commands.entity(entity).despawn();
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(10.0),
        ..shapes::RegularPolygon::default()
    };
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("map.png"),
        transform: Transform::from_xyz(1920. / 2., 1080. / 2., 0.),
        ..default()
    });
    build_walls(&mut commands);
    create_player(&mut commands, shape.clone(), asset_server);

    create_bystanders(&mut commands);
    create_buildings(&mut commands);

    create_follower(commands, shape);
}

fn create_buildings(commands: &mut Commands) {
    get_buildings()
        .iter()
        .for_each(|building| building.add_to_scene(commands));
}

fn create_follower(mut commands: Commands, shape: RegularPolygon) {
    let mut rng = rand::thread_rng();
    let eight_byte_range = 0.0..1.0;
    let fill_color = Color::rgb(
        rng.gen_range(eight_byte_range.clone()),
        rng.gen_range(eight_byte_range.clone()),
        rng.gen_range(eight_byte_range.clone()),
    );
    let stroke_color = Color::rgb(
        rng.gen_range(eight_byte_range.clone()),
        rng.gen_range(eight_byte_range.clone()),
        rng.gen_range(eight_byte_range.clone()),
    );
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(fill_color),
                outline_mode: StrokeMode::new(stroke_color, 5.0),
            },
            Transform::from_xyz(500., 500., 4.),
        ))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(Follower)
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.2,
        });
}

fn build_walls(commands: &mut Commands) {
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1920.0, 5.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            1920.0 / 2.0,
            -5.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1920.0, 5.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            1920.0 / 2.0,
            1080.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 1080.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            1080.0 / 2.0,
            0.0,
        )));
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 1080.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            1920.0,
            1080.0 / 2.0,
            0.0,
        )));
}

fn create_bystanders(commands: &mut Commands) {
    (1..5).for_each(|_| {
        let bystander = generate_bystander();
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::RegularPolygon {
                    sides: bystander.side_count,
                    feature: shapes::RegularPolygonFeature::Radius(10.0),
                    ..shapes::RegularPolygon::default()
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(bystander.fill_color),
                    outline_mode: StrokeMode::new(bystander.stroke_color, 5.0),
                },
                Transform::from_translation(bystander.start_location),
            ))
            .insert(Bystander {
                destination: bystander.destination,
                destination_building: bystander.destination_building,
                focus: bystander.focus,
            })
            .insert(RigidBody::Dynamic)
            .insert(Restitution::coefficient(0.01))
            .insert(GravityScale(0.0))
            .insert(Collider::cuboid(10.0, 10.0))
            .insert(Velocity {
                linvel: Vec2::new(1.0, 2.0),
                angvel: 0.2,
            });
    });
}

fn create_player(commands: &mut Commands, shape: RegularPolygon, asset_server: Res<AssetServer>) {
    let player_init = player_init();
    commands.spawn_bundle(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!(
                "Get To {}, Don't Let Them Follow You!",
                player_init.destination.name
            ),
            TextStyle {
                font: asset_server.load("fonts/Akira Expanded Demo.otf"),
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
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 5.0),
            },
            Transform::from_xyz(50., 10., 4.),
        ))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(Player {
            destination: player_init.destination,
        })
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.2,
        });
}
