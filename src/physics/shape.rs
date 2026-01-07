use crate::{math::vec2::Vec2, physics::{sat::Sat, transform::{self, Transform}}};


pub enum ShapeKind {
    Rect { size: Vec2 },
}


pub struct Shape{
    pub transform : Transform,
    pub hitbox : Sat,
    kind : ShapeKind,
}

impl Shape {
    pub fn new(kind: ShapeKind, x: f64, y: f64) -> Self {
        let size = match &kind {
            ShapeKind::Rect { size } => *size,
        };

        let transform = Transform::new(x, y, size.x, size.y);

        let p = transform.pos;
        let s = transform.size;

        let vertices = vec![
            Vec2::new(p.x, p.y),
            Vec2::new(p.x + s.x, p.y),
            Vec2::new(p.x + s.x, p.y + s.y),
            Vec2::new(p.x, p.y + s.y),
        ];

        Self {
            transform,
            hitbox: Sat::new(vertices),
            kind,
        }
    }

    pub fn update_sat(&mut self) {
        match &self.kind {
            ShapeKind::Rect { size: _ } => {
                let p = self.transform.pos;
                let s = self.transform.size;

                self.hitbox.set_verticles(vec![
                    Vec2::new(p.x, p.y),
                    Vec2::new(p.x + s.x, p.y),
                    Vec2::new(p.x + s.x, p.y + s.y),
                    Vec2::new(p.x, p.y + s.y),
                ]);
            }
        }
    }
}
