# moju-derive

Rust annotations for carrying reviewed MoJu metadata in source code.

`moju-derive` is used after a MoJu model has been reviewed and promoted into `moju/`. It lets stable model facts travel with code so future `moju-extract` runs can treat those annotations as high-confidence evidence.

## Role

```text
moju/ reviewed model
  -> sync stable metadata into code annotations
  -> moju-extract reads annotations as high-confidence facts
  -> drift report compares code, annotations, and moju/
```

The reviewed `moju/` model remains the source of truth. `#[moju(...)]` attributes are mirrors and anchors, not a replacement for `domain.mju`, `binding.mju`, or `profile.mju`.

## Supported Metadata

Current `#[derive(MoJu)]` support covers local type metadata:

- `kind`
- `domain`
- message `role`
- failure `identity`, `tag`, `description`
- storage `storage_kind`, `durability`
- actor `parent`
- field-level `#[moju(unique)]`

Example:

```rust
use moju_derive::MoJu;

#[derive(MoJu)]
#[moju(kind = "message", role = "command", domain = "Business")]
pub struct SubmitOrder {
    #[moju(unique)]
    pub id: String,
}
```

## Boundaries

Do not use type-level attributes to encode broad design structures such as flow steps, dataflow graph edges, interface routes, storage adapter providers, config sources, or generation profile choices. Those remain in MoJu model files.
