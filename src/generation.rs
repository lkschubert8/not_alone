use bevy::{prelude::*, sprite::Rect};
use bevy_rapier2d::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::components::Entrance;

pub struct PlayerInit {
    pub origin: Building,
    pub destination: Building,
}

pub fn player_init() -> PlayerInit {
    let mut rng = rand::thread_rng();
    let binding = get_buildings();
    let places: Vec<&Building> = binding.choose_multiple(&mut rng, 2).collect();
    PlayerInit {
        origin: places[0].clone(),
        destination: places[1].clone(),
    }
}
pub struct BystanderInit {
    pub fill_color: Color,
    pub stroke_color: Color,
    pub side_count: usize,
    pub start_location: Vec3,
    pub focus: f32,
    pub destination: Vec3,
    pub destination_building: Building,
}
pub fn generate_bystander() -> BystanderInit {
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
    let side_count = rng.gen_range(3..8);
    let x = rng.gen_range(10.0..1910.);
    let y = rng.gen_range(10.0..1070.);
    let z = rng.gen_range(0.0..1.0);

    let binding = get_buildings();
    let destination_building = binding.choose(&mut rng).unwrap().clone();
    // TODO make the entry to the building
    let entrance = destination_building.entrance.unwrap();
    let destination = Vec3::new(
        (entrance.min.x + entrance.max.x) / 2.,
        (entrance.min.y + entrance.max.y) / 2.,
        0.,
    );
    BystanderInit {
        fill_color,
        stroke_color,
        side_count,
        start_location: Vec3::new(x, y, z),
        focus: rng.gen_range(1.0..4.),
        destination,
        destination_building,
    }
}

#[derive(Debug, Clone, Component)]
pub struct Building {
    pub name: String,
    pub bounds: Rect,
    pub entrance: Option<Rect>,
}

impl Building {
    pub fn add_to_scene(&self, commands: &mut Commands) {
        commands
            .spawn()
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                self.bounds.width() / 2.,
                self.bounds.height() / 2.,
            ))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                (self.bounds.max.x + self.bounds.min.x) / 2.,
                (self.bounds.max.y + self.bounds.min.y) / 2.,
                0.0,
            )));
        commands
            .spawn()
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                self.entrance.unwrap().width() / 2.,
                self.entrance.unwrap().height() / 2.,
            ))
            .insert(Sensor)
            .insert(Entrance {
                building_name: self.name.clone(),
            })
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                (self.entrance.unwrap().max.x + self.entrance.unwrap().min.x) / 2.,
                (self.entrance.unwrap().max.y + self.entrance.unwrap().min.y) / 2.,
                0.0,
            )));
    }
}

pub fn get_buildings() -> Vec<Building> {
    vec![
        // Building {
        //     name: "Bus".to_string(),
        //     bounds: Rect {
        //         min: Vec2::new(50., 0.),
        //         max: Vec2::new(100.0, 215.0),
        //     },
        //     entrance: None,
        // },
        Building {
            name: "Bus Stop".to_string(),
            bounds: Rect {
                min: Vec2::new(210., 0.),
                max: Vec2::new(717., 90.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(489., 1080. - 989.),
                max: Vec2::new(551., 1080.0 - 969.0),
            }),
        },
        Building {
            name: "Conner's Ramen".to_string(),
            bounds: Rect {
                min: Vec2::new(460., 250.),
                max: Vec2::new(732., 1080.0 - 640.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(515., 1080. - 840.),
                max: Vec2::new(543., 1080.0 - 832.0),
            }),
        },
        Building {
            name: "Pop and Pop Shop".to_string(),
            bounds: Rect {
                min: Vec2::new(841., 1080. - 831.),
                max: Vec2::new(1191., 1080.0 - 601.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(903., 1080. - 841.),
                max: Vec2::new(994., 1080.0 - 832.0),
            }),
        },
        Building {
            name: "The Gun Show".to_string(),
            bounds: Rect {
                min: Vec2::new(1266., 1080. - 831.),
                max: Vec2::new(1494., 1080.0 - 675.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(1290., 1080. - 840.),
                max: Vec2::new(1310., 1080.0 - 832.0),
            }),
        },
        Building {
            name: "Strip Mall".to_string(),
            bounds: Rect {
                min: Vec2::new(1689., 1080. - 1080.),
                max: Vec2::new(1920., 1080.0 - 649.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(1662., 1080. - 906.),
                max: Vec2::new(1688., 1080.0 - 916.0),
            }),
        },
        Building {
            name: "XYZ Mart".to_string(),
            bounds: Rect {
                min: Vec2::new(1701., 1080. - 603.),
                max: Vec2::new(1920., 1080.0 - 0.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(1679., 1080. - 161.),
                max: Vec2::new(1700., 1080.0 - 97.0),
            }),
        },
        Building {
            name: "Bub's Pub".to_string(),
            bounds: Rect {
                min: Vec2::new(1094., 1080. - 426.),
                max: Vec2::new(1463., 1080.0 - 175.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(1215., 1080. - 449.),
                max: Vec2::new(1248., 1080.0 - 427.0),
            }),
        },
        Building {
            name: "The Tower".to_string(),
            bounds: Rect {
                min: Vec2::new(712., 1080. - 426.),
                max: Vec2::new(951., 1080.0 - 0.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(792., 1080. - 440.),
                max: Vec2::new(856., 1080.0 - 427.0),
            }),
        },
        Building {
            name: "Knight's Web Cafe".to_string(),
            bounds: Rect {
                min: Vec2::new(459., 1080. - 430.),
                max: Vec2::new(606., 1080.0 - 190.0),
            },
            entrance: Some(Rect {
                min: Vec2::new(512., 1080. - 438.),
                max: Vec2::new(537., 1080.0 - 430.0),
            }),
        },
    ]
}
