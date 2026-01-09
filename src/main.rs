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
    let rect = Rc::new(RefCell::new(Rectangle::new(50.0, 50.0, 125.0, 100.0)));

    let rect1 = Rc::new(RefCell::new(Rectangle::new(120.0, 210.0, 100.0, 100.0)));
    world.push(rect.clone());
    world.push(rect1.clone());


    loop {
        let dt = get_frame_time();
        world.step(dt as f64); 
        clear_background(WHITE);

        let r = rect.borrow();
        draw_rectangle(
            r.transform.pos.x as f32,
            r.transform.pos.y as f32,
            r.transform.size.x as f32,
            r.transform.size.y as f32,
        BLACK
        );

        let r1 = rect1.borrow();
        draw_rectangle(
            r1.transform.pos.x as f32,
            r1.transform.pos.y as f32,
            r1.transform.size.x as f32,
            r1.transform.size.y as f32,
        BLACK
        );

        next_frame().await;
    }   
}
 