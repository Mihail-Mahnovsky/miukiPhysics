use crate::math::vec2::dot;
use crate::physics::physicsContext::PhysicsContext;
use crate::{math::vec2::Vec2, physics::body::Body};

use std::cell::RefCell;
use std::rc::Rc;

const FIXED_DT: f64 = 0.02;
const MAX_STEPS: usize = 5;

pub struct World {
    objects: Vec<Rc<RefCell<dyn Body>>>,
    pub context: PhysicsContext,
    accumulator: f64,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            context: PhysicsContext {
                gravity: Vec2::new(0.0, 980.0),
                dt : 0.0
            },
            accumulator : 0.0,
        }
    }

    pub fn set_gravity(&mut self, new_gravity: Vec2) {
        self.context.gravity = new_gravity;
    }

    pub fn get_gravity(&self) -> Vec2 {
        self.context.gravity.clone()
    }

    pub fn push(&mut self, obj: Rc<RefCell<dyn Body>>) {
        self.objects.push(obj);
    }

    pub fn at(&self, index: usize) -> Rc<RefCell<dyn Body>> {
        self.objects[index].clone()
    }

    pub fn collide(&mut self) {
        let len = self.objects.len();

        for i in 0..len {
            for j in (i + 1)..len {
                let a = self.objects[i].clone();
                let b = self.objects[j].clone();

                
            }
        }
    }
    pub fn step(&mut self,real_dt : f64){
        self.accumulator += real_dt as f64;

        let mut steps = 0;
        while self.accumulator >= FIXED_DT && steps < MAX_STEPS {

            self.context.dt = FIXED_DT;
            for obj in &self.objects {
                obj.borrow_mut().update(FIXED_DT as f32, &self.context);
            }

            self.collide();

            self.accumulator -= FIXED_DT;
            steps += 1;
        }
    }
}
