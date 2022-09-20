use bevy::prelude::*;

use crate::generation::Building;

#[derive(Component)]
pub struct Player {
    pub destination: Building,
}

#[derive(Component)]
pub struct Follower;

#[derive(Component)]
pub struct Bystander {
    pub destination: Vec3,
    pub destination_building: Building,
    pub focus: f32,
}

#[derive(Component)]
pub struct Spawner {
    pub current_count: u32,
}

#[derive(Component)]
pub struct Entrance {
    pub building_name: String,
}
