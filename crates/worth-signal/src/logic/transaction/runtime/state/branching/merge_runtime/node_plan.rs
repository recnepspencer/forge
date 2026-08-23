use std::collections::BTreeMap;

use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeConflictKind, BranchMergeFailureKind, BranchMergeRequestScopeFamily,
    CausalityCarryPolicy, LoweredFoundationalMergeRequest, NodeMergeInputState, NodeMergePlan,
    NodeReconciliationDecision, NodeReconciliationShape, RetainedArtifactCarryPolicy,
    RuntimeArtifactCarryPolicy, SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy,
};

use super::super::super::merge::{
    deny_selected_node_non_adoptable, deny_selected_target_rejected_by_declaration,
    scoped_admission_outcome_to_signal_error, AdoptedNodeContract, AdoptionDependencySnapshotRef,
    AdoptionDependencyTopology, SourceNodeAdoptionCarryPolicy, TargetNodeIdentityIntent,
};
use super::super::super::runtime_state::SignalRuntime;
use super::artifact_projection::node_merge_projection;
use super::candidates::CandidateDiscovery;
use super::correspondence::CorrespondenceResolution;
use super::source_only_policy::ResolvedSourceOnlyPolicySelection;

pub(super) struct NodePlanInput<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) lowered_request: &'a LoweredFoundationalMergeRequest,
    pub(super) candidates: &'a CandidateDiscovery<D, I, T>,
    pub(super) correspondence: &'a CorrespondenceResolution,
    pub(super) source_only_policy: &'a ResolvedSourceOnlyPolicySelection,
    pub(super) conflict_records: &'a [crate::logic::transaction::BranchMergeConflictRecord],
}

pub(super) struct NodePlanAssembly {
    pub(super) node_map: crate::logic::transaction::MergeNodeMap,
    pub(super) node_plan: Vec<NodeMergePlan>,
    pub(super) adoption_core: Vec<SourceNodeAdoptionPlanCore>,
    pub(super) adoption_policy: Vec<SourceNodeAdoptionCarryPolicy>,
}

pub(super) fn assemble_node_plan<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    input: NodePlanInput<'_, D, I, T>,
) -> Result<NodePlanAssembly, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let source_state = &input.candidates.branch_states.source_state;
    let target_graph = input
        .candidates
        .branch_states
        .target_state_owned
        .as_ref()
        .or_else(|| {
            runtime
                .branches
                .branch_state(input.candidates.branch_states.target_branch_id)
        })
        .ok_or_else(|| SignalError::invalid_input("merge target branch state disappeared"))?
        .graph();
    let mut node_map = input.correspondence.node_map.clone();
    let resolved_conflict_kinds_by_node: BTreeMap<_, _> = input
        .conflict_records
        .iter()
        .map(|record| (record.source_node, record.conflict_kinds.clone()))
        .collect();
    let mut node_plan = Vec::new();
    let mut adoption_core = Vec::new();
    let mut adoption_policy = Vec::new();

    for source_node in &input.candidates.source_nodes {
        let source_projection = node_merge_projection(source_state.graph(), *source_node)?;
        let source_cmp = source_projection
            .as_ref()
            .map(|projection| projection.comparable.clone());
        let source_authority = source_projection
            .as_ref()
            .map(|projection| projection.authority.clone());
        let source_artifact_id = source_projection
            .as_ref()
            .and_then(|projection| projection.current_artifact_id);
        if let Some(target_node) = input
            .correspondence
            .identity_matches
            .get(source_node)
            .copied()
        {
            node_map.insert(*source_node, target_node);
            let target_projection = node_merge_projection(target_graph, target_node)?;
            let target_cmp = target_projection
                .as_ref()
                .map(|projection| projection.comparable.clone());
            let target_artifact_id = target_projection
                .as_ref()
                .and_then(|projection| projection.current_artifact_id);
            let target_authority = target_projection
                .as_ref()
                .map(|projection| projection.authority.clone());
            let resolved_conflict_kinds = resolved_conflict_kinds_by_node
                .get(source_node)
                .cloned()
                .unwrap_or_default();
            let decision = if resolved_conflict_kinds.iter().any(|kind| {
                matches!(
                    kind,
                    BranchMergeConflictKind::ComparableMismatch
                        | BranchMergeConflictKind::RuntimeArtifactMismatch
                )
            }) {
                NodeReconciliationDecision::AdoptSourceAuthority
            } else if source_cmp.is_some() && source_cmp == target_cmp {
                NodeReconciliationDecision::MarkEquivalentUnchanged
            } else {
                NodeReconciliationDecision::AdoptSourceAuthority
            };
            node_plan.push(NodeMergePlan::new(
                *source_node,
                NodeReconciliationShape::ExistingTargetNode { target_node },
                NodeMergeInputState::new(
                    source_artifact_id,
                    source_cmp.clone(),
                    source_authority.clone(),
                    true,
                ),
                NodeMergeInputState::new(target_artifact_id, target_cmp, target_authority, true),
                decision,
                resolved_conflict_kinds,
            ));
            continue;
        }

        let authority = source_authority.unwrap_or_default();
        if matches!(
            input.source_only_policy.descriptor.policy(),
            SourceOnlyMergePolicy::RejectIntroduction
        ) {
            return Err(source_only_rejection(
                runtime,
                input.lowered_request,
                *source_node,
                input.source_only_policy.descriptor.semantic_name().as_str(),
            ));
        }
        let decision = if matches!(
            authority.adoptability,
            crate::data::trace::MergeAdoptability::Adoptable
        ) {
            NodeReconciliationDecision::AdoptSourceAuthority
        } else {
            if !matches!(
                input
                    .lowered_request
                    .normalized_request()
                    .normalized_scope()
                    .family(),
                BranchMergeRequestScopeFamily::FullBranch
            ) {
                runtime.with_telemetry(|telemetry| {
                    telemetry.transaction.scoped_merge_denial_count += 1
                });
                return Err(scoped_admission_outcome_to_signal_error(
                    deny_selected_node_non_adoptable(input.lowered_request, *source_node),
                ));
            }
            NodeReconciliationDecision::SkipNonAdoptableSource
        };
        node_plan.push(NodeMergePlan::new(
            *source_node,
            NodeReconciliationShape::SourceOnlyIntroduction,
            NodeMergeInputState::new(
                source_artifact_id,
                source_cmp,
                Some(authority.clone()),
                true,
            ),
            NodeMergeInputState::new(None, None, None, false),
            decision,
            Vec::new(),
        ));
        if matches!(decision, NodeReconciliationDecision::AdoptSourceAuthority) {
            adoption_core.push(SourceNodeAdoptionPlanCore {
                source_node: *source_node,
                target_identity: TargetNodeIdentityIntent::AllocateTargetNode,
                authority,
                entry_contract: AdoptedNodeContract {
                    eval_config: source_state.graph().node_eval_config(*source_node)?.clone(),
                },
                dependency_topology: AdoptionDependencyTopology {
                    dependencies: source_state.graph().dependencies_of(*source_node)?.to_vec(),
                },
                dependency_snapshot_ref: AdoptionDependencySnapshotRef {
                    snapshot: source_state.graph().get_dep_snapshot(*source_node)?.clone(),
                },
            });
            adoption_policy.push(SourceNodeAdoptionCarryPolicy {
                runtime_artifact: RuntimeArtifactCarryPolicy::CarryMergeAdoptable,
                retained_artifact: RetainedArtifactCarryPolicy::CarryIfPolicyAllows,
                causality: CausalityCarryPolicy::CarryIfPolicyAllows,
            });
        }
    }
    Ok(NodePlanAssembly {
        node_map,
        node_plan,
        adoption_core,
        adoption_policy,
    })
}

fn source_only_rejection<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    lowered_request: &LoweredFoundationalMergeRequest,
    source_node: NodeId,
    policy_name: &str,
) -> SignalError
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    if !matches!(
        lowered_request
            .normalized_request()
            .normalized_scope()
            .family(),
        BranchMergeRequestScopeFamily::FullBranch
    ) {
        runtime.with_telemetry(|telemetry| telemetry.transaction.scoped_merge_denial_count += 1);
        return scoped_admission_outcome_to_signal_error(
            deny_selected_target_rejected_by_declaration(lowered_request, source_node),
        );
    }
    SignalError::branch_merge_failed(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "source-only policy `{}` rejects introducing source-only node {} into target authority",
            policy_name, source_node
        ),
    )
}
