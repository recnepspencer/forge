#![forbid(unsafe_code)]
//!
//! Raw replication declarations cannot become admitted source authority:
//!
//! ```compile_fail
//! use worth_store_replication::{AdmittedReplicationSource, ReplicationSourceDeclaration};
//!
//! let raw: ReplicationSourceDeclaration = todo!();
//! let _admitted: AdmittedReplicationSource = raw;
//! ```
//!
//! Copied capsule identifiers cannot become publication readiness:
//!
//! ```compile_fail
//! use worth_store_replication::{ReplicationCapsuleId, ReplicationPublicationReadiness};
//!
//! let raw = ReplicationCapsuleId(7);
//! let _readiness: ReplicationPublicationReadiness = raw;
//! ```
//!
//! Peer progress observations cannot mint a published replication outcome:
//!
//! ```compile_fail
//! use worth_store_replication::{PublishedReplication, ReplicationAdmissionObservation};
//!
//! let observed: ReplicationAdmissionObservation = todo!();
//! let _published: PublishedReplication = observed;
//! ```
//!
//! An admitted source cannot skip the owner-issued progress phase:
//!
//! ```compile_fail
//! use worth_store_replication::{AdmittedReplicationSource, ReplicationPublicationReadiness};
//!
//! let admitted: AdmittedReplicationSource = todo!();
//! let _readiness: ReplicationPublicationReadiness = admitted;
//! ```
//!
//! Generic transition outcomes cannot impersonate owner-issued outcomes:
//!
//! ```compile_fail
//! use worth_proof::TransitionOutcome;
//! use worth_store_replication::{
//!     ReplicationSourceAdmissionDenial, ReplicationSourceAdmissionOutcome,
//! };
//!
//! let forged = TransitionOutcome::denied(
//!     ReplicationSourceAdmissionDenial::ReplayIdentityMismatch,
//! );
//! let _owner_outcome: ReplicationSourceAdmissionOutcome = forged;
//! ```

mod admission;
mod bootstrap;
mod disaster_recovery;
mod divergence;
mod identity;
mod observation;
mod progress;
mod progress_store;
mod promotion;
mod publication;
mod rejoin;
mod runtime;
#[cfg(test)]
mod tests;

pub use admission::{
    admit_replication_source, AdmittedReplicationSource, ReplicationSourceAdmissionDenial,
    ReplicationSourceAdmissionOutcome, ReplicationSourceAdmissionOutcomeView,
    ReplicationSourceDeclaration,
};
pub use bootstrap::{
    LoweredReplicaBootstrapPlan, ReplicaBootstrapDenial, ReplicaBootstrapExecutionPort,
    ReplicaBootstrapExecutionReport, ReplicaBootstrapIntent, ReplicaBootstrapOwner,
    ReplicaBootstrapReceipt,
};
pub use disaster_recovery::{
    DisasterRecoveryBundleDenial, DisasterRecoveryComponent, DisasterRecoveryComponentFamily,
    MaterializedDisasterRecoveryBundle, ReplicationDisasterRecoveryOwner,
};
pub use divergence::{
    DivergentReplicaHistoryReport, ReplicaHistoryClassification, ReplicaHistoryObservation,
    ReplicaRecoveryFrontier, ReplicaRecoveryFrontierDenial,
};
pub use promotion::{
    LoweredReplicaPromotionPlan, ReplicaPromotionCandidate, ReplicaPromotionDenial,
    ReplicaPromotionIntent, ReplicaPromotionOwner, ReplicaPromotionReceipt,
};
pub use rejoin::{
    OldPrimaryDivergenceDisposition, OldPrimaryRejoinDenial, OldPrimaryRejoinPlan,
    ReplicationRejoinOwner,
};
pub use identity::{
    ReplicationCapsuleId, ReplicationLineageIdentity, ReplicationPeerId, ReplicationSourceEpoch,
};
pub use observation::{
    ObserveReplicationAdmission, ReplicationAdmissionObservation, ReplicationAdmissionStage,
};
pub use progress::{
    admit_replication_publication_readiness, ObservedReplicationProgress, ReplicationDeliveryKind,
    ReplicationDuplicateDelivery, ReplicationPeerProgress, ReplicationProgressDenial,
    ReplicationProgressInterruption, ReplicationProgressOutcome, ReplicationProgressOutcomeView,
};
pub use progress_store::ReplicationPeerCapacity;
pub use publication::{
    PublishedReplication, ReplicationPublicationDenial, ReplicationPublicationOutcome,
    ReplicationPublicationOutcomeView, ReplicationPublicationReadiness,
};
pub use runtime::ReplicationAdmissionRuntime;
