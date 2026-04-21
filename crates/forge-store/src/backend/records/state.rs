use crate::wal::WalRecord;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::{
    AuthoritativeArtifactDigestRecord, BranchDeltaLayerRecord, BranchHeadRecord, BranchRecord,
    BranchSharedBaseRecord, BulkChunkWitnessRecord, BulkDeterministicPlanRecord,
    BulkProgramIdentityRecord, BulkProgressCheckpointRecord, CommitParentRecord,
    CommitSupportSummaryRecord, CompactionProductRecord, DurableCursorIdentityRecord,
    EmbeddedCheckpointRecord, FrozenBulkManifestRecord, FrozenTransformBasisRecord,
    FrozenTransformPartitionRecord, LineageSupportRecord, MaintenanceBatchRecord,
    MaintenanceCheckpointRecord, MaintenanceDebtSummaryRecord, MaintenanceDeclarationRecord,
    MaintenanceExecutionRecord, MaintenanceLocalitySummaryRecord, MaintenanceQueueSummaryRecord,
    MaintenanceReservationSummaryRecord, MaintenanceResourceBudgetSummaryRecord,
    Milestone6ChunkMembershipRecord, Milestone6CommitCoupledLayoutSeedRecord,
    Milestone6LayoutMaterializationRecord, Milestone6ScopeSliceMembershipRecord,
    Milestone6StructuralBlockRecord, ProgramChunkWitnessIndexRecord, RebuildDebtRecord,
    RetentionBasisRecord, RetentionClosureRecord, SchemaSupportRecord, SnapshotBasisRecord,
    SnapshotImageRecord, StableBasisRecord, StoredCommitEnvelope, SubscriberCheckpointRecord,
    TierRecallRecord, TierResidencyRecord, TierTransferRecord,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StoreState {
    pub canonicalization_version: u32,
    pub next_commit_sequence: u64,
    pub next_head_update_sequence: u64,
    pub branch_records: BTreeMap<String, BranchRecord>,
    pub branch_head_records: BTreeMap<String, BranchHeadRecord>,
    pub commit_envelopes: BTreeMap<u64, StoredCommitEnvelope>,
    pub commit_parent_records: BTreeMap<String, CommitParentRecord>,
    pub authoritative_artifact_digests: BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    #[serde(default)]
    pub commit_support_summaries: BTreeMap<u64, CommitSupportSummaryRecord>,
    #[serde(default)]
    pub schema_support_records: BTreeMap<String, SchemaSupportRecord>,
    #[serde(default)]
    pub lineage_support_records: BTreeMap<String, LineageSupportRecord>,
    #[serde(default)]
    pub durable_cursor_identity_records: BTreeMap<String, DurableCursorIdentityRecord>,
    #[serde(default)]
    pub subscriber_checkpoint_records: BTreeMap<String, SubscriberCheckpointRecord>,
    #[serde(default)]
    pub stable_basis_records: BTreeMap<String, StableBasisRecord>,
    #[serde(default)]
    pub compaction_product_records: BTreeMap<String, CompactionProductRecord>,
    #[serde(default)]
    pub retention_basis_records: BTreeMap<String, RetentionBasisRecord>,
    #[serde(default)]
    pub retention_closure_records: BTreeMap<String, RetentionClosureRecord>,
    #[serde(default)]
    pub rebuild_debt_records: BTreeMap<String, RebuildDebtRecord>,
    #[serde(default)]
    pub next_maintenance_declaration_order: u64,
    #[serde(default)]
    pub next_maintenance_checkpoint_order: u64,
    #[serde(default)]
    pub maintenance_declaration_records: BTreeMap<String, MaintenanceDeclarationRecord>,
    #[serde(default)]
    pub maintenance_execution_records: BTreeMap<String, MaintenanceExecutionRecord>,
    #[serde(default)]
    pub maintenance_batch_records: BTreeMap<String, MaintenanceBatchRecord>,
    #[serde(default)]
    pub maintenance_checkpoint_records: BTreeMap<String, MaintenanceCheckpointRecord>,
    #[serde(default)]
    pub maintenance_queue_summary_records: BTreeMap<String, MaintenanceQueueSummaryRecord>,
    #[serde(default)]
    pub maintenance_locality_summary_records: BTreeMap<String, MaintenanceLocalitySummaryRecord>,
    #[serde(default)]
    pub maintenance_reservation_summary_records:
        BTreeMap<String, MaintenanceReservationSummaryRecord>,
    #[serde(default)]
    pub maintenance_resource_budget_summary_records:
        BTreeMap<String, MaintenanceResourceBudgetSummaryRecord>,
    #[serde(default)]
    pub maintenance_debt_summary_records: BTreeMap<String, MaintenanceDebtSummaryRecord>,
    #[serde(default)]
    pub maintenance_loaded_persisted_summaries_on_boot: bool,
    #[serde(default)]
    pub maintenance_used_legacy_summary_backfill_on_boot: bool,
    #[serde(default)]
    pub maintenance_recovered_backlog_on_boot: u64,
    #[serde(default)]
    pub maintenance_boot_integrity_reject_count: u64,
    #[serde(default)]
    pub branch_shared_base_records: BTreeMap<String, BranchSharedBaseRecord>,
    #[serde(default)]
    pub next_branch_delta_layer_id: u64,
    #[serde(default)]
    pub branch_delta_layer_records: BTreeMap<u64, BranchDeltaLayerRecord>,
    #[serde(default)]
    pub embedded_checkpoint_records: BTreeMap<String, EmbeddedCheckpointRecord>,
    #[serde(default)]
    pub milestone_6_layout_materialization_records:
        BTreeMap<String, Milestone6LayoutMaterializationRecord>,
    #[serde(default)]
    pub milestone_6_commit_coupled_layout_seed_records:
        BTreeMap<String, Milestone6CommitCoupledLayoutSeedRecord>,
    #[serde(default)]
    pub milestone_6_scope_slice_membership_records:
        BTreeMap<String, Milestone6ScopeSliceMembershipRecord>,
    #[serde(default)]
    pub milestone_6_chunk_membership_records: BTreeMap<String, Milestone6ChunkMembershipRecord>,
    #[serde(default)]
    pub milestone_6_structural_block_records: BTreeMap<String, Milestone6StructuralBlockRecord>,
    #[serde(default)]
    pub bulk_program_identity_records: BTreeMap<String, BulkProgramIdentityRecord>,
    #[serde(default)]
    pub frozen_bulk_manifest_records: BTreeMap<String, FrozenBulkManifestRecord>,
    #[serde(default)]
    pub frozen_transform_basis_records: BTreeMap<String, FrozenTransformBasisRecord>,
    #[serde(default)]
    pub frozen_transform_partition_records: BTreeMap<String, FrozenTransformPartitionRecord>,
    #[serde(default)]
    pub bulk_deterministic_plan_records: BTreeMap<String, BulkDeterministicPlanRecord>,
    #[serde(default)]
    pub bulk_progress_checkpoint_records: BTreeMap<String, BulkProgressCheckpointRecord>,
    #[serde(default)]
    pub bulk_chunk_witness_records: BTreeMap<String, BulkChunkWitnessRecord>,
    #[serde(default)]
    pub program_chunk_witness_index_records: BTreeMap<String, ProgramChunkWitnessIndexRecord>,
    #[serde(default)]
    pub next_snapshot_id: u64,
    #[serde(default)]
    pub snapshot_basis_records: BTreeMap<u64, SnapshotBasisRecord>,
    #[serde(default)]
    pub snapshot_image_records: BTreeMap<u64, SnapshotImageRecord>,
    #[serde(default)]
    pub tier_residency_records: BTreeMap<String, TierResidencyRecord>,
    #[serde(default)]
    pub tier_transfer_records: BTreeMap<String, TierTransferRecord>,
    #[serde(default)]
    pub tier_recall_records: BTreeMap<String, TierRecallRecord>,
    #[serde(default)]
    pub next_durable_mutation_id: u64,
    #[serde(default)]
    pub next_wal_sequence: u64,
    #[serde(default)]
    pub wal_records: BTreeMap<u64, WalRecord>,
}
