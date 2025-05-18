# gdrust_kit

A collection of helpful tools for Rust Godot development.

## Available Tools

Each tool can be enabled with its own feature flag:

- `pathfinding`: Tools for pathfinding algorithms
- `collision`: Tools for collision detection and physics

## Usage

### Per default no tool is compiled, you have to choose the feature

```toml
[dependencies]
gdrust_kit = { version = "0.1" } # this compiles nothing
```

### Using all tools

```toml
[dependencies]
gdrust_kit = { version = "0.1", features = ["all"] }
```

### Using just a single tool

```toml
[dependencies]
gdrust_kit = { version = "0.1", features = ["collision"] }
```