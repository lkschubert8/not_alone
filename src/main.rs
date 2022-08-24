//! Renders a 2D scene containing a single, moving sprite.
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod components;
mod generation;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use components::*;
use generation::generate_bystander;
use systems::bystander_movement;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(bystander_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct Moveable;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(10.0),
        ..shapes::RegularPolygon::default()
    };
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Moveable);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("map.png"),
        transform: Transform::from_xyz(1920. / 2., 1080. / 2., 0.),
        ..default()
    });
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 5.0),
            },
            Transform::from_xyz(0., 0., 4.),
        ))
        .insert(Moveable {});

    // Add a bystander to the scene
    for _ in 1..1000 {
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
                heading: 0.,
                focus: bystander.focus,
            })
            .insert(Collider);
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut sprite_position: Query<&mut Transform, With<Moveable>>,
) {
    let speed = 100.0;
    for mut transform in &mut sprite_position {
        if keys.pressed(KeyCode::Up) {
            transform.translation.y += speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::Down) {
            transform.translation.y -= speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::Left) {
            transform.translation.x -= speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::Right) {
            transform.translation.x += speed * time.delta_seconds();
        }
    }
}
