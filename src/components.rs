use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Follower;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Bystander {
    pub heading: f32,
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
