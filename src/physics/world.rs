use crate::physics::physicsContext::PhysicsContext;
use crate::{math::vec2::Vec2, physics::body::Body};

use std::rc::Rc;
use std::cell::RefCell;

pub struct World{
    objects : Vec<Rc<RefCell<dyn Body>>>,
    pub context : PhysicsContext,
}

impl World{
    pub fn new() -> Self{
        Self { 
            objects : Vec::new(),
            context : PhysicsContext{ gravity : Vec2::new(0.0, 980.0), dt : 0.0, }
        }
    }

    pub fn set_gravity(&mut self,new_gravity : Vec2){
        self.context.gravity = new_gravity;
    }

    pub fn get_gravity(&self) -> Vec2{
        self.context.gravity.clone()
    }

    pub fn push(&mut self, obj: Rc<RefCell<dyn Body>>) {
        self.objects.push(obj);
    }

    pub fn at(&self, index: usize) -> Rc<RefCell<dyn Body>> {
        self.objects[index].clone()
    }

    pub fn step(&mut self,dt : f32){
        self.context.dt = dt as f64;
        for obj in &self.objects {
            obj.borrow_mut().update(dt,&self.context);
        }
    }
}