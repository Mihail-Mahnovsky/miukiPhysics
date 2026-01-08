use crate::physics::physicsContext::PhysicsContext;


pub trait Body {
    fn update(&mut self,dt : f32,phys_context : &PhysicsContext);
    fn update_hitbox(&mut self);
}