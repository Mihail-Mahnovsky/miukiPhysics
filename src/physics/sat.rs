use crate::math::vec2::dot;
use crate::math::vec2::Vec2;

/*
    SAT
*/

#[derive(Clone)]
pub struct Sat {
    vertices: Vec<Vec2>,
}

impl Sat {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Sat { vertices }
    }

    pub fn set_verticles(&mut self, new_verticles: Vec<Vec2>) {
        self.vertices = new_verticles;
    }

    pub fn center(&self) -> Vec2 {
        let mut sum = Vec2::zero();
        for v in &self.vertices {
            sum += *v;
        }
        sum / self.vertices.len() as f64
    }

    pub fn project_polygon(&self, axis: &Vec2) -> (f64, f64) {
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
                    polygon[next].x - polygon[i].x,
                    polygon[next].y - polygon[i].y,
                );

                let axis = Vec2::new(-edge.y, edge.x);

                let (min_a, max_a) = self.project_polygon(&axis);
                let (min_b, max_b) = other.project_polygon(&axis);

                if max_a < min_b || max_b < min_a {
                    return false;
                }
            }
        }

        true
    }

    pub fn intersect(&self, other: &Sat) -> Option<Vec2> {
        let mut min_overlap = f64::INFINITY;
        let mut smallest_axis = Vec2::zero();

        for polygon in [&self.vertices, &other.vertices].iter() {
            for i in 0..polygon.len() {
                let next = (i + 1) % polygon.len();
                let edge = polygon[next] - polygon[i];

                let axis = Vec2::new(-edge.y, edge.x).normalize();

                let (min_a, max_a) = self.project_polygon(&axis);
                let (min_b, max_b) = other.project_polygon(&axis);

                if max_a < min_b || max_b < min_a {
                    return None;
                }

                let overlap = f64::min(max_a, max_b) - f64::max(min_a, min_b);

                if overlap < min_overlap {
                    min_overlap = overlap;
                    smallest_axis = axis;
                }
            }
        }

        let center_a = self.center();
        let center_b = other.center();

        let direction = center_b - center_a;
        if dot(&direction, &smallest_axis) < 0.0 {
            smallest_axis = -smallest_axis;
        }

        Some(smallest_axis * min_overlap)
    }
}
