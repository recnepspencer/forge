//! Aspect-native Store boundary vocabulary.
//!
//! This crate is the ordinary Store workspace lane for boundary facts that need
//! Foundational aspect vocabulary plus Store-owned physical witnesses.
//!
//! Raw `AspectValue`, raw `StructAspectValue`, strings, dotted path text, and
//! terminal projections are not authority inputs:
//!
//! ```compile_fail
//! use forge_foundational::AspectValue;
//! use forge_store_aspect_native::StoreAspectBoundaryFact;
//!
//! let raw = AspectValue::Null;
//! let _fact = StoreAspectBoundaryFact::from_raw_value(raw);
//! ```
//!
//! ```compile_fail
//! use forge_store_aspect_native::StoreAspectIdentity;
//!
//! let _identity = StoreAspectIdentity::from("store.physical.segment.identity");
//! ```
//!
//! ```compile_fail
//! use forge_foundational::{aspects, AspectValue};
//! use forge_store_aspect_native::StoreAspectAuthorityInput;
//!
//! let raw_struct = aspects()
//!     .vocabulary()
//!     .struct_value()
//!     .with_field("segment", AspectValue::Null)
//!     .finish()
//!     .unwrap();
//!
//! let _authority = StoreAspectAuthorityInput::new(raw_struct);
//! ```

#![forbid(unsafe_code)]

mod authoritative_patch;
mod authoritative_state;
mod boundary_locators;
mod canonical_basis;
mod canonical_basis_sources;
mod contract_admission;
mod denial;
mod evidence_receipts;
mod identity_authority;
mod json_ingress_readmission;
mod performance_receipts;
mod physical_witness;
mod terminal_json_projection;
mod terminal_projection;
mod terminal_projection_denial;
mod value_admission;

pub use authoritative_patch::{StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact};
pub use authoritative_state::{StoreAspectAuthorityInput, StoreAspectBoundaryFact};
pub use boundary_locators::{
    StoreAspectBoundaryLocator, StoreAspectFieldBoundaryLocator, StoreAspectValueBoundaryLocator,
    StoreBoundaryArtifactBoundaryLocator,
};
pub use canonical_basis::{
    StoreCanonicalBasisFamily, StoreCanonicalBasisFieldRole, StoreCanonicalBasisLane,
    StoreCanonicalBasisSourceDenial, StoreCanonicalBasisSourceKind, StoreCanonicalBasisSourceOwner,
};
pub use canonical_basis_sources::{
    canonical_basis_source_owner_for_family, certify_canonical_basis_field_role,
    certify_canonical_basis_source, STORE_CANONICAL_BASIS_SOURCE_OWNERS,
};
pub use contract_admission::StoreAspectContractAdmission;
pub use denial::StoreAspectNativeDenial;
pub use evidence_receipts::{
    StoreCompletedBoundaryReceiptEvidence, StoreDiagnosticExplanationBundleEvidence,
    StoreDiagnosticSupportReportEvidence, StoreExecutedBoundaryReceiptEvidence,
};
pub use identity_authority::StoreAspectIdentity;
pub use json_ingress_readmission::{
    readmit_external_terminal_projection_document_as_store_aspect_state,
    readmit_terminal_json_projection_as_store_aspect_state, StoreTerminalJsonReadmission,
    StoreTerminalJsonReadmissionOutcome,
};
pub use performance_receipts::StorePerformanceReceiptEvidence;
pub use physical_witness::StorePhysicalBoundaryWitness;
pub use terminal_json_projection::{
    project_store_boundary_fact_to_terminal_json, StoreTerminalJsonProjection,
};
pub use terminal_projection::StoreTerminalProjectionText;
pub use terminal_projection_denial::StoreTerminalProjectionDenial;
pub use value_admission::StoreValidatedAspectValueAdmission;
