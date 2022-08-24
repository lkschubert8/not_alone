use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

enum Locations {
    BusStop,
    ShirosDumplings,
}

pub struct BystanderInit {
    pub fill_color: Color,
    pub stroke_color: Color,
    pub side_count: usize,
    pub start_location: Vec3,
    pub focus: f32,
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
    BystanderInit {
        fill_color,
        stroke_color,
        side_count,
        start_location: Vec3::new(x, y, z),
        focus: rng.gen_range(0.1..1.5),
    }
}
