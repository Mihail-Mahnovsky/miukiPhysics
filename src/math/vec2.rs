use std::ops::{Add, AddAssign, Sub, Mul};

#[derive(Clone,Copy)]
pub struct Vec2{
    pub x : f64,
    pub y : f64,
}

impl Vec2{
    pub fn default() -> Self{
        Self { 
            x: 0.0, 
            y: 0.0 
        }
    }

    pub fn new(x : f64,y : f64) -> Self{
        Self { 
            x,
            y,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}


pub fn dot(v1 : &Vec2,v2 : &Vec2) -> f64{
    v1.x * v2.x + v1.y * v2.y
}