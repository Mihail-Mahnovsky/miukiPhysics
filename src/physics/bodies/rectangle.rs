use crate::physics::body::Body;
use crate::math::vec2::Vec2;
use crate::physics::shape::{Shape, ShapeKind};
use crate::physics::rigetbody::RigetBody;
use crate::physics::transform::Transform;
use crate::Sat;

pub struct Rectangle {
    pub shape: Shape,
    pub body: RigetBody,
}


impl Rectangle{
    pub fn new(x : f64, y : f64, width : f64,height : f64) -> Self {
        Self { shape: Shape::new(ShapeKind::Rect { size: Vec2::new(width, height) }, x, y), body: RigetBody::new() }
    }
}

impl Body for Rectangle {
    fn update(&mut self, dt: f32) {
        let dt = dt as f64;

        self.body.update(dt,&mut self.shape.transform);
        self.shape.update_sat();
    }
}
