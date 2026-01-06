use crate::math::vec2::Vec2;
use crate::math::vec2::dot;

/*
    SAT
*/

#[derive(Clone)]
pub struct Sat{
    vertices: Vec<Vec2>,
}

impl Sat{
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Sat { vertices }
    }

    pub fn set_verticles(&mut self,new_verticles : Vec<Vec2>){
        self.vertices = new_verticles;
    }

    pub fn project_polygon(&self,axis: &Vec2) -> (f64,f64) {
        let mut min = dot(axis, &self.vertices[0]);
        let mut max = min;

        for vertex in &self.vertices[1..] {
            let p = dot(axis, vertex);
            if p < min {
                min = p;
            }
            if p > max {
                max = p;
            }
        }

        (min, max)
    }

        pub fn check_collision(&self, other: &Sat) -> bool {
        for polygon in [&self.vertices, &other.vertices].iter() {
            for i in 0..polygon.len() {
                let next = (i + 1) % polygon.len();
                let edge = Vec2::new(
                    polygon[next].get_x() - polygon[i].get_x(),
                    polygon[next].get_y() - polygon[i].get_y(),
                );
            
                let axis = Vec2::new(-edge.get_y(), edge.get_x());

                let (min_a, max_a) = self.project_polygon(&axis);
                let (min_b, max_b) = other.project_polygon(&axis);

                if max_a < min_b || max_b < min_a {
                    return false; 
                }
            }
        }

        true 
    }
}