pub(super) use super::durability::{
    begin_durability_fixture, durable_record_binding, durable_record_binding_for_store,
    wal_artifact_observation, wal_scope,
};
pub(super) use worth_store_budgets::PreExecutionBudgetEnvelope;
pub(super) use worth_store_contracts::WalRecordFamily;
pub(super) use worth_store_security::{StoreKeyVersionPosture, StoreLegacySecurityPosture};

#[cfg(test)]
pub(super) use worth_store_layout_indexes::{
    layout_lsm_maintenance, lsm_strategy, BaselineLsmExecutionAdmissionDenial,
    LsmCompactionAdmissionRequest, LsmStrategy,
};
#[cfg(test)]
pub(super) use worth_store_lsm_authority::{LsmMembershipKey, LsmMembershipRecord};
#[cfg(test)]
pub(super) use worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test;
#[cfg(test)]
pub(super) use worth_store_wal::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind, PublicationDeclaration,
    StoreWalRecordIdentity, WalFrameArtifactObservation,
};

#[cfg(test)]
pub(super) fn open_lsm_index(
    anchor: &WalFrameArtifactObservation,
) -> Result<worth_store_lsm_authority::LsmMembershipSession, BaselineLsmExecutionAdmissionDenial> {
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    lsm_strategy().open_index(anchor, security.witnesses())
}
