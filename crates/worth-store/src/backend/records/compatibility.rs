use crate::compatibility::{
    ArtifactFamilyId, CompatibilityManifestPublicationLedger,
    CompatibilityManifestPublicationRecord, CompatibilityManifestSummary,
    CompatibilityRecoveredManifestIndex, CompatibilityRegistry, ManifestRecoverySummary,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub(crate) const COMPATIBILITY_MANIFEST_RECORD_FAMILY_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityManifestRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub record: CompatibilityManifestPublicationRecord,
}

impl CompatibilityManifestRecord {
    pub(crate) fn from_publication_record(record: CompatibilityManifestPublicationRecord) -> Self {
        Self {
            artifact_id: compatibility_manifest_artifact_id(record.family_id()),
            family_version: COMPATIBILITY_MANIFEST_RECORD_FAMILY_VERSION,
            record,
        }
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }
}

pub(crate) fn compatibility_manifest_artifact_id(family_id: &ArtifactFamilyId) -> String {
    format!("compatibility_manifest:{}", family_id.as_str())
}

pub(crate) fn first_ship_compatibility_manifest_records(
) -> BTreeMap<String, CompatibilityManifestRecord> {
    let snapshot = CompatibilityRegistry::first_ship();
    let mut ledger = CompatibilityManifestPublicationLedger::new();
    for declaration in snapshot.declarations() {
        ledger.publish_declaration(declaration);
    }
    ledger
        .records()
        .iter()
        .cloned()
        .map(CompatibilityManifestRecord::from_publication_record)
        .map(|record| (record.artifact_id.clone(), record))
        .collect()
}

pub(crate) fn recovered_compatibility_manifest_index_from_records(
    records: &BTreeMap<String, CompatibilityManifestRecord>,
) -> CompatibilityRecoveredManifestIndex {
    CompatibilityManifestPublicationLedger::from_records(
        records
            .values()
            .map(|record| record.record.clone())
            .collect(),
    )
    .recover()
}

pub(crate) fn manifest_recovery_summary_from_records(
    records: &BTreeMap<String, CompatibilityManifestRecord>,
) -> ManifestRecoverySummary {
    let recovered = recovered_compatibility_manifest_index_from_records(records);
    let recovered_count = recovered.records().count() as u64;
    let expected_count = CompatibilityRegistry::first_ship().declarations().len() as u64;
    ManifestRecoverySummary::new(
        expected_count,
        recovered_count,
        expected_count.saturating_sub(recovered_count),
    )
}

pub(crate) fn compatibility_manifest_summaries_from_records(
    records: &BTreeMap<String, CompatibilityManifestRecord>,
) -> Vec<CompatibilityManifestSummary> {
    records
        .values()
        .map(|record| {
            CompatibilityManifestSummary::new(
                record.record.family_id().clone(),
                record.record.manifest_digest().clone(),
            )
        })
        .collect()
}

impl super::StoreState {
    pub(crate) fn initialize_pristine_compatibility_manifests_if_missing(&mut self) {
        if self.compatibility_manifest_records.is_empty() && !self.has_non_compatibility_artifacts()
        {
            self.compatibility_manifest_records = first_ship_compatibility_manifest_records();
        }
    }

    pub(crate) fn initialize_restored_compatibility_manifests_if_missing(&mut self) {
        if self.compatibility_manifest_records.is_empty() {
            self.compatibility_manifest_records = first_ship_compatibility_manifest_records();
        }
    }

    pub(crate) fn recovered_compatibility_manifest_index(
        &self,
    ) -> CompatibilityRecoveredManifestIndex {
        recovered_compatibility_manifest_index_from_records(&self.compatibility_manifest_records)
    }

    pub(crate) fn compatibility_manifest_recovery_summary(&self) -> ManifestRecoverySummary {
        manifest_recovery_summary_from_records(&self.compatibility_manifest_records)
    }

    pub(crate) fn compatibility_manifest_summaries(&self) -> Vec<CompatibilityManifestSummary> {
        compatibility_manifest_summaries_from_records(&self.compatibility_manifest_records)
    }

    fn has_non_compatibility_artifacts(&self) -> bool {
        !self.branch_records.is_empty()
            || !self.branch_head_records.is_empty()
            || !self.commit_envelopes.is_empty()
            || !self.commit_parent_records.is_empty()
            || !self.authoritative_artifact_digests.is_empty()
            || !self.commit_support_summaries.is_empty()
            || !self.schema_support_records.is_empty()
            || !self.lineage_support_records.is_empty()
            || !self.durable_cursor_identity_records.is_empty()
            || !self.subscriber_checkpoint_records.is_empty()
            || !self.stable_basis_records.is_empty()
            || !self.compaction_product_records.is_empty()
            || !self.retention_basis_records.is_empty()
            || !self.retention_closure_records.is_empty()
            || !self.rebuild_debt_records.is_empty()
            || !self.maintenance_declaration_records.is_empty()
            || !self.maintenance_execution_records.is_empty()
            || !self.maintenance_batch_records.is_empty()
            || !self.maintenance_checkpoint_records.is_empty()
            || !self.maintenance_queue_summary_records.is_empty()
            || !self.maintenance_locality_summary_records.is_empty()
            || !self.maintenance_reservation_summary_records.is_empty()
            || !self.maintenance_resource_budget_summary_records.is_empty()
            || !self.maintenance_debt_summary_records.is_empty()
            || !self.branch_shared_base_records.is_empty()
            || !self.branch_delta_layer_records.is_empty()
            || !self.embedded_checkpoint_records.is_empty()
            || !self.milestone_6_layout_materialization_records.is_empty()
            || !self
                .milestone_6_commit_coupled_layout_seed_records
                .is_empty()
            || !self.milestone_6_scope_slice_membership_records.is_empty()
            || !self.milestone_6_chunk_membership_records.is_empty()
            || !self.milestone_6_structural_block_records.is_empty()
            || !self.bulk_program_identity_records.is_empty()
            || !self.frozen_bulk_manifest_records.is_empty()
            || !self.frozen_transform_basis_records.is_empty()
            || !self.frozen_transform_partition_records.is_empty()
            || !self.bulk_deterministic_plan_records.is_empty()
            || !self.bulk_progress_checkpoint_records.is_empty()
            || !self.bulk_chunk_witness_records.is_empty()
            || !self.program_chunk_witness_index_records.is_empty()
            || !self.snapshot_basis_records.is_empty()
            || !self.snapshot_image_records.is_empty()
            || !self.tier_residency_records.is_empty()
            || !self.tier_transfer_records.is_empty()
            || !self.tier_recall_records.is_empty()
            || !self.wal_records.is_empty()
    }
}
