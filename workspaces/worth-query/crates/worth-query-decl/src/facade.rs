//! Declaration-audience surface: exact re-exports from declaration authority.

pub use worth_query_declaration::facade::{
    application_schema, authoring, binding, canonicalization, collection, diagnostics, identity,
    identity_authority, schema_view, typed, validation, view_declaration,
};
pub use worth_query_declaration::{
    worth_query_application_schema, worth_query_aspect, worth_query_currency, worth_query_effect,
    worth_query_entity, worth_query_field, worth_query_operation, worth_query_operation_creates,
    worth_query_operation_deletes, worth_query_operation_emits, worth_query_operation_links,
    worth_query_operation_unlinks, worth_query_operation_writes, worth_query_policy,
    worth_query_relation,
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
