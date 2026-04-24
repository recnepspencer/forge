use crate::data::aspect::{Aspect, AspectMask};
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::reuse::ReuseStrategy;
use crate::data::trace::{RuntimeArtifactHot, RuntimeArtifactWarm};
use crate::logic::transaction::{
    AspectMergePolicySelectionBasis, ConflictIsolationSelectionBasis, ConflictPolicySelectionBasis,
    DeletionPolicySelectionBasis, IdentityMatcherSelectionBasis, MergeStrategySelectionBasis,
    SourceOnlyPolicySelectionBasis,
};
use crate::state::{SignalBranchHandle, SnapshotArtifactRetentionPolicy};
use std::collections::{BTreeMap, BTreeSet};

use super::super::merge::{
    adopt_source_node_into_target, remap_dependency_snapshot, AdoptedNodeContract,
    AdoptionDependencySnapshotRef, AdoptionDependencyTopology, ArtifactMergeAction,
    AspectMergeDecisionOutcome, AspectMergePolicy, AspectMergePolicyDescriptor,
    BranchConflictResolutionPlan, BranchMergeBase, BranchMergeConflictEvidence,
    BranchMergeConflictKind, BranchMergeConflictRecord, BranchMergeConflictSummary,
    BranchMergeCounters, BranchMergeDivergence, BranchMergeExecutionSummary,
    BranchMergeFailureEvidence, BranchMergeFailureKind, BranchMergeIdentityFailureEvidence,
    BranchMergeKind, BranchMergePlan, BranchMergeReconciliationPolicy, BranchMergeRequest,
    BranchMergeResolutionRequirement, BranchMergeResult, BranchMergeStrategy, CausalityCarryPolicy,
    ConflictIsolationGranularity, ConflictIsolationPolicyDescriptor, ConflictIsolationPolicyName,
    ConflictIsolationWitness, ConflictMergePolicy, ConflictPolicyDescriptor,
    ConflictResolutionRecord, ConflictResolutionStrategy, ConservativeIsolationExpansion,
    ConservativeOverlapExpansion, DeletionMergePolicy, DeletionPolicyDescriptor,
    FrozenAspectMergePolicyRegistry, FrozenConflictIsolationRegistry, FrozenConflictPolicyRegistry,
    FrozenDeletionPolicyRegistry, FrozenIdentityMatcherRegistry, FrozenMergeBaseStrategyRegistry,
    FrozenMergeStrategyRegistry, FrozenSourceOnlyPolicyRegistry, IdentityCorrespondenceBasis,
    IdentityCorrespondenceRecord, IdentityCorrespondenceStatus, IdentityMatchPolicy,
    IdentityMatcherDescriptor, LoweredAspectMergeDecisionPlan, LoweredAspectMergeDecisionRecord,
    LoweredAspectMergePolicyPlan, LoweredAspectMergePolicyRecord, LoweredConflictIsolationPlan,
    LoweredConflictIsolationRecord, LoweredDeletionPolicyPlan, LoweredIdentityCorrespondencePlan,
    LoweredMergeBasePlan, LoweredMergePlan, MergeBaseSelectionBasis, MergeBaseSelectionPolicy,
    MergeBaseStrategyDescriptor, MergeBoundaryWitness, MergeBoundaryWitnessKind,
    MergeDecisionBasis, MergeNodeMap, MergeTouchedNodeSet, MergedArtifactRecord,
    NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision, NodeReconciliationShape,
    PlannedMergeCandidateSet, ProofMinimalOverlapBasis, RegionIsolationSummary,
    RetainedArtifactCarryPolicy, RuntimeArtifactCarryPolicy, SourceNodeAdoptionCarryPolicy,
    SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy, SourceOnlyPolicyDescriptor,
    StructuralMergeCandidateRecord, StructuralMergeJournalSlice, TargetNodeIdentityIntent,
    TopologyRepairSummary,
};
use super::super::runtime_state::SignalRuntime;
use super::branches::LatestMergeReference;

#[derive(Debug, Clone)]
struct ResolvedMergeStrategySelection {
    descriptor: crate::logic::transaction::runtime::MergeStrategyDescriptor,
    basis: MergeStrategySelectionBasis,
}

#[derive(Debug, Clone)]
struct ResolvedConflictPolicySelection {
    descriptor: ConflictPolicyDescriptor,
    basis: ConflictPolicySelectionBasis,
}

#[derive(Debug, Clone)]
struct ResolvedConflictIsolationSelection {
    descriptor: ConflictIsolationPolicyDescriptor,
    basis: ConflictIsolationSelectionBasis,
}

#[derive(Debug, Clone)]
struct ResolvedAspectPolicySelection {
    descriptor: AspectMergePolicyDescriptor,
    basis: AspectMergePolicySelectionBasis,
}

#[derive(Debug, Clone)]
struct ResolvedMergeBaseSelection {
    descriptor: MergeBaseStrategyDescriptor,
    basis: MergeBaseSelectionBasis,
}

#[derive(Debug, Clone)]
struct ResolvedIdentityMatcherSelection {
    descriptor: IdentityMatcherDescriptor,
    basis: IdentityMatcherSelectionBasis,
}

#[derive(Debug, Clone)]
struct ResolvedSourceOnlyPolicySelection {
    descriptor: SourceOnlyPolicyDescriptor,
    basis: SourceOnlyPolicySelectionBasis,
}

#[derive(Debug, Clone)]
struct ResolvedDeletionPolicySelection {
    descriptor: DeletionPolicyDescriptor,
    basis: DeletionPolicySelectionBasis,
}

#[derive(Debug, Clone)]
struct IdentityResolutionOutcome {
    matches: BTreeMap<NodeId, NodeId>,
    correspondence: LoweredIdentityCorrespondencePlan,
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn merge_branch(
        &mut self,
        source: SignalBranchHandle,
        target: SignalBranchHandle,
    ) -> Result<BranchMergeResult, SignalError> {
        let request = BranchMergeRequest {
            source_branch: source,
            target_branch: target,
            strategy_name: None,
            strategy_hint: None,
            merge_base_name: None,
            conflict_policy_name: None,
            identity_matcher_name: None,
            source_only_policy_name: None,
            deletion_policy_name: None,
            conflict_isolation_policy_name: None,
            aspect_policy_bindings: Vec::new(),
        };
        let plan = self.plan_branch_merge_request(&request)?;
        self.execute_branch_merge_request_plan(&request, &plan)
    }

    pub(crate) fn plan_branch_merge_request(
        &mut self,
        request: &BranchMergeRequest,
    ) -> Result<BranchMergePlan, SignalError> {
        if request.source_branch.id == request.target_branch.id {
            let error = SignalError::branch_merge_failed(
                BranchMergeFailureKind::SelfMergeRejected,
                "branch merge cannot target itself",
            );
            crate::diagnostics::recorder::record_branch_merge_failure(
                &mut self.graph,
                &error,
                Some(request.source_branch.clone()),
                Some(request.target_branch.clone()),
            );
            return Err(error);
        }
        match self.build_branch_merge_plan(request) {
            Ok(plan) => Ok(plan),
            Err(error) => {
                crate::diagnostics::recorder::record_branch_merge_failure(
                    &mut self.graph,
                    &error,
                    Some(request.source_branch.clone()),
                    Some(request.target_branch.clone()),
                );
                Err(error)
            }
        }
    }

    pub(crate) fn execute_branch_merge_request_plan(
        &mut self,
        request: &BranchMergeRequest,
        plan: &BranchMergePlan,
    ) -> Result<BranchMergeResult, SignalError> {
        let summary = match self.execute_branch_merge_plan(request, plan) {
            Ok(summary) => summary,
            Err(error) => {
                crate::diagnostics::recorder::record_branch_merge_failure(
                    &mut self.graph,
                    &error,
                    Some(request.source_branch.clone()),
                    Some(request.target_branch.clone()),
                );
                return Err(error);
            }
        };
        crate::diagnostics::recorder::record_branch_merge_summary(
            &mut self.graph,
            &summary,
            request.source_branch.name.clone(),
            request.target_branch.name.clone(),
        );
        Ok(BranchMergeResult {
            source_branch: summary.source_branch_id,
            target_branch: summary.target_branch_id,
            schema_registry_digest: summary.schema_registry_digest.clone(),
            registry_bundle_digest: summary.registry_bundle_digest.clone(),
            lowered_strategy_bundle_digest: summary.lowered_strategy_bundle_digest.clone(),
            merge_kind: summary.merge_kind,
            divergence: summary.divergence,
            merge_strategy: summary.merge_strategy,
            selected_strategy_name: summary.selected_strategy_name.clone(),
            selected_strategy_digest: summary.selected_strategy_digest.clone(),
            selected_strategy_basis: summary.selected_strategy_basis,
            selected_conflict_policy_name: summary.selected_conflict_policy_name.clone(),
            selected_conflict_policy_digest: summary.selected_conflict_policy_digest.clone(),
            selected_conflict_policy_basis: summary.selected_conflict_policy_basis,
            selected_conflict_isolation_name: summary.selected_conflict_isolation_name.clone(),
            selected_conflict_isolation_digest: summary.selected_conflict_isolation_digest.clone(),
            selected_conflict_isolation_basis: summary.selected_conflict_isolation_basis,
            selected_identity_matcher_name: summary.selected_identity_matcher_name.clone(),
            selected_identity_matcher_digest: summary.selected_identity_matcher_digest.clone(),
            selected_identity_matcher_basis: summary.selected_identity_matcher_basis,
            selected_source_only_policy_name: summary.selected_source_only_policy_name.clone(),
            selected_source_only_policy_digest: summary.selected_source_only_policy_digest.clone(),
            selected_source_only_policy_basis: summary.selected_source_only_policy_basis,
            selected_deletion_policy_name: summary.selected_deletion_policy_name.clone(),
            selected_deletion_policy_digest: summary.selected_deletion_policy_digest.clone(),
            selected_deletion_policy_basis: summary.selected_deletion_policy_basis,
            selected_merge_base_name: summary.selected_merge_base_name.clone(),
            selected_merge_base_digest: summary.selected_merge_base_digest.clone(),
            selected_merge_base_basis: summary.selected_merge_base_basis,
            selected_semantics: summary.selected_semantics.clone(),
            reconciliation_policy: summary.reconciliation_policy,
            boundary_witness: summary.boundary_witness.clone(),
            identity_correspondence: summary.identity_correspondence.clone(),
            deletion_plan: summary.deletion_plan.clone(),
            conflict_isolation_plan: summary.conflict_isolation_plan.clone(),
            aspect_policy_plan: summary.aspect_policy_plan.clone(),
            aspect_decision_plan: summary.aspect_decision_plan.clone(),
            proof_minimal_overlap: summary.proof_minimal_overlap.clone(),
            conservative_overlap: summary.conservative_overlap.clone(),
            planned_candidates: summary.planned_candidates.clone(),
            merged_snapshot_id: summary.target_snapshot_id_after,
            lowered_merge_base: summary.lowered_merge_base.clone(),
            target_snapshot_id_before: summary.target_snapshot_id_before,
            target_snapshot_id_after: summary.target_snapshot_id_after,
            source_snapshot_id: summary.source_snapshot_id,
            resolution_plan: summary.resolution_plan.clone(),
            records: summary.records.clone(),
            counters: summary.counters,
        })
    }

    fn build_branch_merge_plan(
        &mut self,
        request: &BranchMergeRequest,
    ) -> Result<BranchMergePlan, SignalError> {
        let source_state_owned = if request.source_branch.id == self.graph.current_branch().id {
            {
                self.capture_heavy_branch_state()
            }
        } else {
            self.branches
                .branch_state(request.source_branch.id)
                .cloned()
                .ok_or_else(|| {
                    SignalError::unknown_branch(
                        Some(request.source_branch.id),
                        request.source_branch.name.clone(),
                    )
                })?
        };
        let target_state_owned = if request.target_branch.id == self.graph.current_branch().id {
            Some(self.capture_heavy_branch_state())
        } else {
            None
        };
        let target_state = target_state_owned
            .as_ref()
            .or_else(|| self.branches.branch_state(request.target_branch.id));
        let target_state = target_state.ok_or_else(|| {
            SignalError::unknown_branch(
                Some(request.target_branch.id),
                request.target_branch.name.clone(),
            )
        })?;
        let source_state = &source_state_owned;
        let target_graph = target_state.graph();
        let target_snapshot_id_before =
            target_graph.branch_head_snapshot_id(request.target_branch.id);
        let source_snapshot_id = source_state
            .graph()
            .branch_head_snapshot_id(request.source_branch.id);
        let resolved_merge_base = resolve_merge_base_descriptor(
            &self.merge_base_strategy_registry,
            request,
            MergeBaseSelectionPolicy::ForkPointSnapshot,
        )?;
        let merge_base_snapshot = match resolved_merge_base.descriptor.policy() {
            MergeBaseSelectionPolicy::ForkPointSnapshot => {
                source_state.ancestry().forked_from_snapshot_id()
            }
        };
        let resolved_merge_base_record = BranchMergeBase {
            source_branch_id: request.source_branch.id,
            target_branch_id: request.target_branch.id,
            forked_from_snapshot_id: merge_base_snapshot,
            source_snapshot_id,
            target_snapshot_id_before,
        };
        let lowered_merge_base = Some(LoweredMergeBasePlan {
            resolved_base: resolved_merge_base_record.clone(),
            selected_merge_base_name: resolved_merge_base.descriptor.semantic_name().clone(),
            selected_merge_base_digest: resolved_merge_base.descriptor.digest().to_string(),
            selected_merge_base_basis: resolved_merge_base.basis,
        });
        let mut node_map = MergeNodeMap::default();
        if !source_state.mutation_ledger().boundary_established {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                "branch merge requires an established mutation-journal boundary; whole-live branch scans are no longer admitted",
            ));
        }
        let boundary_witness = MergeBoundaryWitness {
            source_branch_id: request.source_branch.id,
            target_branch_id: request.target_branch.id,
            kind: MergeBoundaryWitnessKind::MutationJournalBoundary,
            forked_from_snapshot_id: merge_base_snapshot,
            source_snapshot_id,
            target_snapshot_id_before,
        };
        let source_journal = StructuralMergeJournalSlice::from_branch_journal(
            boundary_witness.clone(),
            source_state.mutation_ledger().structural_merge_journal(),
        );
        let target_identity_journal = target_state.mutation_ledger().structural_merge_journal();
        let source_nodes = source_journal.candidate_nodes();
        let planned_candidates = PlannedMergeCandidateSet {
            nodes: source_nodes.clone(),
        };
        let mut proof_minimal_overlap_nodes = Vec::new();
        let mut conservative_overlap_nodes = planned_candidates
            .nodes
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let mut conservative_support_nodes = BTreeSet::new();
        for source_node in &source_nodes {
            for dependency in source_state.graph().dependencies_of(*source_node)? {
                if target_graph.is_alive(dependency.source()) {
                    node_map.insert(dependency.source(), dependency.source());
                    if !planned_candidates.nodes.contains(&dependency.source()) {
                        conservative_support_nodes.insert(dependency.source());
                    }
                    conservative_overlap_nodes.insert(dependency.source());
                }
            }
            for snapshot_entry in source_state
                .graph()
                .get_dep_snapshot(*source_node)?
                .entries()
            {
                if target_graph.is_alive(snapshot_entry.source) {
                    node_map.insert(snapshot_entry.source, snapshot_entry.source);
                    if !planned_candidates.nodes.contains(&snapshot_entry.source) {
                        conservative_support_nodes.insert(snapshot_entry.source);
                    }
                    conservative_overlap_nodes.insert(snapshot_entry.source);
                }
            }
        }
        let resolved_identity_matcher = resolve_identity_matcher_descriptor(
            &self.identity_matcher_registry,
            &self.schema_registry,
            source_state.graph(),
            &source_nodes,
            request,
            IdentityMatchPolicy::ExactNodeId,
        )?;
        let identity_outcome = resolve_identity_matches(
            resolved_identity_matcher.descriptor.semantic_name(),
            resolved_identity_matcher.descriptor.policy(),
            source_state.graph(),
            target_graph,
            &source_nodes,
            &target_identity_journal,
        )?;
        let identity_matches = identity_outcome.matches;
        let identity_correspondence = identity_outcome.correspondence;
        let matched_target_nodes = identity_matches.values().copied().collect::<BTreeSet<_>>();
        for source_node in &source_nodes {
            if let Some(target_node) = identity_matches.get(source_node).copied() {
                proof_minimal_overlap_nodes.push(*source_node);
                conservative_overlap_nodes.insert(*source_node);
                conservative_overlap_nodes.insert(target_node);
                node_map.insert(*source_node, target_node);
            }
        }
        let proof_minimal_overlap = ProofMinimalOverlapBasis {
            shared_nodes: proof_minimal_overlap_nodes,
        };
        let target_overlap_journal = crate::logic::transaction::BranchMutationJournalSlice {
            records: target_identity_journal
                .records
                .iter()
                .filter(|record| matched_target_nodes.contains(&record.node))
                .cloned()
                .collect(),
        };
        let target_only_nodes = target_identity_journal
            .records
            .iter()
            .filter(|record| !matched_target_nodes.contains(&record.node))
            .map(|record| record.node)
            .collect::<Vec<_>>();
        let conservative_overlap = ConservativeOverlapExpansion {
            expanded_nodes: conservative_overlap_nodes.into_iter().collect(),
            support_nodes: conservative_support_nodes.into_iter().collect(),
        };
        let target_has_overlapping_merge_delta = !target_overlap_journal.records.is_empty();
        let divergence = if merge_base_snapshot == target_snapshot_id_before
            && !target_has_overlapping_merge_delta
        {
            BranchMergeDivergence::None
        } else {
            BranchMergeDivergence::TargetAdvanced
        };
        let mut merge_kind = if matches!(divergence, BranchMergeDivergence::None) {
            BranchMergeKind::FastForward
        } else {
            BranchMergeKind::Applied
        };
        let default_merge_strategy = match merge_kind {
            BranchMergeKind::FastForward => BranchMergeStrategy::AdoptSourceHead,
            BranchMergeKind::Applied => BranchMergeStrategy::AdoptSourceSubset,
            BranchMergeKind::ConflictResolved => BranchMergeStrategy::RebaseSourceOntoTarget,
        };
        let resolved_strategy = resolve_merge_strategy_descriptor(
            &self.merge_strategy_registry,
            &self.schema_registry,
            source_state.graph(),
            &source_nodes,
            request,
            default_merge_strategy,
        )?;
        let resolved_conflict_policy = resolve_conflict_policy_descriptor(
            &self.conflict_policy_registry,
            &self.schema_registry,
            source_state.graph(),
            &source_nodes,
            request,
            resolved_strategy
                .descriptor
                .reconciliation_policy()
                .conflict,
        )?;
        let resolved_conflict_isolation = resolve_conflict_isolation_descriptor(
            &self.conflict_isolation_registry,
            &self.schema_registry,
            source_state.graph(),
            &source_nodes,
            request,
            ConflictIsolationGranularity::PerNode,
        )?;
        let resolved_source_only_policy = resolve_source_only_policy_descriptor(
            &self.source_only_policy_registry,
            &self.schema_registry,
            source_state.graph(),
            &source_nodes,
            request,
            resolved_strategy
                .descriptor
                .reconciliation_policy()
                .source_only,
        )?;
        let resolved_deletion_policy = resolve_deletion_policy_descriptor(
            &self.deletion_policy_registry,
            &self.schema_registry,
            source_state.graph(),
            &source_nodes,
            request,
            DeletionMergePolicy::PreserveTargetOnly,
        )?;
        let mut merge_strategy = resolved_strategy.descriptor.merge_strategy();
        let mut reconciliation_policy =
            resolved_strategy.descriptor.reconciliation_policy().clone();
        reconciliation_policy.conflict = resolved_conflict_policy.descriptor.policy();
        reconciliation_policy.source_only = resolved_source_only_policy.descriptor.policy();
        reconciliation_policy.deletion = resolved_deletion_policy.descriptor.policy();
        let deletion_plan = LoweredDeletionPolicyPlan {
            target_only_nodes: target_only_nodes.clone(),
            target_only_count: target_only_nodes.len() as u64,
            rejected_target_only_count: u64::from(
                matches!(
                    resolved_deletion_policy.descriptor.policy(),
                    DeletionMergePolicy::RejectTargetOnlyConflict
                ) && !target_only_nodes.is_empty(),
            ),
        };
        if matches!(
            resolved_deletion_policy.descriptor.policy(),
            DeletionMergePolicy::RejectTargetOnlyConflict
        ) && !target_only_nodes.is_empty()
        {
            return Err(SignalError::branch_merge_failed_with_evidence(
                BranchMergeFailureKind::DivergenceRequiresConflictResolution,
                format!(
                    "deletion policy `{}` rejects {} target-only branch delta node(s)",
                    resolved_deletion_policy.descriptor.semantic_name().as_str(),
                    target_only_nodes.len()
                ),
                BranchMergeFailureEvidence::Deletion(
                    crate::logic::transaction::BranchMergeDeletionFailureEvidence {
                        deletion_policy_name: resolved_deletion_policy
                            .descriptor
                            .semantic_name()
                            .clone(),
                        target_only_nodes,
                        deletion_plan: deletion_plan.clone(),
                    },
                ),
            ));
        }
        let aspect_policy_plan = lower_aspect_policy_plan(
            &self.aspect_merge_policy_registry,
            &self.schema_registry,
            source_state.graph(),
            &planned_candidates.nodes,
            request,
        )?;
        let runtime_proof = crate::logic::transaction::runtime::runtime_proof_report(
            self.schema_registry.registry_digest(),
            self.merge_strategy_registry.registry_digest(),
            self.merge_base_strategy_registry.registry_digest(),
            self.aspect_merge_policy_registry.registry_digest(),
            self.conflict_isolation_registry.registry_digest(),
            self.conflict_policy_registry.registry_digest(),
            self.identity_matcher_registry.registry_digest(),
            self.source_only_policy_registry.registry_digest(),
            self.deletion_policy_registry.registry_digest(),
        );
        let mut divergence = divergence;
        let mut conflict_records = Vec::new();
        let mut resolution_plan = None;
        if matches!(divergence, BranchMergeDivergence::TargetAdvanced) {
            for source_node in &source_nodes {
                let Some(target_node) = identity_matches.get(source_node).copied() else {
                    continue;
                };
                let source_cmp = node_merge_projection(source_state.graph(), *source_node)?
                    .map(|projection| projection.comparable);
                let target_cmp = node_merge_projection(target_graph, target_node)?
                    .map(|projection| projection.comparable);
                let source_structural_record = source_journal
                    .records
                    .iter()
                    .find(|record| record.node == *source_node)
                    .cloned();
                let target_structural_record = target_overlap_journal
                    .records
                    .iter()
                    .find(|record| record.node == target_node)
                    .cloned();
                let conflict_kinds = classify_conflict_kinds(
                    source_cmp.as_ref(),
                    target_cmp.as_ref(),
                    source_structural_record.as_ref(),
                    target_structural_record.as_ref(),
                );
                if !conflict_kinds.is_empty() {
                    divergence = BranchMergeDivergence::SharedStateConflict;
                    conflict_records.push(BranchMergeConflictRecord {
                        source_node: *source_node,
                        target_node,
                        conflict_kinds,
                        source_comparable: source_cmp,
                        target_comparable: target_cmp,
                        source_structural_record,
                        target_structural_record,
                    });
                }
            }
        }
        if !conflict_records.is_empty() {
            let conflict_summary = summarize_conflict_records(&conflict_records);
            let planned_resolution = build_conflict_resolution_plan(
                request.source_branch.id,
                request.target_branch.id,
                divergence,
                &conflict_records,
            );
            if can_auto_resolve_conflicts(&reconciliation_policy, &planned_resolution) {
                merge_kind = BranchMergeKind::ConflictResolved;
                merge_strategy = BranchMergeStrategy::AdoptSourceSubset;
                resolution_plan = Some(planned_resolution);
            } else {
                return Err(SignalError::branch_merge_failed_with_evidence(
                    BranchMergeFailureKind::DivergenceRequiresConflictResolution,
                    format!(
                        "branch merge classified {} shared-state conflict record(s)",
                        conflict_records.len()
                    ),
                    BranchMergeFailureEvidence::Conflict(BranchMergeConflictEvidence {
                        divergence,
                        reconciliation_policy,
                        summary: conflict_summary,
                        resolution_plan: planned_resolution,
                        records: conflict_records,
                    }),
                ));
            }
        }

        let conflict_isolation_plan = lower_conflict_isolation_plan(
            resolved_conflict_isolation
                .descriptor
                .semantic_name()
                .clone(),
            resolved_conflict_isolation.descriptor.digest().to_string(),
            resolved_conflict_isolation.basis,
            resolved_conflict_isolation.descriptor.granularity(),
            source_state.graph(),
            &conflict_records,
        )?;

        let resolved_conflict_kinds_by_node: BTreeMap<_, _> = conflict_records
            .iter()
            .map(|record| (record.source_node, record.conflict_kinds.clone()))
            .collect();
        let mut node_plan = Vec::new();
        let mut adoption_core = Vec::new();
        let mut adoption_policy = Vec::new();
        for source_node in source_nodes {
            let source_projection = node_merge_projection(source_state.graph(), source_node)?;
            let source_cmp = source_projection
                .as_ref()
                .map(|projection| projection.comparable.clone());
            let source_authority = source_projection
                .as_ref()
                .map(|projection| projection.authority.clone());
            let source_artifact_id = source_projection
                .as_ref()
                .and_then(|projection| projection.current_artifact_id);
            if let Some(target_node) = identity_matches.get(&source_node).copied() {
                node_map.insert(source_node, target_node);
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
                    .get(&source_node)
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
                    source_node,
                    NodeReconciliationShape::ExistingTargetNode { target_node },
                    NodeMergeInputState::new(
                        source_artifact_id,
                        source_cmp.clone(),
                        source_authority.clone(),
                        true,
                    ),
                    NodeMergeInputState::new(
                        target_artifact_id,
                        target_cmp.clone(),
                        target_authority,
                        true,
                    ),
                    decision,
                    resolved_conflict_kinds,
                ));
            } else {
                let authority = source_authority.unwrap_or_default();
                if matches!(
                    reconciliation_policy.source_only,
                    SourceOnlyMergePolicy::RejectIntroduction
                ) {
                    return Err(SignalError::branch_merge_failed(
                        BranchMergeFailureKind::UnsupportedMergeStrategy,
                        format!(
                            "source-only policy `{}` rejects introducing source-only node {} into target authority",
                            resolved_source_only_policy
                                .descriptor
                                .semantic_name()
                                .as_str(),
                            source_node
                        ),
                    ));
                }
                let decision = if matches!(
                    authority.adoptability,
                    crate::data::trace::MergeAdoptability::Adoptable
                ) {
                    NodeReconciliationDecision::AdoptSourceAuthority
                } else {
                    NodeReconciliationDecision::SkipNonAdoptableSource
                };
                node_plan.push(NodeMergePlan::new(
                    source_node,
                    NodeReconciliationShape::SourceOnlyIntroduction,
                    NodeMergeInputState::new(
                        source_artifact_id,
                        source_cmp.clone(),
                        Some(authority.clone()),
                        true,
                    ),
                    NodeMergeInputState::new(None, None, None, false),
                    decision,
                    Vec::new(),
                ));
                if matches!(decision, NodeReconciliationDecision::AdoptSourceAuthority) {
                    adoption_core.push(SourceNodeAdoptionPlanCore {
                        source_node,
                        target_identity: TargetNodeIdentityIntent::AllocateTargetNode,
                        authority,
                        entry_contract: AdoptedNodeContract {
                            eval_config: source_state
                                .graph()
                                .node_eval_config(source_node)?
                                .clone(),
                        },
                        dependency_topology: AdoptionDependencyTopology {
                            dependencies: source_state
                                .graph()
                                .dependencies_of(source_node)?
                                .to_vec(),
                        },
                        dependency_snapshot_ref: AdoptionDependencySnapshotRef {
                            snapshot: source_state.graph().get_dep_snapshot(source_node)?.clone(),
                        },
                    });
                    adoption_policy.push(SourceNodeAdoptionCarryPolicy {
                        runtime_artifact: RuntimeArtifactCarryPolicy::CarryMergeAdoptable,
                        retained_artifact: RetainedArtifactCarryPolicy::CarryIfPolicyAllows,
                        causality: CausalityCarryPolicy::CarryIfPolicyAllows,
                    });
                }
            }
        }
        let aspect_decision_plan =
            lower_aspect_merge_decision_plan(&aspect_policy_plan, &node_plan);

        Ok(LoweredMergePlan::new(
            request.source_branch.id,
            request.target_branch.id,
            self.schema_registry.registry_digest().to_owned(),
            runtime_proof.registry_bundle_digest.clone(),
            merge_kind,
            divergence,
            merge_strategy,
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
            resolved_identity_matcher.descriptor.semantic_name().clone(),
            resolved_identity_matcher.descriptor.digest().to_string(),
            resolved_identity_matcher.basis,
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
            boundary_witness,
            source_journal,
            target_overlap_journal,
            identity_correspondence,
            deletion_plan,
            conflict_isolation_plan,
            aspect_policy_plan,
            aspect_decision_plan,
            proof_minimal_overlap,
            conservative_overlap,
            planned_candidates,
            source_snapshot_id,
            target_snapshot_id_before,
            Some(resolved_merge_base_record),
            lowered_merge_base,
            resolution_plan,
            node_map,
            node_plan,
            adoption_core,
            adoption_policy,
        ))
    }

    #[cfg(test)]
    pub(crate) fn inspect_branch_merge_plan_for_test(
        &mut self,
        source: SignalBranchHandle,
        target: SignalBranchHandle,
    ) -> Result<BranchMergePlan, SignalError> {
        self.build_branch_merge_plan(&BranchMergeRequest {
            source_branch: source,
            target_branch: target,
            strategy_name: None,
            strategy_hint: None,
            merge_base_name: None,
            conflict_policy_name: None,
            identity_matcher_name: None,
            source_only_policy_name: None,
            deletion_policy_name: None,
            conflict_isolation_policy_name: None,
            aspect_policy_bindings: Vec::new(),
        })
    }

    fn execute_branch_merge_plan(
        &mut self,
        request: &BranchMergeRequest,
        plan: &BranchMergePlan,
    ) -> Result<BranchMergeExecutionSummary, SignalError> {
        let source_state = if request.source_branch.id == self.graph.current_branch().id {
            {
                self.capture_heavy_branch_state()
            }
        } else {
            self.branches
                .branch_state(request.source_branch.id)
                .cloned()
                .ok_or_else(|| {
                    SignalError::unknown_branch(
                        Some(request.source_branch.id),
                        request.source_branch.name.clone(),
                    )
                })?
        };
        let mut target_state = if request.target_branch.id == self.graph.current_branch().id {
            {
                self.capture_heavy_branch_state()
            }
        } else {
            self.branches
                .branch_state(request.target_branch.id)
                .cloned()
                .ok_or_else(|| {
                    SignalError::unknown_branch(
                        Some(request.target_branch.id),
                        request.target_branch.name.clone(),
                    )
                })?
        };

        let mut node_map = plan.node_map().clone();
        let mut dependency_remaps = Vec::new();
        let mut records = Vec::new();
        let mut touched = BTreeSet::new();
        let mut repaired_sources = BTreeSet::new();
        let target_snapshot_before = plan.target_snapshot_id_before();
        let identity_records_by_source = plan
            .identity_correspondence()
            .records
            .iter()
            .map(|record| (record.source_node, record))
            .collect::<BTreeMap<_, _>>();

        for (core, policy) in plan
            .adoption_core()
            .iter()
            .zip(plan.adoption_policy().iter())
        {
            let (materialized, remaps) = adopt_source_node_into_target(
                &mut target_state,
                source_state.graph(),
                core,
                policy,
                &node_map,
            )?;
            node_map.insert(core.source_node, materialized.target_node);
            touched.insert(materialized.target_node);
            repaired_sources.extend(remaps.iter().map(|record| record.target_dependency));
            dependency_remaps.extend(remaps);
        }

        for node_plan in plan.node_plan() {
            match node_plan.shape() {
                NodeReconciliationShape::ExistingTargetNode { target_node } => {
                    let source_image = source_state
                        .graph()
                        .node_checkpoint_image(node_plan.source_node())?;
                    let mut replacement = source_image.clone();
                    let (dependencies_id, dep_snapshot_id) =
                        target_state.graph().node_dependency_ids(target_node)?;
                    replacement.set_dependencies_id(dependencies_id);
                    replacement
                        .set_subscribers_id(target_state.graph().node_subscribers_id(target_node)?);
                    replacement.set_dep_snapshot_id(dep_snapshot_id);
                    if matches!(
                        node_plan.decision(),
                        NodeReconciliationDecision::AdoptSourceAuthority
                    ) {
                        repaired_sources.extend(
                            target_state
                                .graph()
                                .dependencies_of(target_node)?
                                .iter()
                                .map(|edge| edge.source()),
                        );
                        let mapped_edges = source_state
                            .graph()
                            .dependencies_of(node_plan.source_node())?
                            .iter()
                            .map(|edge| {
                                let mapped = node_map.resolve(edge.source()).ok_or_else(|| {
                                    SignalError::invalid_input(format!(
                                        "merge plan has unresolved dependency remap {} for node {}",
                                        edge.source(),
                                        node_plan.source_node()
                                    ))
                                })?;
                                Ok(match edge.scope_ref().cloned() {
                                    Some(scope) => DependencyEdge::with_partition_scope(
                                        mapped,
                                        edge.aspect(),
                                        scope,
                                    ),
                                    None => DependencyEdge::new(mapped, edge.aspect()),
                                })
                            })
                            .collect::<Result<Vec<_>, SignalError>>()?;
                        repaired_sources.extend(mapped_edges.iter().map(|edge| edge.source()));
                        let remapped_snapshot = remap_dependency_snapshot(
                            node_plan.source_node(),
                            source_state
                                .graph()
                                .get_dep_snapshot(node_plan.source_node())?,
                            &node_map,
                        )?;
                        target_state
                            .replace_node_from_checkpoint_image(target_node, replacement)?;
                        target_state
                            .graph_mut()
                            .set_dependencies(target_node, mapped_edges)?;
                        target_state
                            .graph_mut()
                            .set_dep_snapshot(target_node, remapped_snapshot)?;
                        touched.insert(target_node);
                    } else if matches!(
                        node_plan.decision(),
                        NodeReconciliationDecision::MarkEquivalentUnchanged
                    ) && node_plan.resolved_conflict_kinds().iter().any(|kind| {
                        matches!(kind, BranchMergeConflictKind::DependencySnapshotMismatch)
                    }) {
                        let remapped_snapshot = remap_dependency_snapshot(
                            node_plan.source_node(),
                            source_state
                                .graph()
                                .get_dep_snapshot(node_plan.source_node())?,
                            &node_map,
                        )?;
                        target_state
                            .graph_mut()
                            .set_dep_snapshot(target_node, remapped_snapshot)?;
                        touched.insert(target_node);
                    }
                }
                NodeReconciliationShape::SourceOnlyIntroduction => {
                    if let Some(mapped) = node_map.resolve(node_plan.source_node()) {
                        touched.insert(mapped);
                    }
                }
            }
        }

        let merged_snapshot = {
            let policy = target_state.graph().runtime_policy();
            let artifact_retention = SnapshotArtifactRetentionPolicy::from_runtime_policy(policy);
            let meta = target_state
                .graph_mut()
                .diagnostics_state_mut()
                .allocate_snapshot_meta(policy, artifact_retention);
            target_state
                .graph_mut()
                .diagnostics_state_mut()
                .set_branch_head_snapshot(request.target_branch.id, meta.snapshot_id);
            meta.snapshot_id
        };

        let target_snapshot_after = Some(merged_snapshot);
        target_state
            .ancestry_mut()
            .set_latest_merge_reference(Some(LatestMergeReference::new(
                request.source_branch.id,
                plan.source_snapshot_id(),
                target_snapshot_before,
                target_snapshot_after,
                plan.merge_kind(),
                plan.merge_strategy(),
            )));
        target_state
            .mutation_ledger_mut()
            .clear_all(target_snapshot_after);
        target_state.clear_branch_mutation_nodes();

        for node_plan in plan.node_plan() {
            let target_node = match node_plan.shape() {
                NodeReconciliationShape::ExistingTargetNode { target_node } => Some(target_node),
                NodeReconciliationShape::SourceOnlyIntroduction => {
                    node_map.resolve(node_plan.source_node())
                }
            };
            let target_projection_after = target_node
                .map(|node| node_merge_projection(target_state.graph(), node))
                .transpose()?
                .flatten();
            let action = match node_plan.shape() {
                NodeReconciliationShape::SourceOnlyIntroduction => {
                    if matches!(
                        node_plan.decision(),
                        NodeReconciliationDecision::SkipNonAdoptableSource
                    ) {
                        ArtifactMergeAction::SkippedNonAdoptable
                    } else {
                        ArtifactMergeAction::IntroducedIntoTarget
                    }
                }
                NodeReconciliationShape::ExistingTargetNode { .. } => match node_plan.decision() {
                    NodeReconciliationDecision::MarkEquivalentUnchanged => {
                        ArtifactMergeAction::EquivalentUnchanged
                    }
                    NodeReconciliationDecision::PreserveTarget => {
                        ArtifactMergeAction::PreservedTarget
                    }
                    NodeReconciliationDecision::AdoptSourceAuthority => {
                        ArtifactMergeAction::Adopted
                    }
                    NodeReconciliationDecision::ReplaceTargetAuthority => {
                        ArtifactMergeAction::Replaced
                    }
                    NodeReconciliationDecision::SkipNonAdoptableSource => {
                        ArtifactMergeAction::SkippedNonAdoptable
                    }
                    NodeReconciliationDecision::RejectRequiresConflictResolution => {
                        ArtifactMergeAction::SkippedNonAdoptable
                    }
                },
            };
            let basis = match action {
                ArtifactMergeAction::EquivalentUnchanged => MergeDecisionBasis::EquivalentArtifacts,
                ArtifactMergeAction::IntroducedIntoTarget => {
                    MergeDecisionBasis::SourceNodeIntroducedIntoTarget
                }
                ArtifactMergeAction::Adopted => MergeDecisionBasis::SourceAuthorityAdopted,
                ArtifactMergeAction::PreservedTarget => MergeDecisionBasis::MissingSourceArtifact,
                ArtifactMergeAction::Replaced => MergeDecisionBasis::SourceAuthorityAdopted,
                ArtifactMergeAction::SkippedNonAdoptable => {
                    MergeDecisionBasis::TargetPreservedNonAdoptable
                }
            };
            let identity_record = identity_records_by_source.get(&node_plan.source_node());
            records.push(MergedArtifactRecord {
                source_node: node_plan.source_node(),
                target_node,
                source_artifact_id: node_plan.source_state().current_artifact_id(),
                target_artifact_id_before: node_plan.target_state().current_artifact_id(),
                target_artifact_id_after: target_projection_after
                    .as_ref()
                    .and_then(|projection| projection.current_artifact_id),
                action,
                basis,
                source_comparable: node_plan.source_state().comparable().cloned(),
                target_comparable: target_projection_after
                    .as_ref()
                    .map(|projection| projection.comparable.clone()),
                identity_basis: identity_record.and_then(|record| record.basis),
                identity_status: identity_record.map(|record| record.status),
                identity_candidate_count: identity_record
                    .map(|record| record.candidate_count)
                    .unwrap_or_default(),
                resolved_conflict_kinds: node_plan.resolved_conflict_kinds().to_vec(),
            });
        }

        records.sort_by_key(|record| (record.source_node.index(), record.source_node.generation()));
        let touched_set = MergeTouchedNodeSet {
            nodes: touched.into_iter().collect(),
        };
        let counters = BranchMergeCounters {
            boundary_witness_kind: plan.boundary_witness().kind,
            source_slice_breadth: plan.source_journal().breadth(),
            proof_minimal_overlap_breadth: plan.proof_minimal_overlap().breadth(),
            conservative_overlap_expansion_breadth: plan.conservative_overlap().breadth(),
            final_candidate_breadth: plan.planned_candidates().breadth(),
            reconciliation_breadth: plan.node_plan().len() as u64,
            candidate_node_count: plan.node_plan().len() as u64,
            examined_node_count: plan.node_plan().len() as u64,
            adopted_count: records
                .iter()
                .filter(|record| matches!(record.action, ArtifactMergeAction::Adopted))
                .count() as u64,
            introduced_node_count: records
                .iter()
                .filter(|record| matches!(record.action, ArtifactMergeAction::IntroducedIntoTarget))
                .count() as u64,
            replaced_count: records
                .iter()
                .filter(|record| matches!(record.action, ArtifactMergeAction::Replaced))
                .count() as u64,
            preserved_target_count: records
                .iter()
                .filter(|record| matches!(record.action, ArtifactMergeAction::PreservedTarget))
                .count() as u64,
            skipped_non_adoptable_count: records
                .iter()
                .filter(|record| matches!(record.action, ArtifactMergeAction::SkippedNonAdoptable))
                .count() as u64,
            equivalent_unchanged_count: records
                .iter()
                .filter(|record| matches!(record.action, ArtifactMergeAction::EquivalentUnchanged))
                .count() as u64,
            source_only_count: plan
                .node_plan()
                .iter()
                .filter(|node| {
                    matches!(
                        node.shape(),
                        NodeReconciliationShape::SourceOnlyIntroduction
                    )
                })
                .count() as u64,
            target_only_count: plan.deletion_plan().target_only_count,
            dependency_remap_count: dependency_remaps.len() as u64,
            identity_target_candidates_indexed: plan
                .identity_correspondence()
                .target_candidate_count,
            identity_source_lookups: plan.identity_correspondence().source_lookup_count,
            identity_ambiguous_match_count: plan.identity_correspondence().ambiguous_match_count,
            identity_rejected_admissibility_count: plan
                .identity_correspondence()
                .rejected_admissibility_count,
            conflict_isolation_record_count: plan.conflict_isolation_plan().records.len() as u64,
            conflict_isolation_expansion_breadth: plan.conflict_isolation_plan().expansion_breadth,
            subscriber_repair_breadth: repaired_sources.len() as u64,
            merge_lineage_record_count: (records.len() + 1) as u64,
            replay_event_count: 1,
        };
        let summary = BranchMergeExecutionSummary {
            source_branch_id: plan.source_branch_id(),
            target_branch_id: plan.target_branch_id(),
            schema_registry_digest: plan.schema_registry_digest().to_owned(),
            registry_bundle_digest: plan.registry_bundle_digest().to_owned(),
            lowered_strategy_bundle_digest: plan.lowered_strategy_bundle_digest().to_owned(),
            merge_kind: plan.merge_kind(),
            divergence: plan.divergence(),
            merge_strategy: plan.merge_strategy(),
            selected_strategy_name: plan.selected_strategy_name().clone(),
            selected_strategy_digest: plan.selected_strategy_digest().to_string(),
            selected_strategy_basis: plan.selected_strategy_basis(),
            selected_conflict_policy_name: plan.selected_conflict_policy_name().clone(),
            selected_conflict_policy_digest: plan.selected_conflict_policy_digest().to_string(),
            selected_conflict_policy_basis: plan.selected_conflict_policy_basis(),
            selected_conflict_isolation_name: plan.selected_conflict_isolation_name().clone(),
            selected_conflict_isolation_digest: plan
                .selected_conflict_isolation_digest()
                .to_string(),
            selected_conflict_isolation_basis: plan.selected_conflict_isolation_basis(),
            selected_identity_matcher_name: plan.selected_identity_matcher_name().clone(),
            selected_identity_matcher_digest: plan.selected_identity_matcher_digest().to_string(),
            selected_identity_matcher_basis: plan.selected_identity_matcher_basis(),
            selected_source_only_policy_name: plan.selected_source_only_policy_name().clone(),
            selected_source_only_policy_digest: plan
                .selected_source_only_policy_digest()
                .to_string(),
            selected_source_only_policy_basis: plan.selected_source_only_policy_basis(),
            selected_deletion_policy_name: plan.selected_deletion_policy_name().clone(),
            selected_deletion_policy_digest: plan.selected_deletion_policy_digest().to_string(),
            selected_deletion_policy_basis: plan.selected_deletion_policy_basis(),
            selected_merge_base_name: plan
                .lowered_merge_base()
                .map(|base| base.selected_merge_base_name.clone())
                .expect("merge-base plan"),
            selected_merge_base_digest: plan
                .lowered_merge_base()
                .map(|base| base.selected_merge_base_digest.clone())
                .expect("merge-base plan"),
            selected_merge_base_basis: plan
                .lowered_merge_base()
                .map(|base| base.selected_merge_base_basis)
                .expect("merge-base plan"),
            selected_semantics: plan.selected_semantics().clone(),
            reconciliation_policy: plan.reconciliation_policy().clone(),
            boundary_witness: plan.boundary_witness().clone(),
            identity_correspondence: plan.identity_correspondence().clone(),
            deletion_plan: plan.deletion_plan().clone(),
            conflict_isolation_plan: plan.conflict_isolation_plan().clone(),
            aspect_policy_plan: plan.aspect_policy_plan().clone(),
            aspect_decision_plan: plan.aspect_decision_plan().clone(),
            proof_minimal_overlap: plan.proof_minimal_overlap().clone(),
            conservative_overlap: plan.conservative_overlap().clone(),
            planned_candidates: plan.planned_candidates().clone(),
            merge_base: plan.merge_base().cloned(),
            lowered_merge_base: plan.lowered_merge_base().cloned(),
            source_snapshot_id: plan.source_snapshot_id(),
            target_snapshot_id_before: target_snapshot_before,
            target_snapshot_id_after: target_snapshot_after,
            resolution_plan: plan.resolution_plan().cloned(),
            node_map,
            records,
            dependency_remaps,
            topology_repair: TopologyRepairSummary {
                touched_node_count: touched_set.nodes.len() as u64,
                subscriber_repair_breadth: repaired_sources.len() as u64,
            },
            counters,
        };

        let merged_source_nodes = summary
            .records
            .iter()
            .filter(|record| !matches!(record.action, ArtifactMergeAction::SkippedNonAdoptable))
            .map(|record| record.source_node)
            .collect::<Vec<_>>();

        let target_snapshot_packet =
            crate::logic::transaction::runtime::state::branching::SnapshotBranchState::from_branch_state(&target_state)
                .packet(merged_snapshot);

        if request.target_branch.id == self.graph.current_branch().id {
            self.apply_branch_lifecycle_transfer(
                crate::logic::transaction::runtime::state::runtime_state::BranchLifecycleTransfer::Move(
                    crate::logic::transaction::runtime::state::runtime_state::AuthorityTransferPacket::new(
                        request.target_branch.id,
                        target_state,
                    ),
                ),
            )?;
        } else {
            self.branches.store_branch_state(target_state);
        }
        if request.source_branch.id == self.graph.current_branch().id {
            let mut updated_source_state = self.capture_heavy_branch_state();
            updated_source_state
                .mutation_ledger_mut()
                .clear_merged_nodes(
                    merged_source_nodes.iter().copied(),
                    plan.source_snapshot_id(),
                );
            updated_source_state.clear_branch_mutation_nodes();
            self.apply_branch_lifecycle_transfer(
                crate::logic::transaction::runtime::state::runtime_state::BranchLifecycleTransfer::Move(
                    crate::logic::transaction::runtime::state::runtime_state::AuthorityTransferPacket::new(
                        request.source_branch.id,
                        updated_source_state,
                    ),
                ),
            )?;
        } else if let Some(()) =
            self.branches
                .with_stored_branch_state_mut(request.source_branch.id, |source_state| {
                    source_state.mutation_ledger_mut().clear_merged_nodes(
                        merged_source_nodes.iter().copied(),
                        plan.source_snapshot_id(),
                    );
                    source_state.clear_branch_mutation_nodes();
                })
        {
        }
        self.branches.insert_snapshot(target_snapshot_packet);
        let branch_catalog = if request.target_branch.id == self.graph.current_branch().id {
            self.graph.diagnostics_state().branch_catalog().clone()
        } else {
            self.branches
                .branch_state(request.target_branch.id)
                .map(|state| state.graph().diagnostics_state().branch_catalog().clone())
                .unwrap_or_default()
        };
        self.synchronize_branch_catalogs(branch_catalog);
        Ok(summary)
    }
}

fn classify_conflict_kinds(
    source_cmp: Option<&crate::logic::transaction::ArtifactMergeComparable>,
    target_cmp: Option<&crate::logic::transaction::ArtifactMergeComparable>,
    source_structural_record: Option<&StructuralMergeCandidateRecord>,
    target_structural_record: Option<&StructuralMergeCandidateRecord>,
) -> Vec<BranchMergeConflictKind> {
    let mut kinds = Vec::new();
    if source_cmp != target_cmp {
        kinds.push(BranchMergeConflictKind::ComparableMismatch);
    }
    if source_cmp.map(|cmp| &cmp.authority) != target_cmp.map(|cmp| &cmp.authority) {
        kinds.push(BranchMergeConflictKind::MergeAuthorityMismatch);
    }
    if structural_delta_conflicts(
        source_structural_record,
        target_structural_record,
        StructuralConflictFacet::DependencyTopology,
    ) {
        kinds.push(BranchMergeConflictKind::DependencyTopologyMismatch);
    }
    if structural_delta_conflicts(
        source_structural_record,
        target_structural_record,
        StructuralConflictFacet::DependencySnapshot,
    ) {
        kinds.push(BranchMergeConflictKind::DependencySnapshotMismatch);
    }
    if structural_delta_conflicts(
        source_structural_record,
        target_structural_record,
        StructuralConflictFacet::RuntimeArtifact,
    ) {
        kinds.push(BranchMergeConflictKind::RuntimeArtifactMismatch);
    }
    kinds
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeMergeProjection {
    comparable: crate::logic::transaction::ArtifactMergeComparable,
    current_artifact_id: Option<crate::diagnostics::lineage::LineageArtifactId>,
    authority: crate::data::trace::ArtifactMergeAuthority,
}

fn node_merge_projection(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<Option<NodeMergeProjection>, SignalError> {
    let hot = graph.node_runtime_artifact_hot(node)?;
    let warm = graph.node_runtime_artifact_warm(node)?;
    match (hot, warm) {
        (Some(hot), Some(warm)) => Ok(Some(NodeMergeProjection {
            comparable: merge_comparable_from_lanes(hot, warm),
            current_artifact_id: warm.lineage_artifact_id.get(),
            authority: warm.merge_authority.clone(),
        })),
        (None, None) => Ok(None),
        _ => Err(SignalError::internal(format!(
            "runtime artifact hot/warm lane mismatch for merge-comparable node {}",
            node
        ))),
    }
}

fn merge_comparable_from_lanes(
    hot: &RuntimeArtifactHot,
    warm: &RuntimeArtifactWarm,
) -> crate::logic::transaction::ArtifactMergeComparable {
    crate::logic::transaction::ArtifactMergeComparable {
        output_identity: warm.output_identity.clone(),
        continuity_token: warm.continuity_token.clone_inner(),
        reuse_basis: warm.reuse_basis.clone_inner(),
        dependency_fingerprint: crate::logic::transaction::DependencyFingerprint {
            dependency_count: hot.dependency_count,
            meaningful_input_changes: hot.meaningful_input_changes,
            output_hash: hot.output_hash,
        },
        authority: warm.merge_authority.clone(),
    }
}

#[derive(Clone, Copy)]
enum StructuralConflictFacet {
    DependencyTopology,
    DependencySnapshot,
    RuntimeArtifact,
}

fn structural_delta_conflicts(
    source_structural_record: Option<&StructuralMergeCandidateRecord>,
    target_structural_record: Option<&StructuralMergeCandidateRecord>,
    facet: StructuralConflictFacet,
) -> bool {
    let source = structural_deltas_for_facet(source_structural_record, facet);
    let target = structural_deltas_for_facet(target_structural_record, facet);
    source != target
}

fn structural_deltas_for_facet(
    record: Option<&StructuralMergeCandidateRecord>,
    facet: StructuralConflictFacet,
) -> Vec<crate::data::graph::BranchStructuralDelta> {
    record
        .map(|record| {
            record
                .structural_deltas
                .iter()
                .filter(|delta| match (facet, delta) {
                    (
                        StructuralConflictFacet::DependencyTopology,
                        crate::data::graph::BranchStructuralDelta::DependencyTopologyChanged(_),
                    ) => true,
                    (
                        StructuralConflictFacet::DependencySnapshot,
                        crate::data::graph::BranchStructuralDelta::DependencySnapshotChanged(_),
                    ) => true,
                    (
                        StructuralConflictFacet::RuntimeArtifact,
                        crate::data::graph::BranchStructuralDelta::RuntimeArtifactChanged(_),
                    ) => true,
                    _ => false,
                })
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

fn summarize_conflict_records(records: &[BranchMergeConflictRecord]) -> BranchMergeConflictSummary {
    let mut summary = BranchMergeConflictSummary {
        total_conflict_count: records.len() as u64,
        ..BranchMergeConflictSummary::default()
    };
    for record in records {
        if record
            .conflict_kinds
            .contains(&BranchMergeConflictKind::ComparableMismatch)
        {
            summary.comparable_mismatch_count += 1;
        }
        if record
            .conflict_kinds
            .contains(&BranchMergeConflictKind::DependencyTopologyMismatch)
        {
            summary.dependency_topology_mismatch_count += 1;
        }
        if record
            .conflict_kinds
            .contains(&BranchMergeConflictKind::DependencySnapshotMismatch)
        {
            summary.dependency_snapshot_mismatch_count += 1;
        }
        if record
            .conflict_kinds
            .contains(&BranchMergeConflictKind::RuntimeArtifactMismatch)
        {
            summary.runtime_artifact_mismatch_count += 1;
        }
        if record
            .conflict_kinds
            .contains(&BranchMergeConflictKind::MergeAuthorityMismatch)
        {
            summary.merge_authority_mismatch_count += 1;
        }
    }
    let counts = [
        (
            BranchMergeConflictKind::ComparableMismatch,
            summary.comparable_mismatch_count,
        ),
        (
            BranchMergeConflictKind::DependencyTopologyMismatch,
            summary.dependency_topology_mismatch_count,
        ),
        (
            BranchMergeConflictKind::DependencySnapshotMismatch,
            summary.dependency_snapshot_mismatch_count,
        ),
        (
            BranchMergeConflictKind::RuntimeArtifactMismatch,
            summary.runtime_artifact_mismatch_count,
        ),
        (
            BranchMergeConflictKind::MergeAuthorityMismatch,
            summary.merge_authority_mismatch_count,
        ),
    ];
    summary.primary_conflict_kind = counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .and_then(|(kind, count)| if count > 0 { Some(kind) } else { None });
    let mut required_resolution = Vec::new();
    if summary.comparable_mismatch_count > 0 {
        required_resolution.push(BranchMergeResolutionRequirement::ReconcileComparableState);
    }
    if summary.dependency_topology_mismatch_count > 0 {
        required_resolution.push(BranchMergeResolutionRequirement::ReconcileDependencyTopology);
    }
    if summary.dependency_snapshot_mismatch_count > 0 {
        required_resolution.push(BranchMergeResolutionRequirement::ReconcileDependencySnapshot);
    }
    if summary.runtime_artifact_mismatch_count > 0 {
        required_resolution.push(BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState);
    }
    if summary.merge_authority_mismatch_count > 0 {
        required_resolution.push(BranchMergeResolutionRequirement::ReconcileMergeAuthority);
    }
    summary.required_resolution = required_resolution;
    summary
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
        ConflictMergePolicy::RejectSharedStateConflict => false,
        ConflictMergePolicy::ResolveSourceStateWhenStructureMatches => {
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
                    && (!record
                        .required_resolution
                        .contains(&BranchMergeResolutionRequirement::ReconcileComparableState)
                        || record
                            .supported_strategies
                            .contains(&ConflictResolutionStrategy::AdoptSourceComparableState))
                    && (!record
                        .required_resolution
                        .contains(&BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState)
                        || record
                            .supported_strategies
                            .contains(&ConflictResolutionStrategy::AdoptSourceRuntimeArtifactState))
                    && (!record
                        .required_resolution
                        .contains(&BranchMergeResolutionRequirement::ReconcileDependencySnapshot)
                        || record
                            .supported_strategies
                            .contains(&ConflictResolutionStrategy::ReplaySourceDependencySnapshot))
            })
        }
    }
}

fn resolve_merge_base_descriptor(
    registry: &FrozenMergeBaseStrategyRegistry,
    request: &BranchMergeRequest,
    default_policy: MergeBaseSelectionPolicy,
) -> Result<ResolvedMergeBaseSelection, SignalError> {
    if let Some(strategy_name) = request.merge_base_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(strategy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge-base strategy `{}` is not registered in the frozen merge-base strategy registry",
                        strategy_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedMergeBaseSelection {
            descriptor,
            basis: MergeBaseSelectionBasis::RequestNamed,
        });
    }

    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::MissingMergeBase,
                "no built-in merge-base strategy matches the default selection policy",
            )
        })?;
    Ok(ResolvedMergeBaseSelection {
        descriptor,
        basis: MergeBaseSelectionBasis::BuiltInDefault,
    })
}

fn resolve_merge_strategy_descriptor(
    registry: &FrozenMergeStrategyRegistry,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_strategy: BranchMergeStrategy,
) -> Result<ResolvedMergeStrategySelection, SignalError> {
    if let Some(strategy_name) = request.strategy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(strategy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge strategy `{}` is not registered in the frozen merge strategy registry",
                        strategy_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedMergeStrategySelection {
            descriptor,
            basis: MergeStrategySelectionBasis::RequestNamed,
        });
    }

    if let Some(strategy_hint) = request.strategy_hint {
        let descriptor = registry
            .first_matching_strategy(strategy_hint)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge strategy {:?} has no registered descriptor in the frozen merge strategy registry",
                        strategy_hint
                    ),
                )
            })?;
        return Ok(ResolvedMergeStrategySelection {
            descriptor,
            basis: MergeStrategySelectionBasis::RequestHint,
        });
    }

    if let Some(node_override_name) = unanimous_node_override_name(source_graph, candidate_nodes)? {
        let descriptor = registry
            .resolve_by_name(&node_override_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "node merge strategy override `{}` is not registered in the frozen merge strategy registry",
                        node_override_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedMergeStrategySelection {
            descriptor,
            basis: MergeStrategySelectionBasis::NodeOverride,
        });
    }

    if let Some(schema_default_name) =
        unanimous_schema_default_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&schema_default_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "schema default merge strategy `{}` is not registered in the frozen merge strategy registry",
                        schema_default_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedMergeStrategySelection {
            descriptor,
            basis: MergeStrategySelectionBasis::SchemaDefault,
        });
    }

    let descriptor = registry
        .first_matching_strategy(default_strategy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "merge strategy {:?} has no registered descriptor in the frozen merge strategy registry",
                    default_strategy
                ),
            )
        })?;
    Ok(ResolvedMergeStrategySelection {
        descriptor,
        basis: MergeStrategySelectionBasis::DivergenceDefault,
    })
}

fn resolve_conflict_policy_descriptor(
    registry: &FrozenConflictPolicyRegistry,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_policy: ConflictMergePolicy,
) -> Result<ResolvedConflictPolicySelection, SignalError> {
    if let Some(policy_name) = request.conflict_policy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(policy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "conflict policy `{}` is not registered in the frozen conflict policy registry",
                        policy_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedConflictPolicySelection {
            descriptor,
            basis: ConflictPolicySelectionBasis::RequestNamed,
        });
    }

    if let Some(policy_name) = unanimous_node_conflict_policy_name(source_graph, candidate_nodes)? {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "node conflict policy override `{}` is not registered in the frozen conflict policy registry",
                        policy_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedConflictPolicySelection {
            descriptor,
            basis: ConflictPolicySelectionBasis::NodeOverride,
        });
    }

    if let Some(policy_name) =
        unanimous_schema_conflict_policy_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "schema default conflict policy `{}` is not registered in the frozen conflict policy registry",
                        policy_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedConflictPolicySelection {
            descriptor,
            basis: ConflictPolicySelectionBasis::SchemaDefault,
        });
    }

    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "conflict policy {:?} has no registered descriptor in the frozen conflict policy registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedConflictPolicySelection {
        descriptor,
        basis: ConflictPolicySelectionBasis::BuiltInDefault,
    })
}

fn resolve_identity_matcher_descriptor(
    registry: &FrozenIdentityMatcherRegistry,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_policy: IdentityMatchPolicy,
) -> Result<ResolvedIdentityMatcherSelection, SignalError> {
    if let Some(matcher_name) = request.identity_matcher_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(matcher_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "identity matcher `{}` is not registered in the frozen identity matcher registry",
                        matcher_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedIdentityMatcherSelection {
            descriptor,
            basis: IdentityMatcherSelectionBasis::RequestNamed,
        });
    }

    if let Some(matcher_name) = unanimous_node_identity_matcher_name(source_graph, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&matcher_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "node identity matcher override `{}` is not registered in the frozen identity matcher registry",
                        matcher_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedIdentityMatcherSelection {
            descriptor,
            basis: IdentityMatcherSelectionBasis::NodeOverride,
        });
    }

    if let Some(matcher_name) =
        unanimous_schema_identity_matcher_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&matcher_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "schema default identity matcher `{}` is not registered in the frozen identity matcher registry",
                        matcher_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedIdentityMatcherSelection {
            descriptor,
            basis: IdentityMatcherSelectionBasis::SchemaDefault,
        });
    }

    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "identity matcher {:?} has no registered descriptor in the frozen identity matcher registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedIdentityMatcherSelection {
        descriptor,
        basis: IdentityMatcherSelectionBasis::BuiltInDefault,
    })
}

fn resolve_identity_matches(
    matcher_name: &crate::logic::transaction::runtime::IdentityMatcherName,
    policy: IdentityMatchPolicy,
    source_graph: &SignalGraph,
    target_graph: &SignalGraph,
    source_nodes: &[NodeId],
    target_identity_journal: &crate::logic::transaction::BranchMutationJournalSlice,
) -> Result<IdentityResolutionOutcome, SignalError> {
    let mut matches = BTreeMap::new();
    let mut used_target_nodes = BTreeSet::new();
    let mut records = Vec::new();
    let mut source_lookup_count = 0u64;
    let mut rejected_admissibility_count = 0u64;

    for source_node in source_nodes {
        if target_graph.is_alive(*source_node) {
            matches.insert(*source_node, *source_node);
            used_target_nodes.insert(*source_node);
            let source_projection = node_merge_projection(source_graph, *source_node)?;
            let target_projection = node_merge_projection(target_graph, *source_node)?;
            records.push(IdentityCorrespondenceRecord {
                source_node: *source_node,
                target_node: Some(*source_node),
                basis: Some(IdentityCorrespondenceBasis::ExactNodeId),
                status: IdentityCorrespondenceStatus::Matched,
                source_output_identity: source_projection
                    .as_ref()
                    .and_then(|projection| projection.comparable.output_identity.clone()),
                target_output_identity: target_projection
                    .as_ref()
                    .and_then(|projection| projection.comparable.output_identity.clone()),
                candidate_count: 1,
                candidate_target_nodes: vec![*source_node],
                admissibility_rejection: None,
            });
        }
    }

    if !matches!(
        policy,
        IdentityMatchPolicy::OutputIdentityWithinTargetJournal
    ) {
        for source_node in source_nodes {
            if matches.contains_key(source_node) {
                continue;
            }
            let source_projection = node_merge_projection(source_graph, *source_node)?;
            records.push(IdentityCorrespondenceRecord {
                source_node: *source_node,
                target_node: None,
                basis: None,
                status: IdentityCorrespondenceStatus::UnmatchedNoCandidate,
                source_output_identity: source_projection
                    .as_ref()
                    .and_then(|projection| projection.comparable.output_identity.clone()),
                target_output_identity: None,
                candidate_count: 0,
                candidate_target_nodes: Vec::new(),
                admissibility_rejection: None,
            });
        }
        return Ok(IdentityResolutionOutcome {
            matches,
            correspondence: LoweredIdentityCorrespondencePlan {
                target_candidate_count: target_identity_journal.records.len() as u64,
                source_lookup_count,
                ambiguous_match_count: 0,
                rejected_admissibility_count,
                records,
            },
        });
    }

    let mut target_index: BTreeMap<_, Vec<(NodeId, Option<crate::data::output::OutputIdentity>)>> =
        BTreeMap::new();
    for record in &target_identity_journal.records {
        let projection = node_merge_projection(target_graph, record.node)?;
        let output_identity = projection
            .as_ref()
            .and_then(|projection| projection.comparable.output_identity.clone());
        if let Some(identity) = output_identity.clone() {
            target_index
                .entry(identity)
                .or_default()
                .push((record.node, output_identity));
        }
    }
    let ambiguous_match_count = 0u64;

    for source_node in source_nodes {
        if matches.contains_key(source_node) {
            continue;
        }
        source_lookup_count += 1;
        let Some(source_projection) = node_merge_projection(source_graph, *source_node)? else {
            records.push(IdentityCorrespondenceRecord {
                source_node: *source_node,
                target_node: None,
                basis: None,
                status: IdentityCorrespondenceStatus::UnmatchedNoCandidate,
                source_output_identity: None,
                target_output_identity: None,
                candidate_count: 0,
                candidate_target_nodes: Vec::new(),
                admissibility_rejection: None,
            });
            continue;
        };
        let Some(source_output_identity) = source_projection.comparable.output_identity.clone()
        else {
            records.push(IdentityCorrespondenceRecord {
                source_node: *source_node,
                target_node: None,
                basis: None,
                status: IdentityCorrespondenceStatus::UnmatchedNoCandidate,
                source_output_identity: None,
                target_output_identity: None,
                candidate_count: 0,
                candidate_target_nodes: Vec::new(),
                admissibility_rejection: None,
            });
            continue;
        };

        let source_contract = source_graph
            .node_eval_config(*source_node)?
            .contract
            .clone();
        let raw_candidates = target_index
            .get(&source_output_identity)
            .cloned()
            .unwrap_or_default();
        let mut candidates = Vec::new();
        let mut admissibility_rejection = None;
        for (target_node, target_identity) in raw_candidates {
            if used_target_nodes.contains(&target_node) {
                continue;
            }
            let target_contract = target_graph.node_eval_config(target_node)?.contract.clone();
            let source_binding = source_graph.node_schema_binding(*source_node)?;
            let target_binding = target_graph.node_schema_binding(target_node)?;
            let schema_compatible = matches!(
                (source_binding, target_binding),
                (Some(source_binding), Some(target_binding))
                    if source_binding.schema_id() == target_binding.schema_id()
            );
            let source_admits = source_contract
                .reuse
                .equivalence
                .supports_strategy(ReuseStrategy::CrossIdentityPersistentMatch);
            let target_admits = target_contract
                .reuse
                .equivalence
                .supports_strategy(ReuseStrategy::CrossIdentityPersistentMatch);
            if !(schema_compatible && source_admits && target_admits) {
                rejected_admissibility_count += 1;
                admissibility_rejection = Some(
                    "output-identity matching requires same schema binding and cross-identity persistent matching on both node contracts"
                        .to_string(),
                );
                continue;
            }
            candidates.push((target_node, target_identity));
        }

        if candidates.len() > 1 {
            let candidate_target_nodes = candidates
                .iter()
                .map(|(target_node, _)| *target_node)
                .collect::<Vec<_>>();
            records.push(IdentityCorrespondenceRecord {
                source_node: *source_node,
                target_node: None,
                basis: Some(IdentityCorrespondenceBasis::OutputIdentityTargetJournal),
                status: IdentityCorrespondenceStatus::AmbiguousCandidates,
                source_output_identity: Some(source_output_identity.clone()),
                target_output_identity: None,
                candidate_count: candidate_target_nodes.len() as u32,
                candidate_target_nodes: candidate_target_nodes.clone(),
                admissibility_rejection: None,
            });
            let correspondence = LoweredIdentityCorrespondencePlan {
                target_candidate_count: target_identity_journal.records.len() as u64,
                source_lookup_count,
                ambiguous_match_count: 1,
                rejected_admissibility_count,
                records,
            };
            return Err(SignalError::branch_merge_failed_with_evidence(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "identity matcher found ambiguous target journal correspondence for source node {} and output identity",
                    source_node
                ),
                BranchMergeFailureEvidence::Identity(BranchMergeIdentityFailureEvidence {
                    identity_matcher_name: matcher_name.clone(),
                    source_node: *source_node,
                    source_output_identity: Some(source_output_identity),
                    candidate_target_nodes,
                    correspondence,
                }),
            ));
        }
        if let Some((target_node, target_output_identity)) = candidates.first().cloned() {
            matches.insert(*source_node, target_node);
            used_target_nodes.insert(target_node);
            records.push(IdentityCorrespondenceRecord {
                source_node: *source_node,
                target_node: Some(target_node),
                basis: Some(IdentityCorrespondenceBasis::OutputIdentityTargetJournal),
                status: IdentityCorrespondenceStatus::Matched,
                source_output_identity: Some(source_output_identity),
                target_output_identity,
                candidate_count: 1,
                candidate_target_nodes: vec![target_node],
                admissibility_rejection: None,
            });
        } else {
            let status = if admissibility_rejection.is_some() {
                IdentityCorrespondenceStatus::UnmatchedRejectedAdmissibility
            } else {
                IdentityCorrespondenceStatus::UnmatchedNoCandidate
            };
            records.push(IdentityCorrespondenceRecord {
                source_node: *source_node,
                target_node: None,
                basis: None,
                status,
                source_output_identity: Some(source_output_identity),
                target_output_identity: None,
                candidate_count: 0,
                candidate_target_nodes: Vec::new(),
                admissibility_rejection,
            });
        }
    }

    Ok(IdentityResolutionOutcome {
        matches,
        correspondence: LoweredIdentityCorrespondencePlan {
            target_candidate_count: target_identity_journal.records.len() as u64,
            source_lookup_count,
            ambiguous_match_count,
            rejected_admissibility_count,
            records,
        },
    })
}

fn resolve_source_only_policy_descriptor(
    registry: &FrozenSourceOnlyPolicyRegistry,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_policy: SourceOnlyMergePolicy,
) -> Result<ResolvedSourceOnlyPolicySelection, SignalError> {
    if let Some(policy_name) = request.source_only_policy_name.as_ref() {
        let descriptor = registry.resolve_by_name(policy_name).cloned().ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "source-only policy `{}` is not registered in the frozen source-only policy registry",
                    policy_name.as_str()
                ),
            )
        })?;
        return Ok(ResolvedSourceOnlyPolicySelection {
            descriptor,
            basis: SourceOnlyPolicySelectionBasis::RequestNamed,
        });
    }

    if let Some(policy_name) =
        unanimous_node_source_only_policy_name(source_graph, candidate_nodes)?
    {
        let descriptor = registry.resolve_by_name(&policy_name).cloned().ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node source-only policy override `{}` is not registered in the frozen source-only policy registry",
                    policy_name.as_str()
                ),
            )
        })?;
        return Ok(ResolvedSourceOnlyPolicySelection {
            descriptor,
            basis: SourceOnlyPolicySelectionBasis::NodeOverride,
        });
    }

    if let Some(policy_name) =
        unanimous_schema_source_only_policy_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry.resolve_by_name(&policy_name).cloned().ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "schema default source-only policy `{}` is not registered in the frozen source-only policy registry",
                    policy_name.as_str()
                ),
            )
        })?;
        return Ok(ResolvedSourceOnlyPolicySelection {
            descriptor,
            basis: SourceOnlyPolicySelectionBasis::SchemaDefault,
        });
    }

    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "source-only policy {:?} has no registered descriptor in the frozen source-only policy registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedSourceOnlyPolicySelection {
        descriptor,
        basis: SourceOnlyPolicySelectionBasis::BuiltInDefault,
    })
}

fn resolve_deletion_policy_descriptor(
    registry: &FrozenDeletionPolicyRegistry,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_policy: DeletionMergePolicy,
) -> Result<ResolvedDeletionPolicySelection, SignalError> {
    if let Some(policy_name) = request.deletion_policy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(policy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                    "deletion policy `{}` is not registered in the frozen deletion policy registry",
                    policy_name.as_str()
                ),
                )
            })?;
        return Ok(ResolvedDeletionPolicySelection {
            descriptor,
            basis: DeletionPolicySelectionBasis::RequestNamed,
        });
    }

    if let Some(policy_name) = unanimous_node_deletion_policy_name(source_graph, candidate_nodes)? {
        let descriptor = registry.resolve_by_name(&policy_name).cloned().ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node deletion policy override `{}` is not registered in the frozen deletion policy registry",
                    policy_name.as_str()
                ),
            )
        })?;
        return Ok(ResolvedDeletionPolicySelection {
            descriptor,
            basis: DeletionPolicySelectionBasis::NodeOverride,
        });
    }

    if let Some(policy_name) =
        unanimous_schema_deletion_policy_name(source_graph, schema_registry, candidate_nodes)?
    {
        let descriptor = registry.resolve_by_name(&policy_name).cloned().ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "schema default deletion policy `{}` is not registered in the frozen deletion policy registry",
                    policy_name.as_str()
                ),
            )
        })?;
        return Ok(ResolvedDeletionPolicySelection {
            descriptor,
            basis: DeletionPolicySelectionBasis::SchemaDefault,
        });
    }

    let descriptor = registry
        .first_matching_policy(default_policy)
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "deletion policy {:?} has no registered descriptor in the frozen deletion policy registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedDeletionPolicySelection {
        descriptor,
        basis: DeletionPolicySelectionBasis::BuiltInDefault,
    })
}

fn lower_aspect_policy_plan(
    registry: &FrozenAspectMergePolicyRegistry,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
) -> Result<LoweredAspectMergePolicyPlan, SignalError> {
    let mut nodes_by_aspect: BTreeMap<u8, Vec<NodeId>> = BTreeMap::new();
    for node in candidate_nodes {
        let config = source_graph.node_eval_config(*node)?;
        let produces = config.contract.semantics.produces;
        if produces == AspectMask::ALL || produces.is_empty() {
            continue;
        }
        for aspect in iter_declared_aspects(produces) {
            nodes_by_aspect.entry(aspect.id()).or_default().push(*node);
        }
    }

    let mut records = Vec::new();
    for (aspect_id, affected_source_nodes) in nodes_by_aspect {
        let aspect = Aspect::new(aspect_id);
        let resolved = resolve_aspect_policy_descriptor(
            registry,
            schema_registry,
            source_graph,
            &affected_source_nodes,
            request,
            aspect,
            AspectMergePolicy::RequireConflict,
        )?;
        records.push(LoweredAspectMergePolicyRecord {
            aspect,
            selected_policy_name: resolved.descriptor.semantic_name().clone(),
            selected_policy_digest: resolved.descriptor.digest().to_string(),
            selected_policy_basis: resolved.basis,
            affected_source_nodes,
        });
    }

    Ok(LoweredAspectMergePolicyPlan { records })
}

fn lower_conflict_isolation_plan(
    selected_policy_name: ConflictIsolationPolicyName,
    selected_policy_digest: String,
    selected_policy_basis: ConflictIsolationSelectionBasis,
    granularity: ConflictIsolationGranularity,
    source_graph: &SignalGraph,
    conflict_records: &[BranchMergeConflictRecord],
) -> Result<LoweredConflictIsolationPlan, SignalError> {
    let mut records = Vec::new();
    for record in conflict_records {
        let isolated_aspects = match granularity {
            ConflictIsolationGranularity::PerAspect => {
                let produces = source_graph
                    .node_eval_config(record.source_node)?
                    .contract
                    .semantics
                    .produces;
                if produces == AspectMask::ALL || produces.is_empty() {
                    Vec::new()
                } else {
                    iter_declared_aspects(produces).collect()
                }
            }
            ConflictIsolationGranularity::PerNode
            | ConflictIsolationGranularity::HostDeclaredRegion => Vec::new(),
        };
        records.push(LoweredConflictIsolationRecord {
            source_node: record.source_node,
            target_node: Some(record.target_node),
            granularity,
            isolated_aspects,
        });
    }
    Ok(LoweredConflictIsolationPlan {
        selected_policy_name: Some(selected_policy_name),
        selected_policy_digest: Some(selected_policy_digest),
        selected_policy_basis: Some(selected_policy_basis),
        expansion_breadth: 0,
        witness: Some(ConflictIsolationWitness {
            granularity,
            conflict_record_count: conflict_records.len() as u64,
        }),
        region_summary: RegionIsolationSummary {
            isolated_region_count: records.len() as u64,
            host_declared_region_count: u64::from(matches!(
                granularity,
                ConflictIsolationGranularity::HostDeclaredRegion
            )),
        },
        conservative_expansion: ConservativeIsolationExpansion {
            expanded_node_count: 0,
        },
        records,
    })
}

fn lower_aspect_merge_decision_plan(
    aspect_policy_plan: &LoweredAspectMergePolicyPlan,
    node_plan: &[NodeMergePlan],
) -> LoweredAspectMergeDecisionPlan {
    let node_plan_by_source = node_plan
        .iter()
        .map(|plan| (plan.source_node(), plan))
        .collect::<BTreeMap<_, _>>();
    let mut records = Vec::new();

    for policy_record in &aspect_policy_plan.records {
        for source_node in &policy_record.affected_source_nodes {
            let Some(node_plan) = node_plan_by_source.get(source_node) else {
                continue;
            };
            let target_node = match node_plan.shape() {
                NodeReconciliationShape::ExistingTargetNode { target_node } => Some(target_node),
                NodeReconciliationShape::SourceOnlyIntroduction => None,
            };
            let outcome = match node_plan.decision() {
                NodeReconciliationDecision::AdoptSourceAuthority => {
                    if matches!(
                        node_plan.shape(),
                        NodeReconciliationShape::SourceOnlyIntroduction
                    ) {
                        AspectMergeDecisionOutcome::SourceIntroducedIntoTarget
                    } else {
                        AspectMergeDecisionOutcome::SourceAuthorityAdopted
                    }
                }
                NodeReconciliationDecision::MarkEquivalentUnchanged => {
                    AspectMergeDecisionOutcome::EquivalentUnchanged
                }
                NodeReconciliationDecision::PreserveTarget => {
                    AspectMergeDecisionOutcome::TargetPreserved
                }
                NodeReconciliationDecision::SkipNonAdoptableSource => {
                    AspectMergeDecisionOutcome::SourceSkippedNonAdoptable
                }
                NodeReconciliationDecision::ReplaceTargetAuthority => {
                    AspectMergeDecisionOutcome::SourceAuthorityAdopted
                }
                NodeReconciliationDecision::RejectRequiresConflictResolution => {
                    AspectMergeDecisionOutcome::ConflictRequired
                }
            };
            records.push(LoweredAspectMergeDecisionRecord {
                aspect: policy_record.aspect,
                source_node: *source_node,
                target_node,
                selected_policy_name: policy_record.selected_policy_name.clone(),
                selected_policy_digest: policy_record.selected_policy_digest.clone(),
                selected_policy_basis: policy_record.selected_policy_basis,
                outcome,
            });
        }
    }

    LoweredAspectMergeDecisionPlan { records }
}

fn iter_declared_aspects(mask: AspectMask) -> impl Iterator<Item = Aspect> {
    (0..crate::data::aspect::MAX_ASPECTS)
        .map(|index| Aspect::new(index as u8))
        .filter(move |aspect| mask.contains(AspectMask::from_aspect(*aspect)))
}

fn resolve_aspect_policy_descriptor(
    registry: &FrozenAspectMergePolicyRegistry,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    aspect: Aspect,
    default_policy: AspectMergePolicy,
) -> Result<ResolvedAspectPolicySelection, SignalError> {
    let request_bindings = request
        .aspect_policy_bindings
        .iter()
        .filter(|binding| binding.aspect() == aspect)
        .map(|binding| binding.policy_name().clone())
        .collect::<Vec<_>>();
    if request_bindings.len() > 1 {
        return Err(SignalError::branch_merge_failed(
            BranchMergeFailureKind::UnsupportedMergeStrategy,
            format!(
                "request declared multiple aspect merge policies for aspect {}",
                aspect.id()
            ),
        ));
    }
    if let Some(policy_name) = request_bindings.into_iter().next() {
        let descriptor = registry.resolve_by_name(&policy_name).cloned().ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "request aspect merge policy `{}` is not registered in the frozen aspect merge policy registry",
                    policy_name.as_str()
                ),
            )
        })?;
        return Ok(ResolvedAspectPolicySelection {
            descriptor,
            basis: AspectMergePolicySelectionBasis::RequestNamed,
        });
    }

    if let Some(policy_name) =
        unanimous_node_aspect_policy_name(source_graph, candidate_nodes, aspect)?
    {
        let descriptor = registry.resolve_by_name(&policy_name).cloned().ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node aspect merge policy override `{}` is not registered in the frozen aspect merge policy registry",
                    policy_name.as_str()
                ),
            )
        })?;
        return Ok(ResolvedAspectPolicySelection {
            descriptor,
            basis: AspectMergePolicySelectionBasis::NodeOverride,
        });
    }

    if let Some(policy_name) =
        unanimous_schema_aspect_policy_name(source_graph, schema_registry, candidate_nodes, aspect)?
    {
        let descriptor = registry.resolve_by_name(&policy_name).cloned().ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "schema default aspect merge policy `{}` is not registered in the frozen aspect merge policy registry",
                    policy_name.as_str()
                ),
            )
        })?;
        return Ok(ResolvedAspectPolicySelection {
            descriptor,
            basis: AspectMergePolicySelectionBasis::SchemaDefault,
        });
    }

    let descriptor = registry
        .resolve_by_name(&crate::logic::transaction::AspectMergePolicyName::new(
            match default_policy {
                AspectMergePolicy::RequireConflict => "signal.aspect.require-conflict",
                AspectMergePolicy::PreferSource => "signal.aspect.prefer-source",
                AspectMergePolicy::PreferTarget => "signal.aspect.prefer-target",
            },
        ))
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "aspect merge policy {:?} has no registered descriptor in the frozen aspect merge policy registry",
                    default_policy
                ),
            )
        })?;
    Ok(ResolvedAspectPolicySelection {
        descriptor,
        basis: AspectMergePolicySelectionBasis::BuiltInDefault,
    })
}

fn resolve_conflict_isolation_descriptor(
    registry: &FrozenConflictIsolationRegistry,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    request: &BranchMergeRequest,
    default_granularity: ConflictIsolationGranularity,
) -> Result<ResolvedConflictIsolationSelection, SignalError> {
    if let Some(policy_name) = request.conflict_isolation_policy_name.as_ref() {
        let descriptor = registry
            .resolve_by_name(policy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "branch merge request references unknown conflict isolation policy `{}`",
                        policy_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedConflictIsolationSelection {
            descriptor,
            basis: ConflictIsolationSelectionBasis::RequestNamed,
        });
    }
    if let Some(policy_name) =
        unanimous_node_conflict_isolation_name(source_graph, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "node-owned conflict isolation policy `{}` is not registered",
                        policy_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedConflictIsolationSelection {
            descriptor,
            basis: ConflictIsolationSelectionBasis::NodeOverride,
        });
    }
    if let Some(policy_name) =
        unanimous_schema_conflict_isolation_name(schema_registry, source_graph, candidate_nodes)?
    {
        let descriptor = registry
            .resolve_by_name(&policy_name)
            .cloned()
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "schema-owned conflict isolation policy `{}` is not registered",
                        policy_name.as_str()
                    ),
                )
            })?;
        return Ok(ResolvedConflictIsolationSelection {
            descriptor,
            basis: ConflictIsolationSelectionBasis::SchemaDefault,
        });
    }
    let descriptor = registry
        .resolve_by_name(&match default_granularity {
            ConflictIsolationGranularity::PerNode => {
                ConflictIsolationPolicyName::new("signal.conflict-isolation.per-node")
            }
            ConflictIsolationGranularity::PerAspect => {
                ConflictIsolationPolicyName::new("signal.conflict-isolation.per-aspect")
            }
            ConflictIsolationGranularity::HostDeclaredRegion => {
                ConflictIsolationPolicyName::new("signal.conflict-isolation.per-node")
            }
        })
        .cloned()
        .ok_or_else(|| {
            SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "conflict isolation default {:?} has no registered descriptor",
                    default_granularity
                ),
            )
        })?;
    Ok(ResolvedConflictIsolationSelection {
        descriptor,
        basis: ConflictIsolationSelectionBasis::BuiltInDefault,
    })
}

fn unanimous_node_conflict_isolation_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<ConflictIsolationPolicyName>, SignalError> {
    let mut selected: Option<ConflictIsolationPolicyName> = None;
    for node in candidate_nodes {
        let Some(name) = source_graph.node_conflict_isolation_policy_name(*node)? else {
            continue;
        };
        match &selected {
            None => selected = Some(name.clone()),
            Some(existing) if existing == name => {}
            Some(_) => {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    "candidate nodes disagree on per-node conflict isolation policy",
                ))
            }
        }
    }
    Ok(selected)
}

fn unanimous_schema_conflict_isolation_name(
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<ConflictIsolationPolicyName>, SignalError> {
    let mut selected: Option<ConflictIsolationPolicyName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let descriptor = schema_registry
            .resolve_by_id(binding.schema_id())
            .ok_or_else(|| {
                SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "node {} references unknown schema id `{}` during conflict isolation resolution",
                        node,
                        binding.schema_id().0
                ),
            )
        })?;
        let Some(name) = descriptor.default_conflict_isolation_policy_name() else {
            continue;
        };
        match &selected {
            None => selected = Some(name.clone()),
            Some(existing) if existing == name => {}
            Some(_) => {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    "candidate nodes disagree on schema-owned conflict isolation policy",
                ))
            }
        }
    }
    Ok(selected)
}

fn unanimous_node_override_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::MergeStrategyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::MergeStrategyName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_merge_strategy_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting merge strategy overrides: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_node_aspect_policy_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
    aspect: Aspect,
) -> Result<Option<crate::logic::transaction::AspectMergePolicyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::AspectMergePolicyName> = None;
    for node in candidate_nodes {
        let candidate = source_graph
            .node_aspect_merge_policy_bindings(*node)?
            .iter()
            .find(|binding| binding.aspect == aspect)
            .map(|binding| binding.policy_name.clone());
        let Some(candidate) = candidate else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting aspect merge policy overrides for aspect {}: `{}` vs `{}`",
                        aspect.id(),
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_node_conflict_policy_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::ConflictPolicyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::ConflictPolicyName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_conflict_policy_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting conflict policy overrides: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_node_identity_matcher_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::IdentityMatcherName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::IdentityMatcherName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_identity_matcher_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting identity matcher overrides: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_node_source_only_policy_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::SourceOnlyPolicyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::SourceOnlyPolicyName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_source_only_policy_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting source-only policy overrides: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_node_deletion_policy_name(
    source_graph: &SignalGraph,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::DeletionPolicyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::DeletionPolicyName> = None;
    for node in candidate_nodes {
        let Some(candidate) = source_graph.node_deletion_policy_name(*node)?.cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate nodes declare conflicting deletion policy overrides: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_schema_default_name(
    source_graph: &SignalGraph,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::MergeStrategyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::MergeStrategyName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let Some(descriptor) = schema_registry.resolve_by_id(binding.schema_id()) else {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node {} references unknown schema id `{}` during merge strategy selection",
                    node,
                    binding.schema_id().0
                ),
            ));
        };
        let Some(candidate) = descriptor.default_merge_strategy_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default merge strategies: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_schema_conflict_policy_name(
    source_graph: &SignalGraph,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::ConflictPolicyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::ConflictPolicyName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let Some(descriptor) = schema_registry.resolve_by_id(binding.schema_id()) else {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node {} references unknown schema id `{}` during conflict policy selection",
                    node,
                    binding.schema_id().0
                ),
            ));
        };
        let Some(candidate) = descriptor.default_conflict_policy_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default conflict policies: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_schema_identity_matcher_name(
    source_graph: &SignalGraph,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::IdentityMatcherName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::IdentityMatcherName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let Some(descriptor) = schema_registry.resolve_by_id(binding.schema_id()) else {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node {} references unknown schema id `{}` during identity matcher selection",
                    node,
                    binding.schema_id().0
                ),
            ));
        };
        let Some(candidate) = descriptor.default_identity_matcher_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default identity matchers: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_schema_source_only_policy_name(
    source_graph: &SignalGraph,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::SourceOnlyPolicyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::SourceOnlyPolicyName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let Some(descriptor) = schema_registry.resolve_by_id(binding.schema_id()) else {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node {} references unknown schema id `{}` during source-only policy selection",
                    node,
                    binding.schema_id().0
                ),
            ));
        };
        let Some(candidate) = descriptor.default_source_only_policy_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default source-only policies: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_schema_deletion_policy_name(
    source_graph: &SignalGraph,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
) -> Result<Option<crate::logic::transaction::runtime::DeletionPolicyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::runtime::DeletionPolicyName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let Some(descriptor) = schema_registry.resolve_by_id(binding.schema_id()) else {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node {} references unknown schema id `{}` during deletion policy selection",
                    node,
                    binding.schema_id().0
                ),
            ));
        };
        let Some(candidate) = descriptor.default_deletion_policy_name().cloned() else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default deletion policies: `{}` vs `{}`",
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
}

fn unanimous_schema_aspect_policy_name(
    source_graph: &SignalGraph,
    schema_registry: &crate::schema::data::SignalSchemaRegistry,
    candidate_nodes: &[NodeId],
    aspect: Aspect,
) -> Result<Option<crate::logic::transaction::AspectMergePolicyName>, SignalError> {
    let mut selected: Option<crate::logic::transaction::AspectMergePolicyName> = None;
    for node in candidate_nodes {
        let Some(binding) = source_graph.node_schema_binding(*node)? else {
            continue;
        };
        let Some(descriptor) = schema_registry.resolve_by_id(binding.schema_id()) else {
            return Err(SignalError::branch_merge_failed(
                BranchMergeFailureKind::UnsupportedMergeStrategy,
                format!(
                    "node {} references unknown schema id `{}` during aspect merge policy selection",
                    node,
                    binding.schema_id().0
                ),
            ));
        };
        let candidate = descriptor
            .default_aspect_merge_policy_bindings()
            .iter()
            .find(|binding| binding.aspect == aspect)
            .map(|binding| binding.policy_name.clone());
        let Some(candidate) = candidate else {
            continue;
        };
        if let Some(existing) = selected.as_ref() {
            if existing != &candidate {
                return Err(SignalError::branch_merge_failed(
                    BranchMergeFailureKind::UnsupportedMergeStrategy,
                    format!(
                        "merge candidate schemas declare conflicting default aspect merge policies for aspect {}: `{}` vs `{}`",
                        aspect.id(),
                        existing.as_str(),
                        candidate.as_str()
                    ),
                ));
            }
        } else {
            selected = Some(candidate);
        }
    }
    Ok(selected)
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
