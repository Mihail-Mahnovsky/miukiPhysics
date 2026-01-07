use crate::math::vec2::Vec2;

#[derive(Clone,Copy)]
pub struct Transform{
    pub pos : Vec2,
    pub size : Vec2,
    pub rotatation : f64,
}

impl Transform {
    pub fn new(x : f64, y : f64,width : f64, height : f64) -> Self{
         Self{
            pos :  Vec2::new(x, y),
            size : Vec2::new(width,height),
            rotatation : 0.0,
        }
    }
}