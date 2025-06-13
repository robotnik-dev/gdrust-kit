# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.1 (2025-06-13)

<csr-id-d476dd1593b1f5ecc9c0efd5d8782b9ffeb6a814/>
<csr-id-9b0ad5f0fab90c1ead29b50ebf5e61cfbc69c48c/>

### Chore

 - <csr-id-d476dd1593b1f5ecc9c0efd5d8782b9ffeb6a814/> clean up Cargo.toml and README files
   - Updated descriptions for clarity across multiple crates.
   - Removed unnecessary dependencies from Cargo.toml files.
   - Simplified documentation comments in lib.rs files.
 - <csr-id-9b0ad5f0fab90c1ead29b50ebf5e61cfbc69c48c/> update dependencies and documentation
   - Update README files to reflect new features and usage.
   - Remove unused collision handling code.
   - Add tests for fuzzy logic functionality in gdrust_utils.

### Chore

 - <csr-id-ac630880fc0caf882f8b3c45a9976dfddf29a368/> update CHANGELOG.md
   Document all notable changes and adhere to Semantic Versioning.

### New Features

<csr-id-2aabf0192ea01a35bd848b1a43314b989294b9ba/>

 - <csr-id-501ca3b639db0954ee8e09ca80110e105e81802b/> Add gdrust_utils crate with fuzzy logic implementation
   - Introduced gdrust_utils crate for utility tools in Rust Godot development.
- Implemented fuzzy logic system with FuzzySet, FuzzyRule, and FuzzySystem structs.
- Updated dependencies in gdrust_pathfinding, and gdrust_player_controller to use godot version 0.3.0.
- Added example demonstrating fuzzy logic application for game difficulty settings.
- Updated Cargo.toml files to include new utils crate and its features.
- Added new dependencies to Cargo.toml for gdrust_collision and gdrust_pathfinding
- Refactored CollisionHandler and Collider2D implementations
- Introduced CollisionBox struct for improved collision management
- Removed Hitbox2D and updated usage examples in documentation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 20 calendar days.
 - 21 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update CHANGELOG.md ([`ac63088`](https://github.com/robotnik-dev/gdrust_kit/commit/ac630880fc0caf882f8b3c45a9976dfddf29a368))
    - Release gdrust_collision v0.1.2, gdrust_pathfinding v0.1.2, gdrust_player_controller v0.1.1, gdrust_utils v0.1.0, gdrust_kit v0.1.2 ([`bb4bcbb`](https://github.com/robotnik-dev/gdrust_kit/commit/bb4bcbb797c8747de50b5276eb65b17af76249e6))
    - Clean up Cargo.toml and README files ([`d476dd1`](https://github.com/robotnik-dev/gdrust_kit/commit/d476dd1593b1f5ecc9c0efd5d8782b9ffeb6a814))
    - Update dependencies and documentation ([`9b0ad5f`](https://github.com/robotnik-dev/gdrust_kit/commit/9b0ad5f0fab90c1ead29b50ebf5e61cfbc69c48c))
    - Add gdrust_utils crate with fuzzy logic implementation ([`501ca3b`](https://github.com/robotnik-dev/gdrust_kit/commit/501ca3b639db0954ee8e09ca80110e105e81802b))
    - Update dependencies and refactor collision handling in gdrust_collision crate ([`2aabf01`](https://github.com/robotnik-dev/gdrust_kit/commit/2aabf0192ea01a35bd848b1a43314b989294b9ba))
</details>

<csr-unknown>
 update dependencies and refactor collision handling in gdrust_collision crate<csr-unknown/>

## v0.1.0 (2025-05-23)

<csr-id-cb34d2cb8694a1f2d663b52e9da4186d600be54a/>

### Chore

 - <csr-id-cb34d2cb8694a1f2d663b52e9da4186d600be54a/> add CHANGELOG.md for gdrust_player_controller crate

### New Features

 - <csr-id-8a3e59bbc521f74989d2953d402f61747e12e8ca/> add gdrust_player_controller crate with player input management tools

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gdrust_collision v0.1.1, gdrust_pathfinding v0.1.1, gdrust_player_controller v0.1.0, gdrust_kit v0.1.1 ([`0e81a0f`](https://github.com/robotnik-dev/gdrust_kit/commit/0e81a0f0edee2bfb5cc01b8d416f9d9fcb7549e0))
    - Add CHANGELOG.md for gdrust_player_controller crate ([`cb34d2c`](https://github.com/robotnik-dev/gdrust_kit/commit/cb34d2cb8694a1f2d663b52e9da4186d600be54a))
    - Add gdrust_player_controller crate with player input management tools ([`8a3e59b`](https://github.com/robotnik-dev/gdrust_kit/commit/8a3e59bbc521f74989d2953d402f61747e12e8ca))
</details>

