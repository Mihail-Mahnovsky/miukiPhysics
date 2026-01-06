mod math;
mod physics;

use crate::math::vec2::{self, Vec2};
use crate::physics::sat::Sat;

fn main() {
    let square1 = Sat::new(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]);

    let square2 = Sat::new(vec![
        Vec2::new(0.5, 0.5),
        Vec2::new(1.5, 0.5),
        Vec2::new(1.5, 1.5),
        Vec2::new(0.5, 1.5),
    ]);

    println!("Collision? {}", square1.check_collision(&square2));
}
