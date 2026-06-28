# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] — Unreleased

### Changed

- **Fix generics and lifetimes in derive macro**: `split_for_impl()` is now called after all local bindings, ensuring correct generic and lifetime handling in the generated `impl` block.

### Removed

- **Removed `module` metadata field**: The `module` attribute was removed from `TypeAttr`, attribute parsing, the `MoJuItem` trait, and both generated `impl` blocks. Module designation remains the responsibility of MoJu model files.

## [0.1.1] — 2025-06-01 (approximate)

### Changed

- Bumped version to 0.1.1.

### Removed

- Removed unused CI workflow files (`pages.yml`, `release.yml`).

## [0.1.0] — 2025-06-01 (approximate)

### Added

- Initial release of `moju-derive`, a proc-macro crate for annotating Rust types with reviewed MoJu metadata.
- `#[derive(MoJu)]` proc-macro with `#[moju(...)]` attribute support.
- Supported metadata kinds: `kind`, `domain`, `role`, `identity`, `tag`, `storage_kind`, `durability`, `parent`, `description`.
- Field-level `#[moju(unique)]` annotation support.
- `MoJuItem` trait with default method implementations.
- CI and release workflows.
- Test suite covering struct and enum derive patterns.

[0.1.2]: https://github.com/dayu-sec/moju-derive/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/dayu-sec/moju-derive/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/dayu-sec/moju-derive/releases/tag/v0.1.0
