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
//! S.2 physical substrate readiness cannot be promoted from readiness descriptors:
//!
//! ```compile_fail
//! use forge_store_readiness::S1PhysicalSubstrateCloseoutReceipt;
//! ```

mod adoption_denial;
mod evidence_fields;
mod foundational_adoption;
mod foundational_lanes;
mod proof_vocabulary;

pub use adoption_denial::FoundationalAdoptionDenial;
pub use evidence_fields::PhysicalFoundationEvidenceField;
pub use foundational_adoption::{
    FoundationalAdoptionFamily, FoundationalAdoptionRow, FoundationalAdoptionStatus,
    FoundationalVocabularyAdoptionMap, FoundationalVocabularyAdoptionMapBuilder,
};
pub use foundational_lanes::FoundationalPublicLaneSet;
pub use proof_vocabulary::{FoundationalAdoptionDigest, ProofVocabularyAdoptionMap};
