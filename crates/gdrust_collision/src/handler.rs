use godot::{
    classes::{Engine, Object},
    global::godot_error,
    init::InitLevel,
    obj::NewAlloc,
    prelude::{godot_api, Base, GodotClass},
};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct CollisionHandler {
    base: Base<Object>,
}

#[godot_api]
impl CollisionHandler {
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
