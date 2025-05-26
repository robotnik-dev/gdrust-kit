use std::collections::HashMap;

use crate::{collider::Collider2D, collision_box::CollisionBox};
use godot::{
    classes::{EditorPlugin, Engine, IEditorPlugin, Object},
    global::{godot_error, godot_print},
    init::InitLevel,
    meta::ToGodot,
    obj::{Gd, NewAlloc},
    prelude::{godot_api, Base, GodotClass},
};

pub const COLLISION_HANDLER_NODE: &str = "CollisionHandler";

#[derive(GodotClass)]
#[class(tool, init, base=EditorPlugin)]
struct CollisionHandlerPlugin {
    handler: Option<Gd<CollisionHandler>>,
    base: Base<EditorPlugin>,
}

#[godot_api]
/// Delegating the virtual functions Godot uses to the CollisionHandler Object because Godot "Objects" are not part of the scene tree
/// and cant receive them.
impl IEditorPlugin for CollisionHandlerPlugin {
    fn enter_tree(&mut self) {
        if let Some(handler) = Engine::singleton().get_singleton(COLLISION_HANDLER_NODE) {
            let mut handler = handler.cast::<CollisionHandler>();
            handler.call("enter_tree", &[]);
            // saving it internally as a Gd smart pointer to avoid getting a singleton every frame in the process functions
            self.handler = Some(handler);
        }
    }

    fn ready(&mut self) {
        if let Some(handler) = self.handler.as_mut() {
            handler.call("ready", &[]);
        }
    }

    fn process(&mut self, delta: f64) {
        if let Some(handler) = self.handler.as_mut() {
            handler.call("process", &[delta.to_variant()]);
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if let Some(handler) = self.handler.as_mut() {
            handler.call("physics_process", &[delta.to_variant()]);
        }
    }

    fn exit_tree(&mut self) {
        if let Some(handler) = self.handler.as_mut() {
            handler.call("exit_tree", &[]);
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct CollisionHandler {
    colliders: HashMap<i32, Collider2D>,
    base: Base<Object>,
}

#[godot_api]
impl CollisionHandler {
    #[signal]
    fn collision_box_registered(&mut self, id: i32, collision_box: Gd<CollisionBox>);

    #[signal]
    fn collision_box_unregistered(&mut self, id: i32, collision_box: Gd<CollisionBox>);

    #[func]
    pub fn enter_tree(&mut self) {}

    #[func]
    pub fn ready(&mut self) {}

    #[func]
    pub fn process(&mut self, delta: f64) {}

    #[func]
    pub fn physics_process(&mut self, delta: f64) {}

    #[func]
    pub fn exit_tree(&mut self) {}

    pub fn generate_unique_id(&self) -> i32 {
        todo!()
    }

    pub fn register_collider_2d(&mut self, id: i32, collider: Collider2D) -> Result<(), String> {
        // return AlreadyRegisteredError when id is duplicate

        // add to hashmap

        // emit registered signal

        todo!()
    }

    pub fn update_collider(&mut self, id: i32, collider: Collider2D) -> Result<i32, String> {
        todo!()
    }

    pub fn unregister_collider2d(&mut self, id: i32) -> Result<Collider2D, String> {
        // emit unregistered signal
        todo!()
    }

    pub fn register(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton()
                .register_singleton("CollisionHandler", &CollisionHandler::new_alloc());
        }
    }

    pub fn unregister(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let collision_handler = "CollisionHandler";

            if let Some(singleton) = engine.get_singleton(collision_handler) {
                engine.unregister_singleton(collision_handler);
                singleton.free();
            } else {
                godot_error!("Failed to get singleton: {collision_handler}");
            }
        }
    }
}
