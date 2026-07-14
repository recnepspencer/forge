//! Aspect-native Store boundary vocabulary.
//!
//! This crate is the ordinary Store workspace lane for boundary facts that need
//! Foundational aspect vocabulary plus Store-owned physical witnesses.
//!
//! Raw `AspectValue`, raw `StructAspectValue`, strings, dotted path text, and
//! terminal projections are not authority inputs:
//!
//! ```compile_fail
//! use worth_foundational::AspectValue;
//! use worth_store_aspect_native::StoreAspectBoundaryFact;
//!
//! let raw = AspectValue::Null;
//! let _fact = StoreAspectBoundaryFact::from_raw_value(raw);
//! ```
//!
//! ```compile_fail
//! use worth_store_aspect_native::StoreAspectIdentity;
//!
//! let _identity = StoreAspectIdentity::from("store.physical.segment.identity");
//! ```
//!
//! ```compile_fail
//! use worth_foundational::{aspects, AspectValue};
//! use worth_store_aspect_native::StoreAspectAuthorityInput;
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

mod authority;
mod boundary_locators;
mod canonical_basis;
mod contract_admission;
mod denial;
mod equivalence_basis;
mod handoff;
mod json_ingress_readmission;
mod physical_witness;
mod receipts;
mod terminal_json_projection;
mod terminal_projection;
mod value_admission;

pub use authority::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact, StoreDigestAuthority,
    StoreDigestAuthorityDenial, StoreDigestAuthorityOutcome, StoreDigestEvidence,
};
pub use boundary_locators::{
    StoreAspectBoundaryLocator, StoreAspectFieldBoundaryLocator, StoreAspectValueBoundaryLocator,
    StoreBoundaryArtifactBoundaryLocator,
};
pub use canonical_basis::{
    canonical_basis_source_owner_for_family, certify_canonical_basis_field_role,
    certify_canonical_basis_source, StoreCanonicalBasisConstruction,
    StoreCanonicalBasisConstructionDenial, StoreCanonicalBasisConstructionOutcome,
    StoreCanonicalBasisDomainMismatch, StoreCanonicalBasisFamily, StoreCanonicalBasisFieldRole,
    StoreCanonicalBasisLane, StoreCanonicalBasisSourceDenial, StoreCanonicalBasisSourceKind,
    StoreCanonicalBasisSourceOwner, STORE_CANONICAL_BASIS_SOURCE_OWNERS,
};
pub use contract_admission::StoreAspectContractAdmission;
pub use denial::StoreAspectNativeDenial;
pub use equivalence_basis::{
    deny_basis_free_digest_comparison, deny_basis_free_parity, deny_basis_free_reuse,
    deny_basis_free_suppression, StoreDigestEquivalenceBasis, StoreDigestEquivalenceDecision,
    StoreDigestEquivalenceDenial, StoreDigestEquivalenceOperation, StoreDigestEquivalenceOutcome,
    StoreEquivalenceBasisIdentity,
};
pub use handoff::{StoreReadinessHandoffArtifact, StoreReadinessHandoffDenial};
pub use json_ingress_readmission::{
    readmit_external_terminal_projection_document_as_store_aspect_state,
    readmit_terminal_json_projection_as_store_aspect_state, StoreTerminalJsonReadmission,
    StoreTerminalJsonReadmissionOutcome,
};
pub use physical_witness::StorePhysicalBoundaryWitness;
pub use receipts::{
    StoreCompletedBoundaryReceiptEvidence, StoreDiagnosticExplanationBundleEvidence,
    StoreDiagnosticSupportReportEvidence, StoreExecutedBoundaryReceiptEvidence,
    StorePerformanceReceiptEvidence,
};
pub use terminal_json_projection::{
    project_store_boundary_fact_to_terminal_json, StoreTerminalJsonProjection,
};
pub use terminal_projection::{
    StoreTerminalChecksumAlgorithm, StoreTerminalChecksumScope, StoreTerminalDocumentChecksum,
    StoreTerminalProjectionDenial, StoreTerminalProjectionDisplayLabel,
    StoreTerminalProjectionDocumentBytes, StoreTerminalProjectionText,
};
pub use value_admission::StoreValidatedAspectValueAdmission;
