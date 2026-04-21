use std::collections::BTreeSet;

use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        retention::{
            basis::{
                branch_basis_label, durable_cursor_basis_label, snapshot_basis_label,
                subscriber_checkpoint_basis_label,
            },
            planning::{
                layout_candidates::{collect_layout_family_candidates, protected_layout_artifacts},
                ranges::{expired_ranges_for_policy, retained_ranges_for_policy},
            },
        },
    },
    failure::StoreError,
    retention::{
        CompactionCandidateRejection, CompactionPlan, ConservativeRetentionPlan,
        RebuildDebtSummary, RetentionCandidatePlan, RetentionClosureSummary,
        RetentionPlanningReport,
    },
};

pub(crate) fn plan_retention_candidates<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    policy_class: crate::RetentionPolicyClass,
) -> Result<RetentionPlanningReport, StoreError> {
    let policy = match policy_class.require_conservative() {
        Ok(policy) => policy.clone(),
        Err(error) => {
            backend.counters().record_compaction_debt(1);
            return Err(error);
        }
    };

    let retained_heads = crate::RetainedHeadSet::new(
        backend
            .state()
            .branch_head_records
            .values()
            .filter(|record| record.head_commit_id.is_some())
            .map(|record| record.branch_id.clone())
            .collect(),
    );

    let mut frontier_commit_ids = backend
        .state()
        .branch_head_records
        .values()
        .filter_map(|record| record.head_commit_id)
        .collect::<Vec<_>>();
    let mut stable_basis_labels = Vec::new();
    let mut stable_basis_frontiers = Vec::new();

    for snapshot_policy in policy.pinned_snapshots() {
        if let Some(record) = backend
            .state()
            .snapshot_basis_records
            .get(&snapshot_policy.snapshot_id().0)
        {
            let label = snapshot_basis_label(record.snapshot_id);
            stable_basis_labels.push(label.clone());
            stable_basis_frontiers.push((
                label,
                record.snapshot_branch_id.clone(),
                record.snapshot_frontier_commit_id,
            ));
            frontier_commit_ids.push(record.snapshot_frontier_commit_id);
        }
    }
    for record in backend.state().durable_cursor_identity_records.values() {
        let label = durable_cursor_basis_label(&record.cursor_id);
        stable_basis_labels.push(label.clone());
        stable_basis_frontiers.push((
            label,
            record.branch_id.clone(),
            record.latest_basis_commit_id,
        ));
        frontier_commit_ids.push(record.latest_basis_commit_id);
    }
    for record in backend.state().subscriber_checkpoint_records.values() {
        let label =
            subscriber_checkpoint_basis_label(&record.cursor_id, record.checkpoint_sequence);
        stable_basis_labels.push(label.clone());
        stable_basis_frontiers.push((label, record.branch_id.clone(), record.basis_commit_id));
        frontier_commit_ids.push(record.basis_commit_id);
    }
    frontier_commit_ids.extend(policy.branch_history_windows().iter().flat_map(|window| {
        backend
            .state()
            .branch_commit_sequences(window.branch_id())
            .into_iter()
            .rev()
            .take(window.minimum_retained_commit_count() as usize)
            .map(|(_, commit_id)| commit_id)
            .collect::<Vec<_>>()
    }));

    let closure_commit_ids = match backend
        .state()
        .retention_closure_from_frontiers(frontier_commit_ids.clone())
    {
        Ok(closure_commit_ids) => closure_commit_ids,
        Err(error) => {
            backend.counters().record_retention_closure_failure();
            return Err(error);
        }
    };
    let closure_commit_set = closure_commit_ids.iter().copied().collect::<BTreeSet<_>>();
    let closure_witness = crate::RetentionClosureWitness::new(
        retained_heads.clone(),
        crate::StableBasisSet::new(stable_basis_labels.clone()),
        closure_commit_ids,
        frontier_commit_ids,
    );
    let closure_summary = RetentionClosureSummary::from_witness(&closure_witness);
    let conservative_plan = ConservativeRetentionPlan::new(
        RetentionCandidatePlan::new(policy_class, closure_summary),
        retained_heads,
        crate::StableBasisSet::new(stable_basis_labels),
        policy.clone(),
    );

    let retained_ranges = retained_ranges_for_policy(backend.state(), &policy, &closure_commit_set);
    let expired_ranges = expired_ranges_for_policy(backend.state(), &policy, &closure_commit_set);
    let (compaction_plans, compaction_rejections) =
        compaction_candidates(backend, &closure_commit_set, &closure_witness, &policy);

    let protected_layout_artifacts =
        protected_layout_artifacts(backend.state(), &stable_basis_frontiers);
    let mut reclaim_candidates = Vec::new();
    let mut rebuild_debts = Vec::new();
    collect_layout_family_candidates(
        &policy,
        backend.state(),
        &closure_commit_set,
        &protected_layout_artifacts,
        &mut reclaim_candidates,
        &mut rebuild_debts,
        backend.counters(),
    );
    for record in backend
        .state()
        .rebuild_debt_records
        .values()
        .filter(|record| !record.cleared)
    {
        let already_present = rebuild_debts.iter().any(|summary| {
            summary.family_label() == record.family_label
                && summary.retained_basis_label() == record.retained_basis_label
                && summary.rebuild_target_id() == record.rebuild_target_id
        });
        if !already_present {
            rebuild_debts.push(RebuildDebtSummary::new(
                record.family_label.clone(),
                record.retained_basis_label.clone(),
                record.rebuild_target_id.clone(),
                record.debt_reason.clone(),
            ));
        }
    }

    let basis_survival_verdicts = stable_basis_frontiers
        .into_iter()
        .map(|(label, _, frontier_commit_id)| {
            if closure_commit_set.contains(&frontier_commit_id) {
                crate::BasisSurvivalVerdict::survives(label)
            } else {
                crate::BasisSurvivalVerdict::expires(
                    label,
                    "basis frontier is outside the retained closure",
                )
            }
        })
        .collect::<Vec<_>>();

    backend.counters().record_retention_policy_evaluation();
    backend
        .counters()
        .record_retention_closure(closure_witness.closure_commit_ids().len() as u64);
    backend
        .counters()
        .record_retained_authoritative_ranges(retained_ranges.len() as u64);
    backend
        .counters()
        .record_expired_authoritative_ranges(expired_ranges.len() as u64);
    for _ in &compaction_plans {
        backend.counters().record_compaction_plan();
    }
    backend
        .counters()
        .record_compaction_debt(compaction_rejections.len() as u64);
    backend
        .counters()
        .record_reclaim_candidates(reclaim_candidates.len() as u64);
    backend
        .counters()
        .record_rebuild_debt(rebuild_debts.len() as u64);

    Ok(RetentionPlanningReport::new(
        closure_witness,
        conservative_plan,
        retained_ranges,
        expired_ranges,
        compaction_plans,
        compaction_rejections,
        reclaim_candidates,
        rebuild_debts,
        basis_survival_verdicts,
    ))
}

fn compaction_candidates<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    closure_commit_set: &BTreeSet<forge_relational::facade::history::CommitId>,
    closure_witness: &crate::RetentionClosureWitness,
    policy: &crate::ConservativeRetentionPolicy,
) -> (Vec<CompactionPlan>, Vec<CompactionCandidateRejection>) {
    let mut compaction_plans = Vec::new();
    let mut compaction_rejections = Vec::new();

    for snapshot_policy in policy.pinned_snapshots() {
        if let Some(record) = backend
            .state()
            .snapshot_basis_records
            .get(&snapshot_policy.snapshot_id().0)
        {
            if closure_commit_set.contains(&record.snapshot_frontier_commit_id) {
                compaction_plans.push(CompactionPlan::new(
                    snapshot_basis_label(record.snapshot_id),
                    closure_witness.clone(),
                    vec!["snapshot_family".to_string()],
                    vec![crate::SupersededPhysicalFamily::new(
                        "snapshot_family",
                        format!("snapshot:{}", record.snapshot_id.0),
                        Some(record.snapshot_frontier_commit_id),
                    )],
                    1,
                ));
            } else {
                compaction_rejections.push(CompactionCandidateRejection::new(
                    "snapshot_family",
                    Some(format!("snapshot:{}", record.snapshot_id.0)),
                    "snapshot frontier is outside the retained closure",
                ));
            }
        }
    }
    for layer in backend.state().branch_delta_layer_records.values() {
        if closure_commit_set.contains(&layer.target_frontier_commit_id) {
            compaction_plans.push(CompactionPlan::new(
                branch_basis_label(&layer.branch_id, layer.target_frontier_commit_id),
                closure_witness.clone(),
                vec!["branch_delta_layer".to_string()],
                vec![crate::SupersededPhysicalFamily::new(
                    "branch_delta_layer",
                    layer.branch_delta_layer_id.0.to_string(),
                    Some(layer.target_frontier_commit_id),
                )],
                1,
            ));
        } else {
            compaction_rejections.push(CompactionCandidateRejection::new(
                "branch_delta_layer",
                Some(layer.branch_delta_layer_id.0.to_string()),
                "branch-delta target frontier is outside the retained closure",
            ));
        }
    }
    for record in backend
        .state()
        .milestone_6_layout_materialization_records
        .values()
    {
        let branch_id = record
            .materialization
            .admitted_plan()
            .request()
            .target()
            .branch_id()
            .clone();
        let frontier = record
            .materialization
            .admitted_plan()
            .request()
            .target()
            .frontier_commit_id();
        if closure_commit_set.contains(&frontier) {
            compaction_plans.push(CompactionPlan::new(
                branch_basis_label(&branch_id, frontier),
                closure_witness.clone(),
                vec!["milestone_6_layout_materialization".to_string()],
                vec![crate::SupersededPhysicalFamily::new(
                    "milestone_6_layout_materialization",
                    record.artifact_id.clone(),
                    Some(frontier),
                )],
                1,
            ));
        } else {
            compaction_rejections.push(CompactionCandidateRejection::new(
                "milestone_6_layout_materialization",
                Some(record.artifact_id.clone()),
                "layout materialization frontier is outside the retained closure",
            ));
        }
    }
    (compaction_plans, compaction_rejections)
}
