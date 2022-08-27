use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Follower;

#[derive(Component)]
pub struct Bystander {
    pub destination: Vec3,
    pub focus: f32,
}

enum FollowerReason {
    DroppedWallet,
    DroppedPhone,
    Crush,
    RecognizedFromSchool,
    Murderer,
    TryingToReachYouRegardingCarsExtendedWarranty,
}

enum Locations {
    BusStop,
    ShirosDumplings,
}

#[derive(Component)]
struct Spawner {
    current_count: u32,
}
