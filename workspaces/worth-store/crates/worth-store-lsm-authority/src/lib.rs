#![forbid(unsafe_code)]

mod membership;
mod physical_binding;
mod replay_source;

pub use membership::{
    admit_lsm_membership_replacement, admit_lsm_replacement_output,
    lookup_published_lsm_membership, lsm_membership_owner_case_inventory, open_lsm_membership,
    persist_lsm_membership_record, prepare_lsm_membership_activation,
    reopen_lsm_membership_from_store, replace_lsm_membership, select_lsm_compaction_membership,
    AdmittedLsmMembershipReplacement, AdmittedLsmReplacementOutput, LsmCompactionMembership,
    LsmCompactionRecordIdentitySet, LsmCompactionRecordSet, LsmMembershipActivationDeclaration,
    LsmMembershipArtifactDeclaration, LsmMembershipDenial, LsmMembershipDisposition,
    LsmMembershipKey, LsmMembershipOpenOutcome, LsmMembershipOpenView, LsmMembershipOperation,
    LsmMembershipOwnerCaseDeclaration, LsmMembershipOwnerCaseId, LsmMembershipOwnerCaseObservation,
    LsmMembershipPersistOutcome, LsmMembershipPersistView, LsmMembershipRecord,
    LsmMembershipReopenCounters, LsmMembershipReplacementOutcome, LsmMembershipReplacementView,
    LsmMembershipReplayPosture, LsmMembershipSelectionOutcome, LsmMembershipSelectionView,
    LsmMembershipSession, LsmPublishedMembershipLookupOutcome, LsmPublishedMembershipLookupView,
    PublishedLsmMembershipIdentity, PublishedLsmMembershipReplacement,
};
pub use physical_binding::LsmPhysicalCompactionIntent;
pub use replay_source::{
    AdmittedLsmReplaySource, LsmReplayExecutionPlan, LsmReplaySourceDenial,
    LsmReplaySourceIdentity, LsmReplaySourceKind,
};

#[cfg(feature = "certification-test-authority")]
pub use membership::issue_published_lsm_membership_for_certification;

pub(crate) use worth_store_wal::{
    AdmittedCheckpointPublicationReceipt, AdmittedWalAppendReceipt, AdmittedWalArtifactStore,
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind,
    CheckpointDurablePublicationScope, DurablePublicationDeclaration, DurablePublicationScope,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope, WalSecurityMetadataCarrier,
};
