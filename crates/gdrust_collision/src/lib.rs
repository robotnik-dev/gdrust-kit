/*!
# Collision tools

Tools for simple collision detection for the godot-rust gdextension.

## Features

## Usage

*/

/// Information about this package
pub mod version {
    /// Returns the current version of this package
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
