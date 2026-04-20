use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        integrity::stable_structural_digest,
        records::StoreState,
    },
    evidence::StoreCounterSnapshot,
};
use serde::Serialize;

pub(crate) fn milestone_10_counter_contract<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone10CounterContract {
    milestone_10_counter_contract_from_snapshot(&backend.counters().snapshot())
}

pub(crate) fn milestone_10_complexity_surface<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone10ComplexitySurface {
    milestone_10_complexity_surface_from_parts(backend.state(), &backend.counters().snapshot())
}

pub(crate) fn milestone_10_artifact_report<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<crate::Milestone10ArtifactReport, crate::StoreError> {
    let state = backend.state();
    let digest_basis = Milestone10ArtifactDigestBasis {
        compaction_product_records: state.compaction_product_records.values().collect(),
        retention_basis_records: state.retention_basis_records.values().collect(),
        retention_closure_records: state.retention_closure_records.values().collect(),
        rebuild_debt_records: state.rebuild_debt_records.values().collect(),
    };
    Ok(crate::Milestone10ArtifactReport {
        artifact_digest: stable_structural_digest(&digest_basis)?,
        unverified_compaction_product_count: state
            .compaction_product_records
            .values()
            .filter(|record| !record.parity_verified || !record.cutover_committed)
            .count(),
        uncleared_rebuild_debt_count: state
            .rebuild_debt_records
            .values()
            .filter(|record| !record.cleared)
            .count(),
    })
}

fn milestone_10_counter_contract_from_snapshot(
    snapshot: &StoreCounterSnapshot,
) -> crate::Milestone10CounterContract {
    crate::Milestone10CounterContract {
        retention_policy_evaluation_count: snapshot.retention_policy_evaluation_count,
        retained_authoritative_range_count: snapshot.retained_authoritative_range_count,
        expired_authoritative_range_count: snapshot.expired_authoritative_range_count,
        compaction_plan_count: snapshot.compaction_plan_count,
        compacted_delta_layer_count: snapshot.compacted_delta_layer_count,
        compacted_snapshot_family_count: snapshot.compacted_snapshot_family_count,
        compacted_layout_family_count: snapshot.compacted_layout_family_count,
        compaction_cutover_count: snapshot.compaction_cutover_count,
        compaction_cutover_rejection_count: snapshot.compaction_cutover_rejection_count,
        reclaim_candidate_count: snapshot.reclaim_candidate_count,
        reclaimed_authoritative_artifact_count: snapshot.reclaimed_authoritative_artifact_count,
        reclaimed_derived_artifact_count: snapshot.reclaimed_derived_artifact_count,
        reclaim_rejected_live_basis_count: snapshot.reclaim_rejected_live_basis_count,
        retention_closure_ancestor_count: snapshot.retention_closure_ancestor_count,
        retention_closure_failure_count: snapshot.retention_closure_failure_count,
        retained_range_rebuild_count: snapshot.retained_range_rebuild_count,
        rebuild_debt_count: snapshot.rebuild_debt_count,
        compaction_debt_count: snapshot.compaction_debt_count,
        retention_truth_parity_failure_count: snapshot.retention_truth_parity_failure_count,
        retention_restore_parity_failure_count: snapshot.retention_restore_parity_failure_count,
        retention_artifact_rebuild_failure_count: snapshot.retention_artifact_rebuild_failure_count,
    }
}

fn milestone_10_complexity_surface_from_parts(
    state: &StoreState,
    snapshot: &StoreCounterSnapshot,
) -> crate::Milestone10ComplexitySurface {
    let retention_candidate_planning = if snapshot.retention_closure_failure_count > 0 {
        crate::Milestone10ComplexityPathStatus::debt(
            "retention closure planning has recorded closure failures that require basis repair",
        )
    } else if snapshot.retention_policy_evaluation_count > 0 {
        crate::Milestone10ComplexityPathStatus::verified(
            "retention planning executes through retained-head and stable-basis closure witnesses",
        )
    } else {
        crate::Milestone10ComplexityPathStatus::verified(
            "retention planning surface is compiled and awaiting the first admitted policy evaluation",
        )
    };

    let has_uncut_compaction = state
        .compaction_product_records
        .values()
        .any(|record| !record.parity_verified || !record.cutover_committed);
    let compaction_publication = if snapshot.retention_truth_parity_failure_count > 0 {
        crate::Milestone10ComplexityPathStatus::debt(
            "compaction publication has recorded retained-truth parity failures",
        )
    } else if has_uncut_compaction {
        crate::Milestone10ComplexityPathStatus::debt(
            "compaction publication has persisted products that are not yet fully parity-verified and cut over",
        )
    } else if snapshot.compaction_debt_count > 0 {
        crate::Milestone10ComplexityPathStatus::debt(
            "compaction planning has deferred or rejected candidates that still represent maintenance debt",
        )
    } else {
        crate::Milestone10ComplexityPathStatus::verified(
            "compaction publication is either cleanly cut over or idle with no known deferred products",
        )
    };

    let reclaim_execution = if snapshot.retention_restore_parity_failure_count > 0 {
        crate::Milestone10ComplexityPathStatus::debt(
            "reclaim execution has recorded restore-parity failures after maintenance",
        )
    } else {
        crate::Milestone10ComplexityPathStatus::verified(
            "reclaim execution enforces typed eligibility and live-basis checks before delete",
        )
    };

    let uncleared_rebuild_debt = state
        .rebuild_debt_records
        .values()
        .any(|record| !record.cleared);
    let retained_range_rebuild = if snapshot.retention_artifact_rebuild_failure_count > 0
        || snapshot.retention_restore_parity_failure_count > 0
    {
        crate::Milestone10ComplexityPathStatus::debt(
            "retained-range rebuild has recorded rebuild or restore parity failures",
        )
    } else if uncleared_rebuild_debt {
        crate::Milestone10ComplexityPathStatus::debt(
            "retained-range rebuild still has published rebuild debt waiting to be cleared",
        )
    } else {
        crate::Milestone10ComplexityPathStatus::verified(
            "retained-range rebuild has no uncleared rebuild debt in durable state",
        )
    };

    crate::Milestone10ComplexitySurface {
        retention_candidate_planning,
        compaction_publication,
        reclaim_execution,
        retained_range_rebuild,
    }
}

#[derive(Serialize)]
struct Milestone10ArtifactDigestBasis<'a> {
    compaction_product_records: Vec<&'a crate::backend::records::CompactionProductRecord>,
    retention_basis_records: Vec<&'a crate::backend::records::RetentionBasisRecord>,
    retention_closure_records: Vec<&'a crate::backend::records::RetentionClosureRecord>,
    rebuild_debt_records: Vec<&'a crate::backend::records::RebuildDebtRecord>,
}
