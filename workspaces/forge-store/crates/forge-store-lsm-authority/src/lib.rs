#![forbid(unsafe_code)]

mod membership;
mod physical_binding;
mod replay_source;

pub use membership::{
    admit_lsm_membership_replacement, admit_lsm_replacement_output,
    prepare_lsm_membership_activation, AdmittedLsmMembershipReplacement,
    AdmittedLsmReplacementOutput, LsmCompactionMembership, LsmMembershipActivationDeclaration,
    LsmMembershipArtifactDeclaration, LsmMembershipDenial, LsmMembershipKey, LsmMembershipRecord,
    LsmMembershipReopenCounters, LsmMembershipReplayPosture, LsmMembershipSession,
    PublishedLsmMembershipIdentity, PublishedLsmMembershipReplacement,
};
pub use physical_binding::LsmPhysicalCompactionIntent;
pub use replay_source::{
    AdmittedLsmReplaySource, LsmReplayExecutionPlan, LsmReplaySourceDenial,
    LsmReplaySourceIdentity, LsmReplaySourceKind,
};

pub(crate) use forge_store_wal::{
    AdmittedCheckpointPublicationReceipt, AdmittedWalAppendReceipt, AdmittedWalArtifactStore,
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind,
    CheckpointDurablePublicationScope, DurablePublicationDeclaration, DurablePublicationScope,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope, WalSecurityMetadataCarrier,
};
