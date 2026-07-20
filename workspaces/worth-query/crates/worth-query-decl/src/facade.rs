//! Declaration-audience surface: exact re-exports from declaration authority.

pub use worth_query_declaration::facade::{
    authoring, binding, canonicalization, collection, diagnostics, identity, identity_authority,
    schema_view, typed, validation, view_declaration,
};

/// Canonical proof-carrying query artifact for declaration consumers.
///
/// ```
/// use worth_query_decl::facade::CanonicalQueryArtifact;
/// # fn _retain(artifact: CanonicalQueryArtifact) -> CanonicalQueryArtifact {
/// #     artifact
/// # }
/// ```
pub use worth_query_declaration::facade::canonicalization::CanonicalQueryArtifact;
