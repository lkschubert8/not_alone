use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::{
    prelude::{QueryFilter, RapierContext, Velocity},
    rapier::{prelude::CollisionEvent, rayon::spawn},
};
use rand::Rng;

use crate::{
    components::{Bystander, Entrance, Follower, Player, Spawner},
    AppState,
};

pub fn bystander_movement(
    time: Res<Time>,

    mut bystander_query: Query<(&mut Transform, &Bystander), With<Bystander>>,
) {
    let mut thread_rng = rand::thread_rng();

    for (mut transform, bystander) in &mut bystander_query {
        let speed = thread_rng.gen_range(0.75..125.);
        let direction = thread_rng.gen_range(0.0..360.0);
        let destination = bystander.destination;
        let diff = destination - transform.translation;
        let heading = diff.y.atan2(diff.x) + (2. * PI);

        let smoothing_factor = bystander.focus;
        let actual_direction = ((heading * smoothing_factor) + direction) / (1. + smoothing_factor);
        let direction_in_radians = actual_direction.to_radians();
        let movement_y = speed * time.delta_seconds() * direction_in_radians.sin();
        let movement_x = speed * time.delta_seconds() * direction_in_radians.cos();
        transform.translation.y += movement_y;
        transform.translation.x += movement_x;
    }
}

pub fn camera_tracker(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera = camera_query.single_mut();
    let player = player_query.single();
    camera.translation = player.translation.clone();
}

pub fn follower_system(
    time: Res<Time>,

    mut follower_query: Query<(&mut Transform, &mut Velocity), With<Follower>>,
    player_query: Query<&mut Transform, (With<Player>, Without<Follower>)>,
    mut lines: ResMut<DebugLines>,
) {
    let mut thread_rng = rand::thread_rng();

    let (mut follower, mut velocity) = follower_query.single_mut();
    let player = player_query.single();
    let diff = follower.translation - player.translation;
    let heading = diff.y.atan2(diff.x) + (2. * PI);

    let speed = 100.0;
    let direction = thread_rng.gen_range(0.0..(2.0 * PI));
    let smoothing_factor = 2.5;
    let actual_direction = ((heading * smoothing_factor) + direction) / (1. + smoothing_factor);
    let movement_y = speed * time.delta_seconds() * actual_direction.sin();
    let movement_x = speed * time.delta_seconds() * actual_direction.cos();
    // println!(
    //     "Angle {}, X {}, Y {}",
    //     heading.to_degrees(),
    //     movement_x,
    //     movement_y
    // );
    follower.translation.y -= movement_y;
    follower.translation.x -= movement_x;
    velocity.linvel = velocity.linvel * 0.95;
    velocity.angvel = 0.0;
    //Follower heading line
    // lines.line(
    //     follower.translation,
    //     follower.translation + (Vec3::new(movement_x, movement_y, 0.) * 10.0),
    //     0.0,
    // );
    // //Line from follower to player
    // lines.line(player.translation, follower.translation, 0.0);
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
pub fn sprite_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let speed = 100.0;
    for (mut transform, mut velocity) in &mut sprite_position {
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
        velocity.linvel = velocity.linvel * 0.95;
        velocity.angvel = 0.0;
        // println!("({},{})", transform.translation.x, transform.translation.y);
    }
}

fn handle_bystanders_arriving_at_destination(
    rapier_context: Res<RapierContext>,
    query_entrances: Query<(Entity, &Entrance)>,
    query_bystanders: Query<(Entity, &Bystander)>,
    mut spawner_query: Query<&mut Spawner>,
    mut commands: Commands,
) {
    let mut spawner = spawner_query.single_mut();
    for (entrance, entrance_component) in query_entrances.iter() {
        for (bystander, bystander_component) in query_bystanders.iter() {
            if rapier_context.intersection_pair(entrance, bystander) == Some(true) {
                if entrance_component.building_name == bystander_component.destination_building.name
                {
                    commands.entity(bystander).despawn();
                    spawner.current_count -= 1;
                }
            }
        }
    }
}

pub fn handle_player_arrival_at_destination(
    rapier_context: Res<RapierContext>,
    query_entrances: Query<(Entity, &Entrance)>,
    query_player: Query<(Entity, &Player, &Transform)>,
    query_follower: Query<(Entity, &Follower, &Transform)>,
    mut app_state: ResMut<State<AppState>>,
) {
    let (player, player_component, player_transform) = query_player.single();
    let (follower, follower_component, follower_transform) = query_follower.single();
    for (entrance, entrance_component) in query_entrances.iter() {
        if rapier_context.intersection_pair(entrance, player) == Some(true) {
            if entrance_component.building_name == player_component.destination.name {
                println!("Player arrived at destination");
                // Check if player can be seen by the follower
                let follower_center = follower_transform.translation.truncate();
                let player_location = player_transform.translation.truncate();
                let ray_direction = (player_location - follower_center).normalize() * 3.0;
                let ray_origin = follower_center + (ray_direction * 8.0);
                if let Some((entity, _)) = rapier_context.cast_ray(
                    ray_origin,
                    ray_direction,
                    200.0,
                    true,
                    QueryFilter::default(),
                ) {
                    println!(
                        "Entity {:?} hit at point, player == {:?}, follower == {:?}",
                        entity, player, follower
                    );
                    if entity == player {
                        app_state.set(AppState::Lose).unwrap();
                    } else {
                        app_state.set(AppState::Win).unwrap();
                    }
                } else {
                    app_state.set(AppState::Win).unwrap();
                }
            }
        }
    }
}
