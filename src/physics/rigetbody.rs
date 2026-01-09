use crate::{
    math::vec2::Vec2,
    physics::{
        physicsContext::PhysicsContext,
        transform::{self, Transform},
    },
};

#[derive(Clone)]
pub struct RigitBody {
    pub is_static: bool,
    pub is_on_floor: bool,
    pub velocity: Vec2,
    pub restitution : f64,
    pub friction : f64,
    pub mass : f64,
}

impl RigitBody {
    pub fn new() -> Self {
        Self {
            is_static: false,
            is_on_floor: false,
            velocity: Vec2::zero(),
            restitution : 0.3,
            friction : 0.8,
            mass : 1.0,
        }
    }

    pub fn add_force(&mut self, force : Vec2,context : &PhysicsContext){
        let aseleration = force / self.mass;
        self.velocity += aseleration * context.dt;
    }

    pub fn update(&mut self, transform: &mut Transform, context : &PhysicsContext) {
        if self.is_static {
            return;
        }

        let floor_y = 600.0;

        if !self.is_on_floor {
            self.velocity.y += context.gravity.y * context.dt;
        }

        transform.pos += self.velocity * context.dt;

        if transform.pos.y + transform.size.y >= floor_y {
            transform.pos.y = floor_y - transform.size.y;

            self.velocity.y = -self.velocity.y * self.restitution;

            if self.velocity.y.abs() < 1.0 {
                self.velocity.y = 0.0;
                self.is_on_floor = true;
            }
            else{
                self.is_on_floor = false;
            }
        } else {
            self.is_on_floor = false;
        }


        if self.is_on_floor {
            self.velocity.x *= self.friction;

            if self.velocity.x.abs() < 0.01 {
                self.velocity.x = 0.0;
            }
        }
    }
}
