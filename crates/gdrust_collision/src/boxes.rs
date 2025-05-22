use godot::{
    builtin::{Color, Vector2},
    classes::Texture2D,
    obj::{Gd, WithBaseField},
    prelude::{godot_api, Base, GodotClass, INode2D, Node2D},
};

use crate::collider::Collider2D;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Hitbox2D {
    #[export]
    texture: Option<Gd<Texture2D>>,

    #[export(flags_2d_physics)]
    collision_layer: i32,

    #[export(flags_2d_physics)]
    collision_mask: i32,

    collider2d: Collider2D,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Hitbox2D {
    fn enter_tree(&mut self) {}

    fn draw(&mut self) {
        let aabb = self.collider2d.aabb;
        self.base_mut()
            .draw_rect_ex(aabb, Color::BLUE)
            .filled(false)
            .width(1.0)
            .done();
    }
}

#[godot_api]
impl Hitbox2D {
    #[func]
    pub fn from_size(size: Vector2) -> Gd<Self> {
        todo!()
    }

    #[func]
    pub fn from_texture(texture: Gd<Texture2D>) -> Gd<Self> {
        todo!()
    }
}
