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
//! use forge_store_readiness::S1PhysicalSubstrateCloseoutReceipt;
//!
//! let _forged = S1PhysicalSubstrateCloseoutReceipt {
//!     scope: ROADMAP_2_S1_SCOPE,
//!     evidence: todo!(),
//! };
//! ```
//!
//! S.2 readiness facts are not public count authority:
//!
//! ```compile_fail
//! use forge_store_readiness::S2PhysicalReadinessFacts;
//!
//! let _forged = S2PhysicalReadinessFacts::from_s1_closeout_counts(4, 2, 2, 3, 1, 9);
//! ```
//!
//! S.2 handoff evidence cannot be synthesized by ordinary callers:
//!
//! ```compile_fail
//! use forge_store_readiness::{
//!     S2PhysicalSubstrateEvidenceCounts, S2PhysicalSubstrateHandoffEvidence,
//! };
//! ```
//!
//! S.2 readiness is not minted from arbitrary fact bags:
//!
//! ```compile_fail
//! use forge_store_contracts::ROADMAP_2_S1_SCOPE;
//! use forge_store_readiness::{S2PhysicalReadinessFacts, S2PhysicalSubstrateReadiness};
//!
//! let facts: S2PhysicalReadinessFacts = todo!();
//! let _forged = S2PhysicalSubstrateReadiness::from_admitted_physical_substrate_closeout(
//!     ROADMAP_2_S1_SCOPE,
//!     facts,
//! );
//! ```
//!
//! S.2 readiness cannot be minted directly from S.0-to-S.1 handoff readiness:
//!
//! ```compile_fail
//! use forge_store_contracts::AcceptedHandoffReadiness;
//! use forge_store_readiness::prove_s2_physical_substrate_readiness;
//!
//! let readiness: AcceptedHandoffReadiness = todo!();
//! let _forged = prove_s2_physical_substrate_readiness(readiness);
//! ```
//!
//! S.3 physical integrity readiness is not a readiness-crate authority surface:
//!
//! ```compile_fail
//! use forge_store_readiness::S3PhysicalIntegrityReadiness;
//!
//! let _forged: S3PhysicalIntegrityReadiness = todo!();
//! ```

mod adoption_denial;
mod evidence_fields;
#[cfg(test)]
mod evidence_fields_tests;
mod foundational_adoption;
mod foundational_lanes;
mod proof_vocabulary;
mod s2_physical_substrate_proof;
mod s2_physical_substrate_readiness;
mod s2_readiness_denial;
mod s2_readiness_facts;
mod s3_readiness_denial;
mod s3_readiness_payload;
mod s3_readiness_recap;

pub use adoption_denial::FoundationalAdoptionDenial;
pub use evidence_fields::PhysicalFoundationEvidenceField;
pub use foundational_adoption::{
    FoundationalAdoptionFamily, FoundationalAdoptionRow, FoundationalAdoptionStatus,
    FoundationalVocabularyAdoptionMap, FoundationalVocabularyAdoptionMapBuilder,
};
pub use foundational_lanes::FoundationalPublicLaneSet;
pub use proof_vocabulary::{FoundationalAdoptionDigest, ProofVocabularyAdoptionMap};
pub use s2_physical_substrate_proof::{
    close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
    S1PhysicalSubstrateCloseoutReceipt,
};
pub use s2_physical_substrate_readiness::S2PhysicalSubstrateReadiness;
pub use s2_readiness_denial::{S2ReadinessDenial, S2ReadinessDenialKind};
pub use s2_readiness_facts::{
    S2PhysicalReadinessFact, S2PhysicalReadinessFacts, S2ReadinessFactPosture,
};
pub use s3_readiness_denial::{S3ReadinessDenial, S3ReadinessDenialKind};
pub use s3_readiness_payload::{
    IntegrityInspectionLifetimeLaw, ProtectedIntegrityViewCapability, S2NoMaterializationWitness,
    S3PhysicalIntegrityReadinessPayload, ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
pub use s3_readiness_recap::{
    BufferPoolAuthorityRecap, PhysicalAuthorityRecap, S2BoundedCounterRecap, S2DenialBehaviorRecap,
    S2DeniedBoundaryKind,
};
