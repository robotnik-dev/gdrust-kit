/*!
# Utility tools

## Features

- Simple fuzzy logic

## Usage

*/

pub mod fuzzy;

/// Information about this package
pub mod version {
    /// Returns the current version of this package
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
