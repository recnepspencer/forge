mod aspect_class;
mod error;
mod locality_identity;
mod overlap_category;
mod overlap_identity;
mod participant_identity;
mod prior_proof_input;
mod routing_contract;

pub use aspect_class::ConflictAspectClass;
pub use error::ConflictRoutingVocabularyError;
pub use locality_identity::{admit_conflict_locality_identity, ConflictLocalityIdentity};
pub use overlap_category::ConflictOverlapCategory;
pub use overlap_identity::{
    admit_conflict_overlap_identity, ConflictOverlapIdentity, ConflictOverlapIdentityInput,
};
pub use participant_identity::{
    admit_conflict_participant_identity, ConflictParticipantAuthority, ConflictParticipantIdentity,
    ConflictParticipantIdentityInput,
};
pub use prior_proof_input::{
    ConflictPriorProofIdentity, ConflictPriorProofInput, ConflictTransactionProofInput,
};
pub use routing_contract::{
    admit_conflict_routing_contract, ConflictRoutingContract, ConflictRoutingPosture,
};

#[cfg(feature = "conflict-routing-internal-authority")]
#[doc(hidden)]
pub mod internal_sources {
    pub use super::locality_identity::{
        admit_conflict_spatial_touch_authority_locality_identity_from_digest,
        admit_conflict_topology_touched_closure_locality_identity_from_digest,
    };
    pub use super::participant_identity::{
        admit_conflict_evidence_participant_identity_from_digest,
        admit_conflict_validator_participant_identity_from_digest,
    };
}
