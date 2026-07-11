#![forbid(unsafe_code)]
//!
//! ```compile_fail
//! use forge_store_readiness::FoundationalVocabularyAdoptionMap;
//!
//! struct LocalLookalike;
//!
//! fn requires_public_foundational_adoption(_: FoundationalVocabularyAdoptionMap) {}
//!
//! requires_public_foundational_adoption(LocalLookalike);
//! ```
//!
//! S.1 closeout receipts cannot be synthesized from raw fields:
//!
//! ```compile_fail
//! use forge_store_contracts::ROADMAP_2_S1_SCOPE;
//! use forge_store_readiness::PhysicalSubstrateCloseoutReceipt;
//!
//! let _forged = PhysicalSubstrateCloseoutReceipt {
//!     scope: ROADMAP_2_S1_SCOPE,
//!     evidence: todo!(),
//! };
//! ```
//!
//! S.2 readiness facts are not public count authority:
//!
//! ```compile_fail
//! use forge_store_readiness::PhysicalSubstrateReadinessFacts;
//!
//! let _forged = PhysicalSubstrateReadinessFacts::from_physical_format_closeout_counts(4, 2, 2, 3, 1, 9);
//! ```
//!
//! S.2 handoff evidence cannot be synthesized by ordinary callers:
//!
//! ```compile_fail
//! use forge_store_readiness::{
//!     PhysicalSubstrateEvidenceCounts, PhysicalSubstrateHandoffEvidence,
//! };
//! ```
//!
//! S.2 readiness is not minted from arbitrary fact bags:
//!
//! ```compile_fail
//! use forge_store_contracts::ROADMAP_2_S1_SCOPE;
//! use forge_store_readiness::{PhysicalSubstrateReadinessFacts, PhysicalSubstrateReadiness};
//!
//! let facts: PhysicalSubstrateReadinessFacts = todo!();
//! let _forged = PhysicalSubstrateReadiness::from_admitted_physical_substrate_closeout(
//!     ROADMAP_2_S1_SCOPE,
//!     facts,
//! );
//! ```
//!
//! S.2 readiness cannot be minted directly from S.0-to-S.1 handoff readiness:
//!
//! ```compile_fail
//! use forge_store_contracts::AcceptedHandoffReadiness;
//! use forge_store_readiness::prove_physical_substrate_readiness;
//!
//! let readiness: AcceptedHandoffReadiness = todo!();
//! let _forged = prove_physical_substrate_readiness(readiness);
//! ```
//!
//! S.3 physical integrity readiness cannot be synthesized from raw fields:
//!
//! ```compile_fail
//! use forge_store_readiness::PhysicalIntegrityReadiness;
//!
//! let _forged = PhysicalIntegrityReadiness {
//!     physical_substrate_readiness: todo!(),
//!     payload: todo!(),
//! };
//! ```
//!
//! S.3 physical integrity readiness cannot be copied and replayed:
//!
//! ```compile_fail
//! use forge_store_readiness::PhysicalIntegrityReadiness;
//!
//! fn copy_readiness(
//!     readiness: PhysicalIntegrityReadiness,
//! ) -> (PhysicalIntegrityReadiness, PhysicalIntegrityReadiness) {
//!     (readiness, readiness)
//! }
//! ```
//!
mod adoption_denial;
mod aspect_native_vocabulary_readiness;
mod evidence_fields;
#[cfg(test)]
mod evidence_fields_tests;
mod foundational_adoption;
mod foundational;
mod foundational_lanes;
mod proof_vocabulary;
mod physical_integrity;
mod physical_substrate;

pub use adoption_denial::FoundationalAdoptionDenial;
pub use aspect_native_vocabulary_readiness::{
    AspectNativeVocabularyFamily, AspectNativeVocabularyPosture,
    StoreAspectNativeVocabularyReadiness,
};
pub use evidence_fields::PhysicalFoundationEvidenceField;
pub use foundational_adoption::{
    FoundationalAdoptionFamily, FoundationalAdoptionRow, FoundationalAdoptionStatus,
    FoundationalVocabularyAdoptionMap, FoundationalVocabularyAdoptionMapBuilder,
};
pub use foundational_lanes::FoundationalPublicLaneSet;
pub use proof_vocabulary::{FoundationalAdoptionDigest, ProofVocabularyAdoptionMap};
pub use foundational::{
    accept_aspect_native_gate_handoff, reconstruct_aspect_native_handoff_verdict,
    reject_terminal_json_projection_as_aspect_native_handoff, AspectNativeGateHandoff,
    AspectNativeGateHandoffDenial, AspectNativeGateHandoffVerdict,
};
pub use physical_substrate::{
    close_physical_substrate_readiness, prove_physical_substrate_readiness,
    PhysicalSubstrateCloseoutReceipt,
};
pub use physical_substrate::{PhysicalSubstrateReadiness, PhysicalSubstrateReadinessDenial, PhysicalSubstrateReadinessDenialKind};
pub use physical_substrate::{
    PhysicalSubstrateReadinessFact, PhysicalSubstrateReadinessFacts, PhysicalSubstrateReadinessFactPosture,
};
pub use physical_integrity::PhysicalIntegrityReadiness;
