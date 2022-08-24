use std::os::unix::thread;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use rand::Rng;

use crate::components::{Bystander, Collider};

pub fn bystander_movement(
    time: Res<Time>,

    mut bystander_query: Query<(&mut Transform, &Collider, &Bystander), With<Bystander>>,
) {
    let mut thread_rng = rand::thread_rng();

    for (mut transform, _, bystander) in &mut bystander_query {
        let speed = thread_rng.gen_range(0.0..100.);
        let direction = thread_rng.gen_range(0.0..360.0);
        let heading = bystander.heading;
        let smoothing_factor = bystander.focus;
        let actual_direction = ((heading * smoothing_factor) + direction) / (1. + smoothing_factor);
        let direction_in_radians = actual_direction.to_radians();
        let mut movement_y = speed * time.delta_seconds() * direction_in_radians.sin();
        let mut movement_x = speed * time.delta_seconds() * direction_in_radians.cos();
        // for (collider_transform, _, _) in bystander_query.iter() {
        //     let collision = collide(
        //         transform.translation,
        //         transform.scale.truncate(),
        //         collider_transform.translation,
        //         collider_transform.scale.truncate(),
        //     );
        //     if let Some(collision) = collision {
        //         match collision {
        //             Collision::Left => movement_x = 0.0_f32.max(movement_x),
        //             Collision::Right => movement_x = 0.0_f32.min(movement_x),
        //             Collision::Top => movement_y = 0.0_f32.min(movement_y),
        //             Collision::Bottom => movement_y = 0.0_f32.max(movement_y),
        //             _ => (),
        //         }
        //     }
        // }
        transform.translation.y += movement_y;
        transform.translation.x += movement_x;
    }
}
