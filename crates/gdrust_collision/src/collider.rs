use godot::{
    builtin::{Rect2, Vector2},
    prelude::GodotClass,
};

pub trait Collider {
    fn intersects(&self, other: Self) -> bool;
}

#[derive(GodotClass)]
#[class(init)]
pub struct CollisionInfo {
    other: Collider2D,
    normal: Vector2,
    impact_point: Vector2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Collider2D {
    pub active: bool,
    pub aabb: Rect2,
    pub collision_layer: i32,
    pub collision_mask: i32,
}

impl Default for Collider2D {
    fn default() -> Self {
        Self {
            active: true,
            aabb: Rect2 {
                size: Vector2 { x: 1.0, y: 1.0 },
                ..Default::default()
            },
            collision_layer: 0,
            collision_mask: 0,
        }
    }
}

impl Collider2D {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(self, position: Vector2) -> Self {
        Collider2D {
            aabb: Rect2 {
                position,
                ..self.aabb
            },
            ..self
        }
    }

    pub fn with_size(self, size: Vector2) -> Self {
        Collider2D {
            aabb: Rect2 { size, ..self.aabb },
            ..self
        }
    }

    pub fn from_components(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            aabb: Rect2::from_components(x, y, width, height),
            ..Default::default()
        }
    }

    pub fn with_collision_layer(self, collision_layer: i32) -> Self {
        Collider2D {
            collision_layer,
            ..self
        }
    }

    pub fn with_collision_mask(self, collision_mask: i32) -> Self {
        Collider2D {
            collision_mask,
            ..self
        }
    }
}

impl Collider for Collider2D {
    fn intersects(&self, other: Self) -> bool {
        self.aabb.intersects(other.aabb)
    }
}
