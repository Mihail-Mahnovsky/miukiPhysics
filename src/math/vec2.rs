
#[derive(Clone)]
pub struct Vec2{
    x : f64,
    y : f64,
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

    pub fn get_x(&self) -> f64{
        self.x
    }

    pub fn get_y(&self) -> f64{
        self.y
    }

    pub fn set_x(&mut self, new_x : f64) {
        self.x = new_x
    }

    pub fn set_y(&mut self, new_y : f64) {
        self.y = new_y
    }
}

pub fn dot(v1 : &Vec2,v2 : &Vec2) -> f64{
    v1.get_x() * v2.get_x() + v1.get_y() * v2.get_y()
}