use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::trace::{RuntimeArtifactHot, RuntimeArtifactWarm};
use crate::state::{SignalBranchHandle, SnapshotArtifactRetentionPolicy};
use std::collections::{BTreeMap, BTreeSet};

use super::super::merge::{
    adopt_source_node_into_target, remap_dependency_snapshot, AdoptedNodeContract,
    AdoptionDependencySnapshotRef, AdoptionDependencyTopology, ArtifactMergeAction,
    BranchConflictResolutionPlan, BranchMergeBase, BranchMergeConflictEvidence,
    BranchMergeConflictKind, BranchMergeConflictRecord, BranchMergeConflictSummary,
    BranchMergeCounters, BranchMergeDivergence, BranchMergeExecutionSummary,
    BranchMergeFailureKind, BranchMergeKind, BranchMergePlan, BranchMergeReconciliationPolicy,
    BranchMergeRequest, BranchMergeResolutionRequirement, BranchMergeResult, BranchMergeStrategy,
    CausalityCarryPolicy, ConflictMergePolicy, ConflictResolutionRecord,
    ConflictResolutionStrategy, ConservativeOverlapExpansion, ExistingTargetMergePolicy,
    LoweredMergePlan, MergeBoundaryWitness, MergeBoundaryWitnessKind, MergeDecisionBasis,
    MergeNodeMap, MergeTouchedNodeSet, MergedArtifactRecord, NodeMergeInputState, NodeMergePlan,
    NodeReconciliationDecision, NodeReconciliationShape, PlannedMergeCandidateSet,
    ProofMinimalOverlapBasis, RetainedArtifactCarryPolicy, RuntimeArtifactCarryPolicy,
    SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy,
    StructuralMergeCandidateRecord, StructuralMergeJournalSlice, TargetNodeIdentityIntent,
    TopologyRepairSummary,
};
use super::super::runtime_state::SignalRuntime;
use super::branches::LatestMergeReference;

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
            merge_kind: summary.merge_kind,
            divergence: summary.divergence,
            merge_strategy: summary.merge_strategy,
            reconciliation_policy: summary.reconciliation_policy,
            boundary_witness: summary.boundary_witness.clone(),
            proof_minimal_overlap: summary.proof_minimal_overlap.clone(),
            conservative_overlap: summary.conservative_overlap.clone(),
            planned_candidates: summary.planned_candidates.clone(),
            merged_snapshot_id: summary.target_snapshot_id_after,
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
        let merge_base_snapshot = source_state.ancestry().forked_from_snapshot_id();
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
            if target_graph.is_alive(*source_node) {
                proof_minimal_overlap_nodes.push(*source_node);
                conservative_overlap_nodes.insert(*source_node);
                node_map.insert(*source_node, *source_node);
            }
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
        let proof_minimal_overlap = {
            ProofMinimalOverlapBasis {
                shared_nodes: proof_minimal_overlap_nodes,
            }
        };
        let target_overlap_journal = crate::logic::transaction::BranchMutationJournalSlice {
            records: target_state
                .mutation_ledger()
                .structural_merge_journal()
                .records
                .into_iter()
                .filter(|record| proof_minimal_overlap.shared_nodes.contains(&record.node))
                .collect(),
        };
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
        let mut merge_strategy = match merge_kind {
            BranchMergeKind::FastForward => BranchMergeStrategy::AdoptSourceHead,
            BranchMergeKind::Applied => BranchMergeStrategy::AdoptSourceSubset,
            BranchMergeKind::ConflictResolved => BranchMergeStrategy::RebaseSourceOntoTarget,
        };
        let reconciliation_policy = BranchMergeReconciliationPolicy {
            existing_target: ExistingTargetMergePolicy::PreserveEquivalentOtherwiseAdoptSource,
            source_only: SourceOnlyMergePolicy::IntroduceAdoptableSkipNonAdoptable,
            conflict: ConflictMergePolicy::ResolveSourceStateWhenStructureMatches,
        };
        let mut divergence = divergence;
        let mut conflict_records = Vec::new();
        let mut resolution_plan = None;
        if matches!(divergence, BranchMergeDivergence::TargetAdvanced) {
            for source_node in &source_nodes {
                if !target_graph.is_alive(*source_node) {
                    continue;
                }
                let source_cmp = node_merge_projection(source_state.graph(), *source_node)?
                    .map(|projection| projection.comparable);
                let target_cmp = node_merge_projection(target_graph, *source_node)?
                    .map(|projection| projection.comparable);
                let source_structural_record = source_journal
                    .records
                    .iter()
                    .find(|record| record.node == *source_node)
                    .cloned();
                let target_structural_record = target_overlap_journal
                    .records
                    .iter()
                    .find(|record| record.node == *source_node)
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
                        target_node: *source_node,
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
                    BranchMergeConflictEvidence {
                        divergence,
                        reconciliation_policy,
                        summary: conflict_summary,
                        resolution_plan: planned_resolution,
                        records: conflict_records,
                    },
                ));
            }
        }

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
            if target_graph.is_alive(source_node) {
                node_map.insert(source_node, source_node);
                let target_projection = node_merge_projection(target_graph, source_node)?;
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
                    NodeReconciliationShape::ExistingTargetNode {
                        target_node: source_node,
                    },
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

        Ok(LoweredMergePlan::new(
            request.source_branch.id,
            request.target_branch.id,
            merge_kind,
            divergence,
            merge_strategy,
            reconciliation_policy,
            boundary_witness,
            source_journal,
            target_overlap_journal,
            proof_minimal_overlap,
            conservative_overlap,
            planned_candidates,
            source_snapshot_id,
            target_snapshot_id_before,
            Some(BranchMergeBase {
                source_branch_id: request.source_branch.id,
                target_branch_id: request.target_branch.id,
                forked_from_snapshot_id: merge_base_snapshot,
                source_snapshot_id,
                target_snapshot_id_before,
            }),
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

        for (core, policy) in plan
            .adoption_core()
            .iter()
            .zip(plan.adoption_policy().iter())
        {
            let (materialized, remaps) = adopt_source_node_into_target(
                target_state.graph_mut(),
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
                            .graph_mut()
                            .replace_entry_from_checkpoint_image(target_node, replacement)?;
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
            target_only_count: 0,
            dependency_remap_count: dependency_remaps.len() as u64,
            subscriber_repair_breadth: repaired_sources.len() as u64,
            merge_lineage_record_count: (records.len() + 1) as u64,
            replay_event_count: 1,
        };
        let summary = BranchMergeExecutionSummary {
            source_branch_id: plan.source_branch_id(),
            target_branch_id: plan.target_branch_id(),
            merge_kind: plan.merge_kind(),
            divergence: plan.divergence(),
            merge_strategy: plan.merge_strategy(),
            reconciliation_policy: plan.reconciliation_policy().clone(),
            boundary_witness: plan.boundary_witness().clone(),
            proof_minimal_overlap: plan.proof_minimal_overlap().clone(),
            conservative_overlap: plan.conservative_overlap().clone(),
            planned_candidates: plan.planned_candidates().clone(),
            merge_base: plan.merge_base().cloned(),
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
