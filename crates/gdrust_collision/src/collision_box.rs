use godot::{
    builtin::{Color, Vector2},
    classes::{Engine, Texture2D},
    global::{godot_error, godot_print},
    obj::{Gd, WithBaseField},
    prelude::{godot_api, Base, GodotClass, INode2D, Node2D},
};

use crate::{
    collider::{Collider2D, CollisionInfo},
    handler::{CollisionHandler, SINGLETON_NAME},
};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct CollisionBox {
    #[export]
    texture: Option<Gd<Texture2D>>,

    #[export(flags_2d_physics)]
    #[var(get, set = set_collision_layer)]
    #[init(val = 1)]
    collision_layer: i32,

    #[export(flags_2d_physics)]
    #[var(get, set = set_collision_mask)]
    #[init(val = 1)]
    collision_mask: i32,

    collider2d: Collider2D,

    registered_id: Option<i32>,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for CollisionBox {
    fn enter_tree(&mut self) {
        self.register_self();
    }

    fn draw(&mut self) {
        let aabb = self.collider2d.aabb;
        self.base_mut()
            .draw_rect_ex(aabb, Color::BLUE)
            .filled(false)
            .width(1.0)
            .done();
    }

    fn exit_tree(&mut self) {
        self.unregister_self();
    }
}

#[godot_api]
impl CollisionBox {
    #[signal]
    pub fn hit(info: Gd<CollisionInfo>);

    #[func]
    pub fn from_size(size: Vector2) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            texture: None,
            collision_layer: 0,
            collision_mask: 0,
            collider2d: Collider2D::new().with_size(size),
            registered_id: None,
            base,
        })
    }

    #[func]
    pub fn from_texture(texture: Gd<Texture2D>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            texture: Some(texture),
            collision_layer: 0,
            collision_mask: 0,
            collider2d: Collider2D::new(),
            registered_id: None,
            base,
        })
    }

    #[func]
    pub fn set_collision_layer(&mut self, collision_layer: i32) {
        self.collision_layer = collision_layer;
        self.collider2d.collision_layer = collision_layer;
    }

    #[func]
    pub fn set_collision_mask(&mut self, collision_mask: i32) {
        self.collision_mask = collision_mask;
        self.collider2d.collision_mask = collision_mask;
    }

    fn register_self(&mut self) {
        if let Some(handler) = Engine::singleton().get_singleton(SINGLETON_NAME) {
            let mut handler = handler.cast::<CollisionHandler>();
            let id = handler.bind().generate_unique_id();
            match handler
                .bind_mut()
                .register_collider_2d(id, self.collider2d.clone())
            {
                Ok(_) => {
                    godot_print!(
                        "registered collider: {:?} with id: {id}",
                        self.collider2d.clone()
                    );
                    self.registered_id = Some(id);
                }
                Err(err) => {
                    godot_error!("Some error while registering Collider2d: {:?}", err);
                }
            };
        } else {
            godot_error!("No Node named: {SINGLETON_NAME} found");
        }
    }

    fn unregister_self(&mut self) {
        if let Some(id) = self.registered_id {
            if let Some(handler) = Engine::singleton().get_singleton(SINGLETON_NAME) {
                match handler
                    .cast::<CollisionHandler>()
                    .bind_mut()
                    .unregister_collider2d(id)
                {
                    Ok(collider) => {
                        godot_print!("unregistered collider: {:?} with id: {id}", collider);
                    }
                    Err(err) => {
                        godot_error!(
                            "Some error while unregistering collider: {:?} with error {:?}",
                            self.collider2d.clone(),
                            err
                        );
                    }
                };
            } else {
                godot_error!("No Node named: {SINGLETON_NAME} found");
            }
        }
    }
}
