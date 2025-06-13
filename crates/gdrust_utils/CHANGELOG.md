# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2025-06-13)

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

 - <csr-id-501ca3b639db0954ee8e09ca80110e105e81802b/> Add gdrust_utils crate with fuzzy logic implementation
   - Introduced gdrust_utils crate for utility tools in Rust Godot development.
   - Implemented fuzzy logic system with FuzzySet, FuzzyRule, and FuzzySystem structs.
   - Updated dependencies in gdrust_pathfinding, and gdrust_player_controller to use godot version 0.3.0.
   - Added example demonstrating fuzzy logic application for game difficulty settings.
   - Updated Cargo.toml files to include new utils crate and its features.

### Bug Fixes

 - <csr-id-ea73c68841ddd26b57b77b023e73a7cae9678be1/> add Clone derive to FuzzyRule and FuzzySystem

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 9 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Clean up Cargo.toml and README files ([`d476dd1`](https://github.com/robotnik-dev/gdrust_kit/commit/d476dd1593b1f5ecc9c0efd5d8782b9ffeb6a814))
    - Update dependencies and documentation ([`9b0ad5f`](https://github.com/robotnik-dev/gdrust_kit/commit/9b0ad5f0fab90c1ead29b50ebf5e61cfbc69c48c))
    - Add Clone derive to FuzzyRule and FuzzySystem ([`ea73c68`](https://github.com/robotnik-dev/gdrust_kit/commit/ea73c68841ddd26b57b77b023e73a7cae9678be1))
    - Add gdrust_utils crate with fuzzy logic implementation ([`501ca3b`](https://github.com/robotnik-dev/gdrust_kit/commit/501ca3b639db0954ee8e09ca80110e105e81802b))
</details>

