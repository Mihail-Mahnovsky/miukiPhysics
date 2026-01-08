use crate::{math::vec2::Vec2, physics::{physicsContext::PhysicsContext, transform::{self, Transform}}};

#[derive(Clone)]
pub struct RigetBody{
    pub is_static : bool,
    pub is_on_floor : bool,
    pub velocity : Vec2,
}

impl RigetBody{
    pub fn new() -> Self{
        Self { 
            is_static: false, 
            is_on_floor : false,
            velocity : Vec2::default(),
        }
    }

    pub fn update(&mut self, dt : f64,transform : &mut Transform,phys_context : &PhysicsContext){
        if self.is_static{
            return;
        }

        if !self.is_on_floor {
            let g = phys_context.gravity.y * 100.0;
            self.velocity.y +=  g * dt;
        }

        transform.pos += self.velocity * dt;
    }
}