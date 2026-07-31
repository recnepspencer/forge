//! Canonical Query declaration authority.
//!
//! This package owns authored intent, canonicalization, schema-visible
//! validation, binding grammar, result shapes, and collection declarations.
//! Runtime installation, execution, publication, and replay are downstream.

#![forbid(unsafe_code)]

mod application_capability;
mod application_query;
mod application_schema;
mod authentication;
mod authoring;
#[macro_use]
mod application_capability_macro;
#[macro_use]
mod application_schema_macro;
#[macro_use]
mod application_operation_macro;
mod binding;
mod canonicalization;
mod collection;
mod diagnostics;
mod domain_computation;
mod identity;
#[path = "canonical_authority.rs"]
mod identity_authority;
mod result_shape;
#[macro_use]
mod schema_macro;
#[path = "schema_basis_authority.rs"]
mod schema_basis_authority;
mod schema_view;
mod typed;
mod validation;
mod view_declaration;

#[cfg(test)]
mod ability_declaration_tests;
#[cfg(test)]
mod application_schema_tests;
#[cfg(test)]
mod authoring_contract_tests;
#[cfg(test)]
mod binding_contract_tests;
#[cfg(test)]
mod canonical_identity_tests;
#[cfg(test)]
mod canonicalization_normalization_tests;
#[cfg(test)]
mod principal_binding_tests;
#[cfg(test)]
mod typed_contract_tests;

mod basis {
    pub use crate::schema_basis_authority::QuerySchemaBasisAuthority;
}

pub mod facade;
