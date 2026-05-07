use crate::certification::error::WorthTopologyCertificationError;
use crate::certification::milestone_three::report::{
    WorthMilestoneThreeEditReplayParityReport, WorthMilestoneThreeEditReplayStepRow,
    WorthMilestoneThreeHostileOutcomeClass,
};
use crate::certification::{WorthDeterministicDigest, WorthReplayParityStatus};
use crate::edit::{
    WorthNamingEditContinuityMatrix, WorthTopologyEditBatch, WorthTopologyEditContract,
    WorthTopologyEditNamingOutcome, WorthTopologyQueryEditExecution,
    WorthTopologyQueryEditExecutionError,
};
use crate::materialization::MaterializedTopologyView;
use crate::parity::digest_materialized_topology_view;

pub(super) fn find_loop_id_by_label(
    materialized: &MaterializedTopologyView,
    label: &str,
) -> Result<forge_relational::facade::identity::EntityId, WorthTopologyCertificationError> {
    materialized
        .topology()
        .loops
        .iter()
        .find(|loop_record| loop_record.label == label)
        .map(|loop_record| loop_record.entity_id)
        .ok_or_else(|| {
            WorthTopologyCertificationError::Query(format!(
                "materialized topology should expose loop `{label}`"
            ))
        })
}

pub(super) fn aggregate_topology_edit_digest(
    batches: &[WorthTopologyEditBatch],
) -> WorthDeterministicDigestBackedEditDigest {
    let rows = batches
        .iter()
        .flat_map(|batch| batch.contracts().iter().map(contract_digest_row));
    let contract_count = batches.iter().map(|batch| batch.contracts().len()).sum();
    let family_count = batches.iter().map(|batch| batch.families().len()).sum();
    let changed_scope_count = batches
        .iter()
        .flat_map(|batch| batch.contracts().iter())
        .map(|contract| contract.changed_scopes().len())
        .sum();
    let naming_scope_count = batches
        .iter()
        .flat_map(|batch| batch.contracts().iter())
        .map(|contract| contract.naming_scopes().len())
        .sum();
    let derived_region_count = batches
        .iter()
        .flat_map(|batch| batch.contracts().iter())
        .map(|contract| contract.derived_regions().len())
        .sum();
    WorthDeterministicDigestBackedEditDigest {
        digest: digest_rows(rows),
        contract_count,
        family_count,
        changed_scope_count,
        naming_scope_count,
        derived_region_count,
    }
}

pub(super) type WorthDeterministicDigestBackedEditDigest = crate::edit::WorthTopologyEditDigest;

pub(super) fn aggregate_naming_edit_continuity_matrix(
    batches: &[WorthTopologyEditBatch],
) -> WorthNamingEditContinuityMatrix {
    let rows = batches
        .iter()
        .flat_map(|batch| batch.naming_edit_continuity_matrix().rows.into_iter())
        .collect::<Vec<_>>();
    let preserved_count = rows
        .iter()
        .filter(|row| row.outcome == WorthTopologyEditNamingOutcome::Preserved)
        .count();
    let ambiguous_count = rows
        .iter()
        .filter(|row| row.outcome == WorthTopologyEditNamingOutcome::Ambiguous)
        .count();
    let rejected_count = rows
        .iter()
        .filter(|row| row.outcome == WorthTopologyEditNamingOutcome::Rejected)
        .count();
    WorthNamingEditContinuityMatrix {
        rows,
        preserved_count,
        ambiguous_count,
        rejected_count,
    }
}

pub(super) fn accepted_step_row(
    step_index: usize,
    batch: &WorthTopologyEditBatch,
    execution: &WorthTopologyQueryEditExecution,
) -> WorthMilestoneThreeEditReplayStepRow {
    WorthMilestoneThreeEditReplayStepRow {
        step_index,
        edit_families: batch.families(),
        topology_edit_digest: execution.topology_edit_digest.clone(),
        naming_edit_continuity_matrix: execution.naming_continuity_matrix.clone(),
        outcome_class: WorthMilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        resulting_materialized_topology_digest: Some(digest_materialized_topology_view(
            &execution.materialized,
        )),
    }
}

pub(super) fn rejected_step_row(
    step_index: usize,
    batch: &WorthTopologyEditBatch,
    error: &WorthTopologyQueryEditExecutionError,
) -> WorthMilestoneThreeEditReplayStepRow {
    WorthMilestoneThreeEditReplayStepRow {
        step_index,
        edit_families: batch.families(),
        topology_edit_digest: batch.topology_edit_digest(),
        naming_edit_continuity_matrix: batch.naming_edit_continuity_matrix(),
        outcome_class: WorthMilestoneThreeHostileOutcomeClass::Rejected,
        rejection_class: error.rejection_class(),
        resulting_materialized_topology_digest: None,
    }
}

pub(super) fn replay_not_checked(
    step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
) -> WorthMilestoneThreeEditReplayParityReport {
    WorthMilestoneThreeEditReplayParityReport {
        replay_checked: false,
        parity_status: WorthReplayParityStatus::NotChecked,
        mismatch_count: 0,
        step_rows,
        replay_step_rows: Vec::new(),
        baseline_materialized_topology_digest: None,
        final_materialized_topology_digest: None,
        replay_final_materialized_topology_digest: None,
        returned_to_baseline: None,
    }
}

pub(super) fn replay_checked(
    step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
    replay_step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: WorthDeterministicDigest,
    final_materialized_topology_digest: WorthDeterministicDigest,
    replay_final_materialized_topology_digest: WorthDeterministicDigest,
) -> WorthMilestoneThreeEditReplayParityReport {
    let returned_to_baseline =
        final_materialized_topology_digest == baseline_materialized_topology_digest;
    let mut mismatch_count = 0usize;
    if step_rows != replay_step_rows {
        mismatch_count += 1;
    }
    if final_materialized_topology_digest != replay_final_materialized_topology_digest {
        mismatch_count += 1;
    }
    let parity_status = if mismatch_count == 0 {
        WorthReplayParityStatus::Match
    } else {
        WorthReplayParityStatus::Mismatch
    };
    WorthMilestoneThreeEditReplayParityReport {
        replay_checked: true,
        parity_status,
        mismatch_count,
        step_rows,
        replay_step_rows,
        baseline_materialized_topology_digest: Some(baseline_materialized_topology_digest),
        final_materialized_topology_digest: Some(final_materialized_topology_digest),
        replay_final_materialized_topology_digest: Some(replay_final_materialized_topology_digest),
        returned_to_baseline: Some(returned_to_baseline),
    }
}

pub(super) fn replay_checked_rejected(
    step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
    replay_step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: WorthDeterministicDigest,
) -> WorthMilestoneThreeEditReplayParityReport {
    let mismatch_count = usize::from(step_rows != replay_step_rows);
    let parity_status = if mismatch_count == 0 {
        WorthReplayParityStatus::Match
    } else {
        WorthReplayParityStatus::Mismatch
    };
    WorthMilestoneThreeEditReplayParityReport {
        replay_checked: true,
        parity_status,
        mismatch_count,
        step_rows,
        replay_step_rows,
        baseline_materialized_topology_digest: Some(baseline_materialized_topology_digest.clone()),
        final_materialized_topology_digest: Some(baseline_materialized_topology_digest.clone()),
        replay_final_materialized_topology_digest: Some(baseline_materialized_topology_digest),
        returned_to_baseline: Some(true),
    }
}

fn contract_digest_row(contract: &WorthTopologyEditContract) -> String {
    serde_json::to_string(contract).expect("worth topology edit contracts should serialize")
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> WorthDeterministicDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    WorthDeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}
