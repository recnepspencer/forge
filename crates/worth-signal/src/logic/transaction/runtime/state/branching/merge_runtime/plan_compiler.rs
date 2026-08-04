use crate::data::error::SignalError;
use crate::logic::transaction::runtime::{
    BranchMergeDivergence, BranchMergeKind, BranchMergePlan, BranchMergeStrategy,
    ConflictIsolationGranularity, DeletionMergePolicy, IdentityMatchPolicy,
    LoweredFoundationalMergeRequest, LoweredMergePlan,
};

use super::super::super::merge::runtime_proof_report;
use super::super::super::runtime_state::SignalRuntime;
use super::aspect_policy;
use super::candidates;
use super::conflict_isolation;
use super::conflict_resolution::{self, ConflictResolutionInput};
use super::correspondence;
use super::deletion_policy;
use super::identity_matcher;
use super::merge_base_strategy;
use super::node_plan;
use super::source_only_policy;

pub(super) fn compile<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    lowered_request: &LoweredFoundationalMergeRequest,
) -> Result<BranchMergePlan, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let request = lowered_request.normalized_request().request();

    let branch_states = candidates::discover_branch_states(runtime, request)?;
    let merge_base = merge_base_strategy::resolve_base(
        &runtime.merge_base_strategy_registry,
        request,
        &branch_states,
    )?;
    let candidates =
        candidates::lower_scoped_candidates(runtime, lowered_request, branch_states, &merge_base)?;

    let identity_selection = identity_matcher::resolve_identity_matcher(
        &runtime.identity_matcher_registry,
        &runtime.schema_registry,
        candidates.branch_states.source_state.graph(),
        &candidates.source_nodes,
        request,
        IdentityMatchPolicy::ExactNodeId,
    )?;
    let correspondence =
        correspondence::lower_correspondence(correspondence::CorrespondencePhaseInput {
            runtime,
            request: lowered_request,
            candidates: &candidates,
            matcher_name: identity_selection.descriptor.semantic_name(),
            matcher_policy: identity_selection.descriptor.policy(),
        })?;

    let target_has_overlapping_merge_delta =
        !correspondence.target_overlap_journal.records.is_empty();
    let divergence = if merge_base.base.forked_from_snapshot_id
        == candidates.branch_states.target_snapshot_id_before
        && !target_has_overlapping_merge_delta
    {
        BranchMergeDivergence::None
    } else {
        BranchMergeDivergence::TargetAdvanced
    };
    let initial_merge_kind = if matches!(divergence, BranchMergeDivergence::None) {
        BranchMergeKind::FastForward
    } else {
        BranchMergeKind::Applied
    };
    let default_merge_strategy = match initial_merge_kind {
        BranchMergeKind::FastForward => BranchMergeStrategy::AdoptSourceHead,
        BranchMergeKind::Applied => BranchMergeStrategy::AdoptSourceSubset,
        BranchMergeKind::ConflictResolved => BranchMergeStrategy::RebaseSourceOntoTarget,
    };
    let resolved_strategy = merge_base_strategy::resolve_merge_strategy(
        &runtime.merge_strategy_registry,
        &runtime.schema_registry,
        candidates.branch_states.source_state.graph(),
        &candidates.source_nodes,
        request,
        default_merge_strategy,
    )?;
    let resolved_conflict_policy = super::conflict_policy::resolve_conflict_policy(
        &runtime.conflict_policy_registry,
        &runtime.schema_registry,
        candidates.branch_states.source_state.graph(),
        &candidates.source_nodes,
        request,
        resolved_strategy
            .descriptor
            .reconciliation_policy()
            .conflict,
    )?;
    let resolved_conflict_isolation = conflict_isolation::resolve_conflict_isolation(
        &runtime.conflict_isolation_registry,
        &runtime.schema_registry,
        candidates.branch_states.source_state.graph(),
        &candidates.source_nodes,
        request,
        ConflictIsolationGranularity::PerNode,
    )?;
    let resolved_source_only_policy = source_only_policy::resolve_source_only_policy(
        &runtime.source_only_policy_registry,
        &runtime.schema_registry,
        candidates.branch_states.source_state.graph(),
        &candidates.source_nodes,
        request,
        resolved_strategy
            .descriptor
            .reconciliation_policy()
            .source_only,
    )?;
    let resolved_deletion_policy = deletion_policy::resolve_deletion_policy(
        &runtime.deletion_policy_registry,
        &runtime.schema_registry,
        candidates.branch_states.source_state.graph(),
        &candidates.source_nodes,
        request,
        DeletionMergePolicy::PreserveTargetOnly,
    )?;
    let mut reconciliation_policy = resolved_strategy.descriptor.reconciliation_policy().clone();
    reconciliation_policy.conflict = resolved_conflict_policy.descriptor.policy();
    reconciliation_policy.source_only = resolved_source_only_policy.descriptor.policy();
    reconciliation_policy.deletion = resolved_deletion_policy.descriptor.policy();

    let denial_anchor = candidates
        .scoped_candidates
        .admitted_candidate_nodes()
        .first()
        .copied()
        .or_else(|| candidates.source_nodes.first().copied());
    let deletion_plan = deletion_policy::lower_deletion_plan(
        runtime,
        lowered_request,
        correspondence.target_only_nodes.clone(),
        &resolved_deletion_policy,
        denial_anchor,
    )?;
    let aspect_policy_plan = aspect_policy::lower_aspect_policy_plan(
        &runtime.aspect_merge_policy_registry,
        &runtime.schema_registry,
        candidates.branch_states.source_state.graph(),
        &candidates.planned_candidates.nodes,
        request,
    )?;
    let runtime_proof = runtime_proof_report(
        runtime.schema_registry.registry_digest(),
        runtime.merge_strategy_registry.registry_digest(),
        runtime.merge_base_strategy_registry.registry_digest(),
        runtime.aspect_merge_policy_registry.registry_digest(),
        runtime.conflict_isolation_registry.registry_digest(),
        runtime.conflict_policy_registry.registry_digest(),
        runtime.identity_matcher_registry.registry_digest(),
        runtime.source_only_policy_registry.registry_digest(),
        runtime.deletion_policy_registry.registry_digest(),
    );

    let conflict_outcome = {
        let target_graph = candidates
            .branch_states
            .target_state_owned
            .as_ref()
            .or_else(|| {
                runtime
                    .branches
                    .branch_state(candidates.branch_states.target_branch_id)
            })
            .ok_or_else(|| SignalError::invalid_input("merge target branch state disappeared"))?
            .graph();
        conflict_resolution::classify_and_resolve(ConflictResolutionInput {
            source_branch_id: request.source_branch.id,
            target_branch_id: request.target_branch.id,
            source_graph: candidates.branch_states.source_state.graph(),
            target_graph,
            source_nodes: &candidates.source_nodes,
            identity_matches: &correspondence.identity_matches,
            source_journal: &candidates.source_journal,
            target_overlap_journal: &correspondence.target_overlap_journal,
            initial_divergence: divergence,
            initial_merge_kind,
            initial_merge_strategy: resolved_strategy.descriptor.merge_strategy(),
            reconciliation_policy: &reconciliation_policy,
        })?
    };
    let conflict_isolation_plan = conflict_isolation::lower_conflict_isolation_plan(
        &resolved_conflict_isolation,
        candidates.branch_states.source_state.graph(),
        &conflict_outcome.records,
    )?;
    let node_assembly = {
        node_plan::assemble_node_plan(
            runtime,
            node_plan::NodePlanInput {
                lowered_request,
                candidates: &candidates,
                correspondence: &correspondence,
                source_only_policy: &resolved_source_only_policy,
                conflict_records: &conflict_outcome.records,
            },
        )?
    };
    let aspect_decision_plan =
        aspect_policy::lower_aspect_decision_plan(&aspect_policy_plan, &node_assembly.node_plan);

    Ok(LoweredMergePlan::new(
        request.source_branch.id,
        request.target_branch.id,
        runtime.schema_registry.registry_digest().to_owned(),
        runtime_proof.registry_bundle_digest.clone(),
        conflict_outcome.merge_kind,
        conflict_outcome.divergence,
        conflict_outcome.merge_strategy,
        resolved_strategy.descriptor.semantic_name().clone(),
        resolved_strategy.descriptor.digest().to_string(),
        resolved_strategy.basis,
        resolved_conflict_policy.descriptor.semantic_name().clone(),
        resolved_conflict_policy.descriptor.digest().to_string(),
        resolved_conflict_policy.basis,
        resolved_conflict_isolation
            .descriptor
            .semantic_name()
            .clone(),
        resolved_conflict_isolation.descriptor.digest().to_string(),
        resolved_conflict_isolation.basis,
        identity_selection.descriptor.semantic_name().clone(),
        identity_selection.descriptor.digest().to_string(),
        identity_selection.basis,
        resolved_source_only_policy
            .descriptor
            .semantic_name()
            .clone(),
        resolved_source_only_policy.descriptor.digest().to_string(),
        resolved_source_only_policy.basis,
        resolved_deletion_policy.descriptor.semantic_name().clone(),
        resolved_deletion_policy.descriptor.digest().to_string(),
        resolved_deletion_policy.basis,
        reconciliation_policy,
        candidates.boundary_witness,
        candidates.source_journal,
        correspondence.target_overlap_journal,
        correspondence.identity_correspondence,
        deletion_plan,
        conflict_isolation_plan,
        aspect_policy_plan,
        aspect_decision_plan,
        candidates.scoped_candidates,
        candidates.scoped_merge_proof,
        correspondence.proof_minimal_overlap,
        correspondence.conservative_overlap,
        candidates.planned_candidates,
        candidates.branch_states.source_snapshot_id,
        candidates.branch_states.target_snapshot_id_before,
        Some(merge_base.base),
        Some(merge_base.lowered),
        conflict_outcome.resolution_plan,
        node_assembly.node_map,
        node_assembly.node_plan,
        node_assembly.adoption_core,
        node_assembly.adoption_policy,
    ))
}
