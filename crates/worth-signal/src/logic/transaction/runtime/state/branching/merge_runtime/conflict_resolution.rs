use std::collections::BTreeSet;

use crate::data::error::SignalError;
use crate::logic::transaction::runtime::{
    BranchConflictResolutionPlan, BranchMergeConflictEvidence, BranchMergeConflictKind,
    BranchMergeConflictRecord, BranchMergeDivergence, BranchMergeFailureEvidence,
    BranchMergeFailureKind, BranchMergeKind, BranchMergeReconciliationPolicy,
    BranchMergeResolutionRequirement, BranchMergeStrategy, ConflictResolutionRecord,
    ConflictResolutionStrategy,
};

use super::super::super::merge::StructuralMergeJournalSlice;
use super::conflict_classification::{classify_conflicts, summarize_conflicts};

pub(super) struct ConflictResolutionInput<'a> {
    pub(super) source_branch_id: crate::state::SignalBranchId,
    pub(super) target_branch_id: crate::state::SignalBranchId,
    pub(super) source_graph: &'a crate::data::graph::SignalGraph,
    pub(super) target_graph: &'a crate::data::graph::SignalGraph,
    pub(super) source_nodes: &'a [crate::data::handle::NodeId],
    pub(super) identity_matches:
        &'a std::collections::BTreeMap<crate::data::handle::NodeId, crate::data::handle::NodeId>,
    pub(super) source_journal: &'a StructuralMergeJournalSlice,
    pub(super) target_overlap_journal: &'a crate::logic::transaction::BranchMutationJournalSlice,
    pub(super) initial_divergence: BranchMergeDivergence,
    pub(super) initial_merge_kind: BranchMergeKind,
    pub(super) initial_merge_strategy: BranchMergeStrategy,
    pub(super) reconciliation_policy: &'a BranchMergeReconciliationPolicy,
}

pub(super) struct ConflictResolutionOutcome {
    pub(super) divergence: BranchMergeDivergence,
    pub(super) merge_kind: BranchMergeKind,
    pub(super) merge_strategy: BranchMergeStrategy,
    pub(super) records: Vec<BranchMergeConflictRecord>,
    pub(super) resolution_plan: Option<BranchConflictResolutionPlan>,
}

pub(super) fn classify_and_resolve(
    input: ConflictResolutionInput<'_>,
) -> Result<ConflictResolutionOutcome, SignalError> {
    let mut divergence = input.initial_divergence;
    let mut merge_kind = input.initial_merge_kind;
    let mut merge_strategy = input.initial_merge_strategy;
    let records = if matches!(divergence, BranchMergeDivergence::TargetAdvanced) {
        classify_conflicts(
            super::conflict_classification::ConflictClassificationInput {
                source_graph: input.source_graph,
                target_graph: input.target_graph,
                source_nodes: input.source_nodes,
                identity_matches: input.identity_matches,
                source_journal: input.source_journal,
                target_overlap_journal: input.target_overlap_journal,
            },
        )?
    } else {
        Vec::new()
    };
    if records.is_empty() {
        return Ok(ConflictResolutionOutcome {
            divergence,
            merge_kind,
            merge_strategy,
            records,
            resolution_plan: None,
        });
    }
    divergence = BranchMergeDivergence::SharedStateConflict;
    let summary = summarize_conflicts(&records);
    let planned_resolution = build_conflict_resolution_plan(
        input.source_branch_id,
        input.target_branch_id,
        divergence,
        &records,
    );
    if !can_auto_resolve_conflicts(input.reconciliation_policy, &planned_resolution) {
        return Err(SignalError::branch_merge_failed_with_evidence(
            BranchMergeFailureKind::DivergenceRequiresConflictResolution,
            format!(
                "branch merge classified {} shared-state conflict record(s)",
                records.len()
            ),
            BranchMergeFailureEvidence::Conflict(BranchMergeConflictEvidence {
                divergence,
                reconciliation_policy: input.reconciliation_policy.clone(),
                summary,
                resolution_plan: planned_resolution,
                records,
            }),
        ));
    }
    merge_kind = BranchMergeKind::ConflictResolved;
    merge_strategy = BranchMergeStrategy::AdoptSourceSubset;
    Ok(ConflictResolutionOutcome {
        divergence,
        merge_kind,
        merge_strategy,
        records,
        resolution_plan: Some(planned_resolution),
    })
}

fn build_conflict_resolution_plan(
    source_branch_id: crate::state::SignalBranchId,
    target_branch_id: crate::state::SignalBranchId,
    divergence: BranchMergeDivergence,
    records: &[BranchMergeConflictRecord],
) -> BranchConflictResolutionPlan {
    BranchConflictResolutionPlan {
        source_branch_id,
        target_branch_id,
        divergence,
        records: records
            .iter()
            .map(|record| ConflictResolutionRecord {
                source_node: record.source_node,
                target_node: record.target_node,
                required_resolution: record
                    .conflict_kinds
                    .iter()
                    .flat_map(conflict_resolution_requirements_for_kind)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                supported_strategies: record
                    .conflict_kinds
                    .iter()
                    .flat_map(conflict_resolution_strategies_for_kind)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
            })
            .collect(),
    }
}

fn can_auto_resolve_conflicts(
    reconciliation_policy: &BranchMergeReconciliationPolicy,
    resolution_plan: &BranchConflictResolutionPlan,
) -> bool {
    match reconciliation_policy.conflict {
        crate::logic::transaction::runtime::ConflictMergePolicy::RejectSharedStateConflict => false,
        crate::logic::transaction::runtime::ConflictMergePolicy::ResolveSourceStateWhenStructureMatches => {
            resolution_plan.records.iter().all(|record| {
                let requirements = record.required_resolution.as_slice();
                !requirements.is_empty()
                    && requirements.iter().all(|requirement| {
                        matches!(
                            requirement,
                            BranchMergeResolutionRequirement::ReconcileComparableState
                                | BranchMergeResolutionRequirement::ReconcileDependencySnapshot
                                | BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState
                        )
                    })
                    && (!record.required_resolution.contains(
                        &BranchMergeResolutionRequirement::ReconcileComparableState,
                    ) || record
                        .supported_strategies
                        .contains(&ConflictResolutionStrategy::AdoptSourceComparableState))
                    && (!record.required_resolution.contains(
                        &BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState,
                    ) || record
                        .supported_strategies
                        .contains(&ConflictResolutionStrategy::AdoptSourceRuntimeArtifactState))
                    && (!record.required_resolution.contains(
                        &BranchMergeResolutionRequirement::ReconcileDependencySnapshot,
                    ) || record
                        .supported_strategies
                        .contains(&ConflictResolutionStrategy::ReplaySourceDependencySnapshot))
            })
        }
    }
}

fn conflict_resolution_requirements_for_kind(
    kind: &BranchMergeConflictKind,
) -> Vec<BranchMergeResolutionRequirement> {
    match kind {
        BranchMergeConflictKind::ComparableMismatch => {
            vec![BranchMergeResolutionRequirement::ReconcileComparableState]
        }
        BranchMergeConflictKind::DependencyTopologyMismatch => {
            vec![BranchMergeResolutionRequirement::ReconcileDependencyTopology]
        }
        BranchMergeConflictKind::DependencySnapshotMismatch => {
            vec![BranchMergeResolutionRequirement::ReconcileDependencySnapshot]
        }
        BranchMergeConflictKind::RuntimeArtifactMismatch => {
            vec![BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState]
        }
        BranchMergeConflictKind::MergeAuthorityMismatch => {
            vec![BranchMergeResolutionRequirement::ReconcileMergeAuthority]
        }
    }
}

fn conflict_resolution_strategies_for_kind(
    kind: &BranchMergeConflictKind,
) -> Vec<ConflictResolutionStrategy> {
    match kind {
        BranchMergeConflictKind::ComparableMismatch => vec![
            ConflictResolutionStrategy::AdoptSourceComparableState,
            ConflictResolutionStrategy::PreserveTargetComparableState,
        ],
        BranchMergeConflictKind::DependencyTopologyMismatch => vec![
            ConflictResolutionStrategy::ReplaySourceDependencyTopology,
            ConflictResolutionStrategy::PreserveTargetDependencyTopology,
        ],
        BranchMergeConflictKind::DependencySnapshotMismatch => vec![
            ConflictResolutionStrategy::ReplaySourceDependencySnapshot,
            ConflictResolutionStrategy::PreserveTargetDependencySnapshot,
        ],
        BranchMergeConflictKind::RuntimeArtifactMismatch => vec![
            ConflictResolutionStrategy::AdoptSourceRuntimeArtifactState,
            ConflictResolutionStrategy::PreserveTargetRuntimeArtifactState,
        ],
        BranchMergeConflictKind::MergeAuthorityMismatch => vec![
            ConflictResolutionStrategy::AdoptSourceMergeAuthority,
            ConflictResolutionStrategy::PreserveTargetMergeAuthority,
        ],
    }
}
