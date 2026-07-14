#[cfg(test)]
pub(crate) mod aspect_native_authority_denial_tests;
pub(crate) mod aspect_native_boundary_audit;
pub(crate) mod aspect_native_boundary_handoff;
#[cfg(test)]
pub(crate) mod aspect_native_identity_tests;
#[cfg(test)]
pub(crate) mod aspect_native_vocabulary_tests;
#[cfg(test)]
pub(crate) mod canonical_basis_entry_construction_tests;
#[cfg(test)]
pub(crate) mod canonical_basis_entry_denial_tests;
#[cfg(test)]
pub(crate) mod canonical_basis_entry_order_tests;
pub(crate) mod canonical_basis_source_inventory;
pub(crate) mod canonical_basis_source_registry;
pub(crate) mod canonical_basis_source_scan;
#[cfg(test)]
pub(crate) mod canonical_basis_source_tests;
#[cfg(test)]
pub(crate) mod digest_authority_denial_tests;
#[cfg(test)]
pub(crate) mod digest_authority_equivalence_tests;
pub(crate) mod foundational_boundary_performance;
#[cfg(test)]
pub(crate) mod handoff_contract_tests;
pub(crate) mod store_json_residue_certification;
pub(crate) mod store_json_residue_denial;
pub(crate) mod store_json_residue_entry;
pub(crate) mod store_json_residue_inventory;
pub(crate) mod store_json_residue_prelude_scan;
pub(crate) mod store_json_residue_scan;
#[cfg(test)]
pub(crate) mod store_json_residue_tests;

pub use aspect_native_boundary_audit::AspectNativeRejectedInputKind;
pub(crate) use aspect_native_boundary_audit::{
    audit_current_aspect_native_boundaries, AspectNativeBoundaryAudit,
    AspectNativeBoundaryAuditDenial,
};
pub use aspect_native_boundary_handoff::{
    accept_aspect_native_boundary_handoff, reconstruct_aspect_native_boundary_verdict,
    reject_terminal_json_projection_as_boundary_handoff, AspectNativeBoundaryHandoff,
    AspectNativeBoundaryHandoffDenial, AspectNativeBoundaryHandoffVerdict,
};
