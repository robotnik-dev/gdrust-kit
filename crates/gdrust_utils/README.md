# gdrust_utils

[![Crates.io](https://img.shields.io/crates/v/gdrust_utils)](https://crates.io/crates/gdrust_utils)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docs](https://docs.rs/gdrust_utils/badge.svg)](https://docs.rs/gdrust_utils)

Utility tools for Rust Godot development.


## Features

### Simple fuzzy system
calcualte from a fuzzy input type (Game difficulty -> f32 from 0.0 to 10.0) to an exact output type (Game parameters):

```
--- Difficulty Level: 6.0 ---
  Normal membership: 0.50
  Hard membership: 0.50
  Resulting settings:
    EnemyHealth: 1.200
    EnemyDamage: 1.150
    EnemySpeed: 1.100
    CheckpointFrequency: 0.450
    ResourceScarcity: 0.650
```
## Usage

### Fuzzy system
Run example with:
```bash
cargo run --example fuzzy
```
Or from gdrust_kit crate with:
```bash
cargo run --example fuzzy -p gdrust_utils
```

## Installation

```toml
# Via gdrust_kit
gdrust_kit = { version = "0.1.0", features = ["utils"] }

# Or standalone
gdrust_utils = "0.1.0"
```

[Documentation](https://docs.rs/gdrust_utils) | [GitHub](https://github.com/robotnik-dev/gdrust_kit)