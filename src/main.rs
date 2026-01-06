mod math;
mod physics;

use macroquad::prelude::*;
use crate::math::vec2::Vec2;
use crate::physics::sat::Sat;
use crate::physics::bodies::rectangle::{self, Rectangle};
use crate::physics::world::World;

use std::rc::Rc;
use std::cell::RefCell;

#[macroquad::main("Test")]
async fn main() {
    let mut world = World::new();
    let rect = Rc::new(RefCell::new(Rectangle::new(50.0, 50.0, 100.0, 100.0)));
    world.push(rect.clone());

    loop {
        let dt = get_frame_time();
        world.step(dt); 
        clear_background(WHITE);

        let r = rect.borrow();
        draw_rectangle(
            r.get_pos().get_x() as f32,
            r.get_pos().get_y() as f32,
            r.get_size().get_x() as f32,
            r.get_size().get_y() as f32,
        BLACK
        );

        next_frame().await;
    }   
}
 