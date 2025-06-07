/*!
# gdrust_kit

A collection of helpful tools for Rust Godot development.

## Available Tools

Each tool can be enabled with its own feature flag:

- `pathfinding`: Tools for pathfinding algorithms
- `collision`: Tools for collision detection and physics
- `player_controller`: Tools for managing player input and controls
- `utils`: Utility tools

## Usage

```toml
[dependencies]
gdrust_kit = { version = "0.1", features = ["collision"] }
```
*/

#[cfg(feature = "pathfinding")]
pub use gdrust_pathfinding as pathfinding;

#[cfg(feature = "collision")]
pub use gdrust_collision as collision;

#[cfg(feature = "player_controller")]
pub use gdrust_player_controller as player_controller;

#[cfg(feature = "utils")]
pub use gdrust_utils as utils;

/// Version information for the meta package
pub mod version {
    /// Returns the version of the meta package
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Returns available tools in this build
    pub fn available_tools() -> Vec<&'static str> {
        #[allow(unused_mut)]
        let mut tools = Vec::new();

        #[cfg(feature = "pathfinding")]
        tools.push("pathfinding");

        #[cfg(feature = "collision")]
        tools.push("collision");

        #[cfg(feature = "player_controller")]
        tools.push("player_controller");

        #[cfg(feature = "utils")]
        tools.push("utils");

        tools
    }
}
