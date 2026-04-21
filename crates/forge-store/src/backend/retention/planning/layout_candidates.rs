use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::history::{BranchId, CommitId};

use crate::{
    backend::records::StoreState,
    evidence::StoreCounters,
    retention::{DerivedFamilyRetentionPolicy, RebuildDebtSummary},
};

pub(crate) fn protected_layout_artifacts(
    state: &StoreState,
    stable_basis_frontiers: &[(String, BranchId, CommitId)],
) -> BTreeSet<String> {
    state
        .milestone_6_layout_materialization_records
        .values()
        .filter_map(|record| {
            let target = record.materialization.admitted_plan().request().target();
            stable_basis_frontiers
                .iter()
                .any(|(_, branch_id, frontier_commit_id)| {
                    branch_id == target.branch_id()
                        && *frontier_commit_id == target.frontier_commit_id()
                })
                .then(|| record.artifact_id.clone())
        })
        .collect()
}

pub(super) fn collect_layout_family_candidates(
    policy: &crate::ConservativeRetentionPolicy,
    state: &StoreState,
    closure_commit_set: &BTreeSet<CommitId>,
    protected_layout_artifacts: &BTreeSet<String>,
    reclaim_candidates: &mut Vec<crate::ReclaimEligibilityWitness>,
    rebuild_debts: &mut Vec<RebuildDebtSummary>,
    counters: &StoreCounters,
) {
    let mut materialization_basis = BTreeMap::new();
    for record in state.milestone_6_layout_materialization_records.values() {
        let target = record.materialization.admitted_plan().request().target();
        materialization_basis.insert(
            record.artifact_id.clone(),
            (
                target.branch_id().clone(),
                target.frontier_commit_id(),
                super::super::basis::branch_basis_label(
                    target.branch_id(),
                    target.frontier_commit_id(),
                ),
            ),
        );
    }

    collect_materialization_candidates(
        policy,
        state,
        closure_commit_set,
        protected_layout_artifacts,
        reclaim_candidates,
        rebuild_debts,
        counters,
        &materialization_basis,
    );
    collect_support_family_candidates(
        policy,
        state,
        closure_commit_set,
        protected_layout_artifacts,
        reclaim_candidates,
        rebuild_debts,
        counters,
        &materialization_basis,
    );
}

fn collect_materialization_candidates(
    policy: &crate::ConservativeRetentionPolicy,
    state: &StoreState,
    closure_commit_set: &BTreeSet<CommitId>,
    protected_layout_artifacts: &BTreeSet<String>,
    reclaim_candidates: &mut Vec<crate::ReclaimEligibilityWitness>,
    rebuild_debts: &mut Vec<RebuildDebtSummary>,
    counters: &StoreCounters,
    materialization_basis: &BTreeMap<String, (BranchId, CommitId, String)>,
) {
    if !policy
        .reclaimable_derived_families()
        .contains(&DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization)
    {
        return;
    }
    for record in state.milestone_6_layout_materialization_records.values() {
        let Some((_, frontier, basis_label)) = materialization_basis.get(&record.artifact_id)
        else {
            continue;
        };
        if !closure_commit_set.contains(frontier) {
            continue;
        }
        if protected_layout_artifacts.contains(&record.artifact_id) {
            counters.record_reclaim_rejected_live_basis();
            continue;
        }
        reclaim_candidates.push(crate::ReclaimEligibilityWitness::new(
            "milestone_6_layout_materialization",
            record.artifact_id.clone(),
            basis_label.clone(),
        ));
        rebuild_debts.push(RebuildDebtSummary::new(
            "milestone_6_layout_materialization",
            basis_label.clone(),
            record.artifact_id.clone(),
            "policy admitted reclaim of a rebuildable Milestone 6 layout family",
        ));
    }
}

fn collect_support_family_candidates(
    policy: &crate::ConservativeRetentionPolicy,
    state: &StoreState,
    closure_commit_set: &BTreeSet<CommitId>,
    protected_layout_artifacts: &BTreeSet<String>,
    reclaim_candidates: &mut Vec<crate::ReclaimEligibilityWitness>,
    rebuild_debts: &mut Vec<RebuildDebtSummary>,
    counters: &StoreCounters,
    materialization_basis: &BTreeMap<String, (BranchId, CommitId, String)>,
) {
    collect_scope_slice_candidates(
        policy,
        state,
        closure_commit_set,
        protected_layout_artifacts,
        reclaim_candidates,
        rebuild_debts,
        counters,
        materialization_basis,
    );
    collect_chunk_membership_candidates(
        policy,
        state,
        closure_commit_set,
        protected_layout_artifacts,
        reclaim_candidates,
        rebuild_debts,
        counters,
        materialization_basis,
    );
    collect_structural_block_candidates(
        policy,
        state,
        closure_commit_set,
        protected_layout_artifacts,
        reclaim_candidates,
        rebuild_debts,
        counters,
        materialization_basis,
    );
}

fn collect_scope_slice_candidates(
    policy: &crate::ConservativeRetentionPolicy,
    state: &StoreState,
    closure_commit_set: &BTreeSet<CommitId>,
    protected_layout_artifacts: &BTreeSet<String>,
    reclaim_candidates: &mut Vec<crate::ReclaimEligibilityWitness>,
    rebuild_debts: &mut Vec<RebuildDebtSummary>,
    counters: &StoreCounters,
    materialization_basis: &BTreeMap<String, (BranchId, CommitId, String)>,
) {
    if !policy
        .reclaimable_derived_families()
        .contains(&DerivedFamilyRetentionPolicy::Milestone6ScopeSliceMembership)
    {
        return;
    }
    for record in state.milestone_6_scope_slice_membership_records.values() {
        if protected_layout_artifacts.contains(&record.layout_materialization_artifact_id) {
            counters.record_reclaim_rejected_live_basis();
            continue;
        }
        let Some((_, frontier, basis_label)) =
            materialization_basis.get(&record.layout_materialization_artifact_id)
        else {
            continue;
        };
        if !closure_commit_set.contains(frontier) {
            continue;
        }
        reclaim_candidates.push(crate::ReclaimEligibilityWitness::new(
            "milestone_6_scope_slice_membership",
            record.artifact_id.clone(),
            basis_label.clone(),
        ));
        rebuild_debts.push(RebuildDebtSummary::new(
            "milestone_6_scope_slice_membership",
            basis_label.clone(),
            record.artifact_id.clone(),
            "policy admitted reclaim of Milestone 6 scope membership backed by a surviving retained basis",
        ));
    }
}

fn collect_chunk_membership_candidates(
    policy: &crate::ConservativeRetentionPolicy,
    state: &StoreState,
    closure_commit_set: &BTreeSet<CommitId>,
    protected_layout_artifacts: &BTreeSet<String>,
    reclaim_candidates: &mut Vec<crate::ReclaimEligibilityWitness>,
    rebuild_debts: &mut Vec<RebuildDebtSummary>,
    counters: &StoreCounters,
    materialization_basis: &BTreeMap<String, (BranchId, CommitId, String)>,
) {
    if !policy
        .reclaimable_derived_families()
        .contains(&DerivedFamilyRetentionPolicy::Milestone6ChunkMembership)
    {
        return;
    }
    for record in state.milestone_6_chunk_membership_records.values() {
        if protected_layout_artifacts.contains(&record.layout_materialization_artifact_id) {
            counters.record_reclaim_rejected_live_basis();
            continue;
        }
        let Some((_, frontier, basis_label)) =
            materialization_basis.get(&record.layout_materialization_artifact_id)
        else {
            continue;
        };
        if !closure_commit_set.contains(frontier) {
            continue;
        }
        reclaim_candidates.push(crate::ReclaimEligibilityWitness::new(
            "milestone_6_chunk_membership",
            record.artifact_id.clone(),
            basis_label.clone(),
        ));
        rebuild_debts.push(RebuildDebtSummary::new(
            "milestone_6_chunk_membership",
            basis_label.clone(),
            record.artifact_id.clone(),
            "policy admitted reclaim of Milestone 6 chunk membership backed by a surviving retained basis",
        ));
    }
}

fn collect_structural_block_candidates(
    policy: &crate::ConservativeRetentionPolicy,
    state: &StoreState,
    closure_commit_set: &BTreeSet<CommitId>,
    protected_layout_artifacts: &BTreeSet<String>,
    reclaim_candidates: &mut Vec<crate::ReclaimEligibilityWitness>,
    rebuild_debts: &mut Vec<RebuildDebtSummary>,
    counters: &StoreCounters,
    materialization_basis: &BTreeMap<String, (BranchId, CommitId, String)>,
) {
    if !policy
        .reclaimable_derived_families()
        .contains(&DerivedFamilyRetentionPolicy::Milestone6StructuralBlock)
    {
        return;
    }
    for record in state.milestone_6_structural_block_records.values() {
        if record
            .supporting_layout_materialization_artifact_ids
            .iter()
            .any(|artifact_id| protected_layout_artifacts.contains(artifact_id))
        {
            counters.record_reclaim_rejected_live_basis();
            continue;
        }
        let Some(materialization_artifact_id) = record
            .supporting_layout_materialization_artifact_ids
            .first()
        else {
            continue;
        };
        let Some((_, frontier, basis_label)) =
            materialization_basis.get(materialization_artifact_id)
        else {
            continue;
        };
        if !closure_commit_set.contains(frontier) {
            continue;
        }
        reclaim_candidates.push(crate::ReclaimEligibilityWitness::new(
            "milestone_6_structural_block",
            record.artifact_id.clone(),
            basis_label.clone(),
        ));
        rebuild_debts.push(RebuildDebtSummary::new(
            "milestone_6_structural_block",
            basis_label.clone(),
            record.artifact_id.clone(),
            "policy admitted reclaim of Milestone 6 structural blocks backed by a surviving retained basis",
        ));
    }
}
