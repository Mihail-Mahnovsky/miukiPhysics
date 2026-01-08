use std::f64::consts::E;

use crate::physics::body::Body;
use crate::math::vec2::Vec2;
use crate::physics::physicsContext::PhysicsContext;
use crate::physics::rigetbody::RigetBody;
use crate::physics::transform::Transform;
use crate::Sat;

pub struct Rectangle {
    pub transform: Transform ,
    pub body: RigetBody ,
    pub hitbox : Sat,
}


impl Rectangle{

    pub fn new(x : f64, y : f64, width : f64,height : f64) -> Self {

        let hitbox = Sat::new(vec![ 
            Vec2::new(x, y),
            Vec2::new(x + width, y),
            Vec2::new(x + width, y + height),
            Vec2::new(x, y + height),
        ]);

        Self { transform: Transform::new(x, y, width, height), body: RigetBody::new(),hitbox :  hitbox}
    }
}

impl Body for Rectangle {
    fn update(&mut self, dt: f32,phys_context : &PhysicsContext) {
        let dt = dt as f64;

        self.body.update(&mut self.transform,phys_context);
        self.update_hitbox();
    }

    fn update_hitbox(&mut self) {
        let x = self.transform.pos.x;
        let y = self.transform.pos.y;
        let width = self.transform.size.x;
        let height = self.transform.size.y;

        self.hitbox.set_verticles(vec![
            Vec2::new(x, y),
            Vec2::new(x + width, y),
            Vec2::new(x + width, y + height),
            Vec2::new(x, y + height),
        ]);
    }
}
