# gdrust_kit

[![Crates.io](https://img.shields.io/crates/v/gdrust_kit)](https://crates.io/crates/gdrust_kit)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docs](https://docs.rs/gdrust_kit/badge.svg)](https://docs.rs/gdrust_kit)

A toolkit of Rust utilities for Godot game development.

## Tools

- `collision`: Collision tools
- `pathfinding`: Pathfinding tools
- `player_controller`: Player input and controls
- `utils`: Utility tools

## Usage

```toml
# Use specific tools
gdrust_kit = { version = "0.1.0", features = ["collision"] }

# Use all tools
gdrust_kit = { version = "0.1.0", features = ["all"] }

# Or use individual crates directly
gdrust_collision = "0.1.0"
```

```rust
// Import tools with
use gdrust_kit::utils;
```

## [Documentation](https://docs.rs/gdrust_kit) | [GitHub](https://github.com/robotnik-dev/gdrust_kit)