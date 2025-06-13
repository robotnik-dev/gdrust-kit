# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.2 (2025-06-13)

### Chore

 - <csr-id-d476dd1593b1f5ecc9c0efd5d8782b9ffeb6a814/> clean up Cargo.toml and README files
   - Updated descriptions for clarity across multiple crates.
   - Removed unnecessary dependencies from Cargo.toml files.
   - Simplified documentation comments in lib.rs files.
 - <csr-id-9b0ad5f0fab90c1ead29b50ebf5e61cfbc69c48c/> update dependencies and documentation
   - Update README files to reflect new features and usage.
   - Remove unused collision handling code.
   - Add tests for fuzzy logic functionality in gdrust_utils.

### New Features

 - <csr-id-e6abaa12b1877a259c53397e3d1cc119438e54ae/> enhance collision handling with registration methods
   - Added methods to register and unregister colliders in CollisionBox.
   - Refactored CollisionHandler to manage collider registration.
 - <csr-id-2aabf0192ea01a35bd848b1a43314b989294b9ba/> update dependencies and refactor collision handling in gdrust_collision crate
   - Added new dependencies to Cargo.toml for gdrust_collision and gdrust_pathfinding
   - Refactored CollisionHandler and Collider2D implementations
   - Introduced CollisionBox struct for improved collision management
   - Removed Hitbox2D and updated usage examples in documentation

### Bug Fixes

 - <csr-id-831002245762f04f2ac3a8c1de015a15fff1861e/> update singleton name for collision handler
   Refactor references from COLLISION_HANDLER_NODE to SINGLETON_NAME for consistency.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 20 calendar days.
 - 21 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Clean up Cargo.toml and README files ([`d476dd1`](https://github.com/robotnik-dev/gdrust_kit/commit/d476dd1593b1f5ecc9c0efd5d8782b9ffeb6a814))
    - Update dependencies and documentation ([`9b0ad5f`](https://github.com/robotnik-dev/gdrust_kit/commit/9b0ad5f0fab90c1ead29b50ebf5e61cfbc69c48c))
    - Update singleton name for collision handler ([`8310022`](https://github.com/robotnik-dev/gdrust_kit/commit/831002245762f04f2ac3a8c1de015a15fff1861e))
    - Enhance collision handling with registration methods ([`e6abaa1`](https://github.com/robotnik-dev/gdrust_kit/commit/e6abaa12b1877a259c53397e3d1cc119438e54ae))
    - Update dependencies and refactor collision handling in gdrust_collision crate ([`2aabf01`](https://github.com/robotnik-dev/gdrust_kit/commit/2aabf0192ea01a35bd848b1a43314b989294b9ba))
</details>

## v0.1.1 (2025-05-23)

<csr-id-cb34d2cb8694a1f2d663b52e9da4186d600be54a/>

### Chore

 - <csr-id-cb34d2cb8694a1f2d663b52e9da4186d600be54a/> add CHANGELOG.md for gdrust_player_controller crate

### New Features

 - <csr-id-8c18fa3f484baf175b506a00ddea0d36d24959df/> update dependencies and improve documentation for gdrust_kit and gdrust_collision
   - Updated license to MIT for gdrust_kit, gdrust_collision, and gdrust_pathfinding.

### Bug Fixes

 - <csr-id-36cb651842e1ca83bc164ce7545669e6f1cb3558/> update license badge formatting in README files for consistency

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 4 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gdrust_collision v0.1.1, gdrust_pathfinding v0.1.1, gdrust_player_controller v0.1.0, gdrust_kit v0.1.1 ([`0e81a0f`](https://github.com/robotnik-dev/gdrust_kit/commit/0e81a0f0edee2bfb5cc01b8d416f9d9fcb7549e0))
    - Add CHANGELOG.md for gdrust_player_controller crate ([`cb34d2c`](https://github.com/robotnik-dev/gdrust_kit/commit/cb34d2cb8694a1f2d663b52e9da4186d600be54a))
    - Release gdrust_collision v0.1.1, gdrust_pathfinding v0.1.1, gdrust_player_controller v0.1.0, gdrust_kit v0.1.1 ([`1f1b905`](https://github.com/robotnik-dev/gdrust_kit/commit/1f1b9052e5129748013ef62c17acb9c7050786b4))
    - Update license badge formatting in README files for consistency ([`36cb651`](https://github.com/robotnik-dev/gdrust_kit/commit/36cb651842e1ca83bc164ce7545669e6f1cb3558))
    - Update dependencies and improve documentation for gdrust_kit and gdrust_collision ([`8c18fa3`](https://github.com/robotnik-dev/gdrust_kit/commit/8c18fa3f484baf175b506a00ddea0d36d24959df))
</details>

<csr-unknown>
Added new dependencies and features in Cargo.lock.Enhanced documentation in gdrust_collision with usage examples and feature descriptions.Introduced Hitbox2D and Collider2D structs with methods for collision handling.Added CollisionHandler for managing collision detection in Godot.<csr-unknown/>

## v0.1.0 (2025-05-18)

<csr-id-303760ca5d73dfccc98e068fe6f9189c1d49f657/>



### Bug Fixes

 - <csr-id-9e20de2feb8e733f04499d77f6b40cddc5faaece/> update repository URLs to use consistent naming convention

### Refactor

 - <csr-id-303760ca5d73dfccc98e068fe6f9189c1d49f657/> rename packages and update dependencies for consistency

### New Features

 - <csr-id-e93f1b71a2c82680fda9da87c99fb88f344a77f2/> add CHANGELOG.md for gdrust_kit, gdrust_collision, and gdrust_pathfinding crates

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gdrust_collision v0.1.0, gdrust_pathfinding v0.1.0, gdrust_kit v0.1.0 ([`accba02`](https://github.com/robotnik-dev/gdrust_kit/commit/accba0212347a6855958d46c34fefd4db45a0174))
    - Release gdrust_collision v0.1.0, gdrust_pathfinding v0.1.0, gdrust_kit v0.1.0 ([`9acf5bf`](https://github.com/robotnik-dev/gdrust_kit/commit/9acf5bfaa8fa9e932fe86c74d02899e8056c7a22))
    - Add CHANGELOG.md for gdrust_kit, gdrust_collision, and gdrust_pathfinding crates ([`e93f1b7`](https://github.com/robotnik-dev/gdrust_kit/commit/e93f1b71a2c82680fda9da87c99fb88f344a77f2))
    - Fix: update descriptions in Cargo.toml and README.md for clarity feat: add README.md for gdrust_collision crate fix: update descriptions in gdrust_collision and gdrust_pathfinding Cargo.toml files ([`3a6df21`](https://github.com/robotnik-dev/gdrust_kit/commit/3a6df214d0d4410dec4f6c234a70f5c6622d75e3))
    - Fix: update license information to include Apache-2.0 docs: enhance README with detailed usage instructions for tools ([`30cf5f2`](https://github.com/robotnik-dev/gdrust_kit/commit/30cf5f25e4c2c7b677553da6f79e1f8a47876e90))
    - Update repository URLs to use consistent naming convention ([`9e20de2`](https://github.com/robotnik-dev/gdrust_kit/commit/9e20de2feb8e733f04499d77f6b40cddc5faaece))
    - Rename packages and update dependencies for consistency ([`303760c`](https://github.com/robotnik-dev/gdrust_kit/commit/303760ca5d73dfccc98e068fe6f9189c1d49f657))
</details>

