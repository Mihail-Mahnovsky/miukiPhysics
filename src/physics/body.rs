use crate::physics::{physicsContext::PhysicsContext, rigetbody::RigitBody, sat::Sat, transform::Transform};


pub trait Body {
    fn update(&mut self,dt : f32,phys_context : &PhysicsContext);
    fn update_hitbox(&mut self);

    fn rigitbody(&mut self) -> &RigitBody;
    fn hitbox(&self) -> &Sat;
    fn transform(&mut self) -> &Transform;

    fn transform_mut(&mut self) -> &mut Transform;
    fn rigitbody_mut(&mut self) -> &mut RigitBody; 
}