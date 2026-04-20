use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        integrity::{
            durable_cursor_identity_artifact_id, retention_basis_artifact_id,
            retention_closure_artifact_id, subscriber_checkpoint_artifact_id,
        },
        records::{RetentionBasisRecord, StoreState},
    },
    retention::{
        CompactionPlan, RetainedReadCostSurface, RetainedReadPath, RetentionClosureSummary,
    },
    snapshot::SnapshotId,
};
use forge_relational::facade::history::{BranchId, CommitId};

pub(super) fn branch_basis_label(branch_id: &BranchId, commit_id: CommitId) -> String {
    format!("branch:{}@{}", branch_id.0, commit_id.0)
}

pub(super) fn snapshot_basis_label(snapshot_id: SnapshotId) -> String {
    format!("snapshot:{}", snapshot_id.0)
}

pub(super) fn durable_cursor_basis_label(cursor_id: &str) -> String {
    format!("cursor:{cursor_id}")
}

pub(super) fn subscriber_checkpoint_basis_label(
    cursor_id: &str,
    checkpoint_sequence: u64,
) -> String {
    format!("cursor:{cursor_id}:checkpoint:{checkpoint_sequence}")
}

pub(super) fn retention_basis_records_for_plan<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    retained_basis_label: &str,
    plan: &CompactionPlan,
) -> Vec<RetentionBasisRecord> {
    let state = backend.state();
    let mut records = Vec::new();
    records.push(retention_basis_record_for_label(
        state,
        retained_basis_label,
    ));
    for basis_label in plan.closure_witness().stable_bases().basis_labels() {
        if basis_label != retained_basis_label {
            records.push(retention_basis_record_for_label(state, basis_label));
        }
    }
    records.sort_by(|left, right| left.artifact_id.cmp(&right.artifact_id));
    records.dedup_by(|left, right| left.artifact_id == right.artifact_id);
    records
}

fn retention_basis_record_for_label(state: &StoreState, basis_label: &str) -> RetentionBasisRecord {
    let artifact_id = retention_basis_artifact_id(basis_label);
    if let Some(snapshot_id) = basis_label.strip_prefix("snapshot:") {
        if let Ok(snapshot_id) = snapshot_id.parse::<u64>() {
            if let Some(record) = state.snapshot_basis_records.get(&snapshot_id) {
                return RetentionBasisRecord {
                    artifact_id,
                    basis_label: basis_label.to_string(),
                    branch_id: Some(record.snapshot_branch_id.clone()),
                    basis_commit_id: Some(record.snapshot_frontier_commit_id),
                    family_version: crate::RETENTION_FAMILY_VERSION,
                };
            }
        }
    }
    if let Some(checkpoint_basis) = basis_label.strip_prefix("cursor:") {
        if let Some((cursor_id, checkpoint_sequence)) = checkpoint_basis.split_once(":checkpoint:")
        {
            if let Ok(checkpoint_sequence) = checkpoint_sequence.parse::<u64>() {
                if let Some(record) =
                    state
                        .subscriber_checkpoint_records
                        .get(&subscriber_checkpoint_artifact_id(
                            cursor_id,
                            checkpoint_sequence,
                        ))
                {
                    return RetentionBasisRecord {
                        artifact_id,
                        basis_label: basis_label.to_string(),
                        branch_id: Some(record.branch_id.clone()),
                        basis_commit_id: Some(record.basis_commit_id),
                        family_version: crate::RETENTION_FAMILY_VERSION,
                    };
                }
            }
        }
    }
    if let Some(cursor_id) = basis_label.strip_prefix("cursor:") {
        if let Some(record) = state
            .durable_cursor_identity_records
            .get(&durable_cursor_identity_artifact_id(cursor_id))
        {
            return RetentionBasisRecord {
                artifact_id,
                basis_label: basis_label.to_string(),
                branch_id: Some(record.branch_id.clone()),
                basis_commit_id: Some(record.latest_basis_commit_id),
                family_version: crate::RETENTION_FAMILY_VERSION,
            };
        }
    }
    if let Some(branch_basis) = basis_label.strip_prefix("branch:") {
        if let Some((branch_name, commit_suffix)) = branch_basis.split_once('@') {
            if let Ok(commit_id) = commit_suffix.parse::<u64>() {
                return RetentionBasisRecord {
                    artifact_id,
                    basis_label: basis_label.to_string(),
                    branch_id: Some(BranchId(branch_name.to_string())),
                    basis_commit_id: Some(CommitId(commit_id)),
                    family_version: crate::RETENTION_FAMILY_VERSION,
                };
            }
        }
    }
    RetentionBasisRecord {
        artifact_id,
        basis_label: basis_label.to_string(),
        branch_id: None,
        basis_commit_id: None,
        family_version: crate::RETENTION_FAMILY_VERSION,
    }
}

pub(super) fn retained_cost_surface_for_basis<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    retained_basis_label: &str,
    read_path: RetainedReadPath,
    compacted_family_count: u64,
    reclaim_deletion_count: u64,
    live_basis_rejection_count: u64,
    rebuild_debt_delta: i64,
) -> RetainedReadCostSurface {
    let state = backend.state();
    let closure_summary = state
        .retention_closure_records
        .get(&retention_closure_artifact_id(retained_basis_label))
        .map(|record| {
            RetentionClosureSummary::new(
                record.retained_head_branch_ids.len() as u64,
                record.stable_basis_labels.len() as u64,
                record.closure_commit_ids.len() as u64,
                record.frontier_commit_ids.len() as u64,
            )
        })
        .unwrap_or_else(|| {
            RetentionClosureSummary::new(
                0,
                u64::from(
                    state
                        .retention_basis_records
                        .contains_key(&retention_basis_artifact_id(retained_basis_label)),
                ),
                0,
                0,
            )
        });
    RetainedReadCostSurface::new(
        read_path,
        closure_summary,
        compacted_family_count,
        0,
        reclaim_deletion_count,
        live_basis_rejection_count,
        rebuild_debt_delta,
    )
}
