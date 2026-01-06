use crate::physics::body::Body;
use crate::math::vec2::{self, Vec2};
use crate::Sat;

#[derive(Clone)]
pub struct Rectangle{
    is_static : bool,
    pos : Vec2,
    size : Vec2,
    hitbox : Sat,
    velocity : Vec2,
    is_on_floor : bool,
    time_of_down : f64,
}

impl Rectangle{
    pub fn new(x : f64,y : f64,width : f64,height : f64) -> Self{
        let pos = Vec2::new(x, y);
        let size = Vec2::new(width, height);
        let hitbox = Sat::new(vec![
            Vec2::new(x, y),                 
            Vec2::new(x + width, y),         
            Vec2::new(x + width, y + height),
            Vec2::new(x, y + height),        
        ]);

        Self{
            is_static : false,
            pos,
            size,
            hitbox,
            velocity : Vec2::default(),
            is_on_floor : false,
            time_of_down : 0.0,
        }
    }

    pub fn get_pos(&self) -> Vec2{
        self.pos.clone()
    }

    pub fn get_size(&self) -> Vec2{
        self.size.clone()
    }

    pub fn get_hitbox(&self) -> Sat{
        self.hitbox.clone()
    }

    pub fn update_hitbox(&mut self){
        self.hitbox.set_verticles(vec![
            Vec2::new(self.get_pos().get_x(), self.get_pos().get_y()),
            Vec2::new(self.get_pos().get_x() + self.get_size().get_x(), self.get_pos().get_y()),
            Vec2::new(self.get_pos().get_x() + self.get_size().get_x(), self.get_pos().get_y() + self.get_size().get_y()),
            Vec2::new(self.get_pos().get_x(), self.get_pos().get_y() + self.get_size().get_y()),
        ]);
    }
}

impl Body for Rectangle{
    fn update(&mut self, dt: f32) {
        let dt = dt as f64;

        if !self.is_static {
            if !self.is_on_floor {
                let g = 9.8 * 100.0;
                self.velocity.set_y(self.velocity.get_y() + g * dt);
            }

            self.pos.set_x(self.pos.get_x() + self.velocity.get_x() * dt);
            self.pos.set_y(self.pos.get_y() + self.velocity.get_y() * dt);
        }

        self.update_hitbox();
    }
}