//! Declaration-audience surface: exact re-exports from the Query engine.

/// Canonical proof-carrying query artifact for declaration consumers.
///
/// ```
/// use worth_query_decl::facade::CanonicalQueryArtifact;
/// # fn _retain(artifact: CanonicalQueryArtifact) -> CanonicalQueryArtifact {
/// #     artifact
/// # }
/// ```
pub use worth_query::facade::foundation::CanonicalQueryArtifact;
