use std::collections::HashMap;

use godot::{
    classes::{INode, Node},
    obj::WithBaseField,
    prelude::{godot_api, Base, GodotClass},
};

use crate::collider::Collider2D;

#[derive(GodotClass)]
#[class(tool, init, base=Node)]
pub struct CollisionHandler {
    collider: HashMap<i32, Collider2D>,
    base: Base<Node>,
}

#[godot_api]
impl INode for CollisionHandler {
    fn enter_tree(&mut self) {
        self.base_mut()
            .add_to_group_ex("CollisionHandler")
            .persistent(true)
            .done();
    }

    fn physics_process(&mut self, delta: f64) {
        // WIP collision checking
    }
}

#[godot_api]
impl CollisionHandler {
    pub fn register_collider_2d(&mut self, collider2d: Collider2D) -> Result<i32, String> {
        // do nothing when its already registered

        // registering in hashmap, generating an ID

        // connecting the exit tree signal to unregister the collider with the same ID
        Ok(0)
    }

    pub fn update_collider(id: i32, new_collider: Collider2D) -> Result<i32, String> {
        Ok(0)
    }

    fn unregister_collider2d(id: i32) -> Result<i32, String> {
        Ok(0)
    }
}
