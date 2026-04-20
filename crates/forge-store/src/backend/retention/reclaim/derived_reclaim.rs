use forge_relational::facade::history::{BranchId, CommitId};

use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        integrity::rebuild_debt_artifact_id,
        records::{RebuildDebtRecord, StoreState},
    },
    failure::{StoreError, StoreErrorKind},
    retention::{ReclaimExecutionReport, RetainedReadPath, RetentionTargetStateVerification},
};

use super::{
    derived_family_support::{apply_derived_reclaim, derived_artifact_exists, rebuild_debt_reason},
    maintenance_verification::maintenance_verification,
};
use crate::backend::retention::{
    basis::retained_cost_surface_for_basis, planning::protected_layout_artifacts,
};

pub(crate) fn execute_derived_reclaim<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    witness: crate::ReclaimEligibilityWitness,
) -> Result<ReclaimExecutionReport, StoreError> {
    let reclaim_unit = crate::DerivedFamilyReclaimUnit::new(
        witness.retained_basis_label(),
        witness.artifact_family(),
        witness.artifact_id(),
    );
    let rebuild_unit = crate::RetainedRangeRebuildUnit::new(
        witness.retained_basis_label(),
        witness.artifact_family(),
        witness.artifact_id(),
    );
    if derived_reclaim_conflicts_with_live_basis(backend.state(), &reclaim_unit) {
        backend.counters().record_reclaim_rejected_live_basis();
        return Err(StoreError::new(
            StoreErrorKind::ReclaimLiveBasisConflict,
            format!(
                "derived reclaim for `{}` cannot delete `{}` while a live stable basis still requires it",
                reclaim_unit.family_label(),
                reclaim_unit.artifact_id()
            ),
        ));
    }

    let mut next = backend.state().clone();
    let deleted_artifact_count = apply_derived_reclaim(&mut next, &reclaim_unit)?;
    let debt_id = rebuild_debt_artifact_id(
        rebuild_unit.family_label(),
        rebuild_unit.retained_basis_label(),
        rebuild_unit.rebuild_target_id(),
    );
    let inserted_new_debt = !next.rebuild_debt_records.contains_key(&debt_id);
    next.rebuild_debt_records.insert(
        debt_id.clone(),
        RebuildDebtRecord {
            artifact_id: debt_id,
            family_label: rebuild_unit.family_label().to_string(),
            retained_basis_label: rebuild_unit.retained_basis_label().to_string(),
            rebuild_target_id: rebuild_unit.rebuild_target_id().to_string(),
            debt_reason: rebuild_debt_reason(rebuild_unit.family_label()).to_string(),
            family_version: crate::RETENTION_FAMILY_VERSION,
            cleared: false,
        },
    );
    let verification = maintenance_verification(
        &next,
        "execute_derived_reclaim",
        Some(RetentionTargetStateVerification::new(
            witness.artifact_family(),
            witness.artifact_id(),
            false,
            derived_artifact_exists(&next, witness.artifact_family(), witness.artifact_id()),
        )),
    )
    .inspect_err(|_| backend.counters().record_retention_restore_parity_failure())?;
    backend.commit_replacement_state(next)?;
    backend
        .counters()
        .record_reclaimed_derived_artifacts(deleted_artifact_count);
    if inserted_new_debt {
        backend.counters().record_rebuild_debt(1);
    }

    Ok(ReclaimExecutionReport::new(
        reclaim_unit,
        rebuild_unit,
        deleted_artifact_count,
        retained_cost_surface_for_basis(
            backend,
            witness.retained_basis_label(),
            RetainedReadPath::CanonicalRetainedAuthority,
            0,
            deleted_artifact_count,
            0,
            if inserted_new_debt { 1 } else { 0 },
        ),
        verification,
    ))
}

fn live_stable_basis_frontiers(state: &StoreState) -> Vec<(BranchId, CommitId)> {
    state
        .stable_basis_records
        .values()
        .map(|record| {
            (
                record.request.branch_id().clone(),
                record.request.frontier_commit_id(),
            )
        })
        .collect()
}

fn derived_reclaim_conflicts_with_live_basis(
    state: &StoreState,
    reclaim_unit: &crate::DerivedFamilyReclaimUnit,
) -> bool {
    let protected_layout_artifacts = protected_layout_artifacts(
        state,
        &live_stable_basis_frontiers(state)
            .into_iter()
            .enumerate()
            .map(|(idx, (branch_id, frontier_commit_id))| {
                (format!("live-basis:{idx}"), branch_id, frontier_commit_id)
            })
            .collect::<Vec<_>>(),
    );
    match reclaim_unit.family_label() {
        "milestone_6_layout_materialization" => {
            protected_layout_artifacts.contains(reclaim_unit.artifact_id())
        }
        "milestone_6_scope_slice_membership" => state
            .milestone_6_scope_slice_membership_records
            .get(reclaim_unit.artifact_id())
            .map(|record| {
                protected_layout_artifacts.contains(&record.layout_materialization_artifact_id)
            })
            .unwrap_or(false),
        "milestone_6_chunk_membership" => state
            .milestone_6_chunk_membership_records
            .get(reclaim_unit.artifact_id())
            .map(|record| {
                protected_layout_artifacts.contains(&record.layout_materialization_artifact_id)
            })
            .unwrap_or(false),
        "milestone_6_structural_block" => state
            .milestone_6_structural_block_records
            .get(reclaim_unit.artifact_id())
            .map(|record| {
                record
                    .supporting_layout_materialization_artifact_ids
                    .iter()
                    .any(|artifact_id| protected_layout_artifacts.contains(artifact_id))
            })
            .unwrap_or(false),
        _ => false,
    }
}
