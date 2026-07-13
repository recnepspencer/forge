//! Specimens for complete Cargo dependency authority (workspace/target/opaque).

/// Path dependency: renames AuthorityMarker to Gate.
pub fn dep_renames_marker_to_gate() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub use AuthorityMarker as Gate;
"#
}

/// Entry: uses workspace-inherited or target-specific dep Gate.
pub fn entry_cargo_use_dep_gate() -> &'static str {
    r#"
use worth_schema_authgate::Gate;

pub fn governed_ceremony<T: Gate>(_authority: T) {}
"#
}

/// Entry: qualified bound on workspace/target dep Gate.
pub fn entry_cargo_qualified_dep_gate() -> &'static str {
    r#"
pub fn governed_ceremony<T: worth_schema_authgate::Gate>(_authority: T) {}
"#
}

/// Dependency with item-position macro that can mint a renamed authority export.
pub fn dep_item_macro_export_generation() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

macro_rules! mint_gate {
    () => {
        pub use AuthorityMarker as Gate;
    };
}

mint_gate!();
"#
}

/// Dependency with opaque attribute on a public item (export-generation fence).
pub fn dep_opaque_attr_public_item() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

#[ceremony_export]
pub use AuthorityMarker as Gate;
"#
}

/// Dependency: opaque attribute on a *private* root item that can still mint a
/// public renamed export via procedural expansion.
pub fn dep_opaque_attr_private_item() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

#[ceremony_export]
use AuthorityMarker as HiddenRoot;
"#
}

/// Dependency: opaque attribute on a private module (expansion may emit pub use).
pub fn dep_opaque_attr_private_module() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

#[ceremony_export]
mod private_export {
    // expansion site: may emit `pub use AuthorityMarker as Gate`
}
"#
}

/// Dependency: private custom derive that can emit a public authority alias.
pub fn dep_private_custom_derive() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

#[derive(CeremonyAlias)]
struct PrivateCarrier;
"#
}

/// Dependency that fails module resolution (declared mod, missing source).
pub fn dep_unresolved_module_root() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub mod missing_export;
"#
}

/// Legal non-authority dep for workspace/target controls.
pub fn dep_describe_only() -> &'static str {
    r#"
pub trait Describe {}
"#
}

pub fn entry_use_describe() -> &'static str {
    r#"
use worth_schema_authgate::Describe;

pub fn describe<T: Describe>(_value: T) {}
"#
}

/// Entry for non-path registry dependency hostiles.
///
/// The dependency is declared only in Cargo.toml (version/registry, no path).
/// Sealing fails closed at source-kind inventory before any renamed export can
/// be omitted from the sealed-export index.
pub fn entry_non_path_registry_host() -> &'static str {
    r#"
pub fn governed_surface() {}
"#
}
