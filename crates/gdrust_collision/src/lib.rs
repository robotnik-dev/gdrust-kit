/*!
# Collision tools

Tools for simple collision detection for the godot-rust gdextension.

## Features

- Engine Singleton that handles collision detection
- Hit- and Hurtboxes that register and unregister themselfs
- Fast and easy collision detection with simple AABB collision

## Usage

Register the Singleton in your main ExtentensionLibrary trait file: `my_game/rust/src/lib.rs`
```no_run
# use godot::prelude::*;
# use godot::classes::Engine;
# use gdrust_collision::handler::CollisionHandler;
struct GodotRustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRustExtension {
    fn on_level_init(level: InitLevel) {
        CollisionHandler::register(level);
    }

    fn on_level_deinit(level: InitLevel) {
        CollisionHandler::unregister(level);
    }
}
```

Create a new hitbox through code[^note] or the editor like any other node
[^note]: Don't forget to call my_node.free() when you're manually managing the lifecycle of nodes!
```no_run
# use godot::prelude::*;
# use gdrust_collision::boxes::Hitbox2D;
let mut bullet = Node2D::new_alloc();
let hitbox = Hitbox2D::new_alloc();   // default size is 1x1 Pixel
bullet.add_child(&hitbox);
```

Setting the size manually
```no_run
# use godot::prelude::*;
# use gdrust_collision::boxes::Hitbox2D;
let hitbox = Hitbox2D::from_size(Vector2 { x: 10.0, y: 10.0 });
```

Or let it be computed from a `Gd<Texture2D>`
```no_run
# use godot::prelude::*;
# use godot::classes::Sprite2D;
# use gdrust_collision::boxes::Hitbox2D;
let bullet = Sprite2D::new_alloc();
if let Some(texture) = bullet.get_texture() {
    let hitbox = Hitbox2D::from_texture(texture);
}
```

*/

pub mod boxes;
pub mod collider;
pub mod handler;

/// Information about this package
pub mod version {
    /// Returns the current version of this package
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
