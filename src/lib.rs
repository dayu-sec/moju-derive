// Re-export the derive macro from the proc-macro sub-crate.
pub use moju_derive_macros::MoJu;

/// The trait that `#[derive(MoJu)]` implements.
///
/// Generated code depends on this trait to carry .mju metadata at runtime.
/// Extraction tools can also use it for compiled-type reflection.
pub trait MoJuItem {
    /// .mju item kind: `"struct"`, `"state"`, `"event"`, `"message"`,
    /// `"failure"`, `"cap"`, `"actor"`, `"module"`, `"interface"`,
    /// `"storage"`, `"command"`, `"dataflow"`, `"lifecycle"`, `"layer"`,
    /// `"dependency_rule"`, `"decision"`, `"failure_policy"`, `"flow"`,
    /// `"verify"`, `"target"`.
    fn moju_kind() -> &'static str;

    /// Domain name, e.g. `"Business"`, `"Storage"`.
    fn moju_domain() -> &'static str;

    /// Message role, if kind is `"message"`: `"command"`, `"query"`, `"response"`.
    fn moju_role() -> Option<&'static str> {
        None
    }

    /// Failure identity path, if kind is `"failure"`, e.g. `"payment.timeout"`.
    fn moju_identity() -> Option<&'static str> {
        None
    }

    /// Failure tag, if kind is `"failure"`, e.g. `"payment"`.
    fn moju_tag() -> Option<&'static str> {
        None
    }

    /// Storage kind, if kind is `"storage"`:
    /// `"table"`, `"document"`, `"key_value"`, `"queue"`, `"object"`,
    /// `"search"`, `"graph"`, `"cache"`.
    fn moju_storage_kind() -> Option<&'static str> {
        None
    }

    /// Storage durability, if kind is `"storage"`:
    /// `"transient"`, `"persistent"`, `"derived"`.
    fn moju_durability() -> Option<&'static str> {
        None
    }

    /// Actor parent name, if kind is `"actor"`, e.g. `"User"`.
    fn moju_parent() -> Option<&'static str> {
        None
    }

    /// Failure description text, if kind is `"failure"`.
    fn moju_description() -> Option<&'static str> {
        None
    }

    /// Field names marked `#[moju(unique)]`.
    fn moju_unique_fields() -> &'static [&'static str] {
        &[]
    }
}
