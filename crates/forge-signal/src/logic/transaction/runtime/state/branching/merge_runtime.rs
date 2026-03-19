use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use std::collections::BTreeSet;
use crate::state::SignalBranchHandle;

use super::branches::LatestMergeReference;
use super::super::merge::{
    adopt_source_node_into_target, merge_comparable, ArtifactMergeAction, BranchMergeBase,
    BranchMergeConflictEvidence, BranchMergeConflictRecord, BranchMergeCounters,
    BranchMergeConflictKind, BranchMergeConflictSummary, BranchMergeDivergence,
    BranchMergeExecutionSummary, BranchMergeKind, BranchMergePlan,
    BranchMergeReconciliationPolicy, BranchMergeRequest, BranchMergeResult,
    BranchMergeResolutionRequirement, BranchMergeStrategy, BranchMergeFailureKind, ConflictMergePolicy,
    CausalityCarryPolicy, MergeCandidateScope, MergeDecisionBasis,
    ExistingTargetMergePolicy, MergeNodeMap, MergeTouchedNodeSet, MergedArtifactRecord,
    NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision,
    NodeReconciliationShape, SourceOnlyMergePolicy, StructuralMergeCandidateRecord,
    RetainedArtifactCarryPolicy, RuntimeArtifactCarryPolicy, SourceNodeAdoptionCarryPolicy,
    SourceNodeAdoptionPlanCore, TargetNodeIdentityIntent, TopologyRepairSummary,
    AdoptedNodeContract, AdoptionDependencySnapshotRef, AdoptionDependencyTopology,
};
use super::super::runtime_state::SignalRuntime;

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
        if source.id == target.id {
            let error = SignalError::branch_merge_failed(
                BranchMergeFailureKind::SelfMergeRejected,
                "branch merge cannot target itself",
            );
            crate::diagnostics::recorder::record_branch_merge_failure(
                &mut self.graph,
                &error,
                Some(source.clone()),
                Some(target.clone()),
            );
            return Err(error);
        }
        let request = BranchMergeRequest {
            source_branch: source.clone(),
            target_branch: target.clone(),
        };
        let plan = match self.build_branch_merge_plan(&request) {
            Ok(plan) => plan,
            Err(error) => {
                crate::diagnostics::recorder::record_branch_merge_failure(
                    &mut self.graph,
                    &error,
                    Some(source.clone()),
                    Some(target.clone()),
                );
                return Err(error);
            }
        };
        let summary = match self.execute_branch_merge_plan(&request, &plan) {
            Ok(summary) => summary,
            Err(error) => {
                crate::diagnostics::recorder::record_branch_merge_failure(
                    &mut self.graph,
                    &error,
                    Some(source.clone()),
                    Some(target.clone()),
                );
                return Err(error);
            }
        };
        crate::diagnostics::recorder::record_branch_merge_summary(
            &mut self.graph,
            &summary,
            source.name,
            target.name,
        );
        Ok(BranchMergeResult {
            source_branch: summary.source_branch_id,
            target_branch: summary.target_branch_id,
            merge_kind: summary.merge_kind,
            divergence: summary.divergence,
            merge_strategy: summary.merge_strategy,
            reconciliation_policy: summary.reconciliation_policy,
            candidate_scope: summary.candidate_scope.clone(),
            merged_snapshot_id: summary.target_snapshot_id_after,
            target_snapshot_id_before: summary.target_snapshot_id_before,
            target_snapshot_id_after: summary.target_snapshot_id_after,
            source_snapshot_id: summary.source_snapshot_id,
            records: summary.records.clone(),
            counters: summary.counters,
        })
    }

    fn build_branch_merge_plan(
        &mut self,
        request: &BranchMergeRequest,
    ) -> Result<BranchMergePlan, SignalError> {
        let source_state_owned = if request.source_branch.id == self.graph.current_branch().id {
            self.capture_branch_state()
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
            Some(self.capture_branch_state())
        } else {
            None
        };
        let target_state = target_state_owned.as_ref().or_else(|| {
            self.branches.branch_state(request.target_branch.id)
        });
        let target_state = target_state.ok_or_else(|| {
            SignalError::unknown_branch(
                Some(request.target_branch.id),
                request.target_branch.name.clone(),
            )
        })?;
        let source_state = &source_state_owned;
        let target_graph = &target_state.authority.graph;
        let target_snapshot_id_before = target_graph.branch_head_snapshot_id(request.target_branch.id);
        let source_snapshot_id = source_state
            .authority
            .graph
            .branch_head_snapshot_id(request.source_branch.id);
        let merge_base_snapshot = source_state.ancestry.forked_from_snapshot_id;
        let mut node_map = MergeNodeMap::default();
        let source_journal = source_state.mutation_ledger.structural_merge_journal();
        let mut source_nodes = source_journal.candidate_nodes();
        let candidate_scope = if source_nodes.is_empty()
            && !source_state.mutation_ledger.boundary_established
        {
            source_nodes = source_state.authority.graph.live_node_ids();
            source_nodes.sort_by_key(|node| (node.index(), node.generation()));
            MergeCandidateScope::WholeLiveAuthoritySurface
        } else {
            MergeCandidateScope::CandidateNodeSet(source_nodes.clone())
        };
        let target_overlap_journal = {
            let target_journal = target_state.mutation_ledger.structural_merge_journal();
            crate::logic::transaction::BranchMutationJournalSlice {
                records: target_journal
                    .records
                    .into_iter()
                    .filter(|record| source_journal.contains_node(record.node))
                    .collect(),
            }
        };
        let target_has_overlapping_merge_delta = !target_overlap_journal.records.is_empty();
        let divergence = if merge_base_snapshot == target_snapshot_id_before
            && !target_has_overlapping_merge_delta
        {
            BranchMergeDivergence::None
        } else {
            BranchMergeDivergence::TargetAdvanced
        };
        let merge_kind = if matches!(divergence, BranchMergeDivergence::None) {
            BranchMergeKind::FastForward
        } else {
            BranchMergeKind::Applied
        };
        let merge_strategy = match merge_kind {
            BranchMergeKind::FastForward => BranchMergeStrategy::AdoptSourceHead,
            BranchMergeKind::Applied => BranchMergeStrategy::AdoptSourceSubset,
            BranchMergeKind::ConflictResolved => BranchMergeStrategy::RebaseSourceOntoTarget,
        };
        let reconciliation_policy = BranchMergeReconciliationPolicy {
            existing_target: ExistingTargetMergePolicy::PreserveEquivalentOtherwiseAdoptSource,
            source_only: SourceOnlyMergePolicy::IntroduceAdoptableSkipNonAdoptable,
            conflict: ConflictMergePolicy::RejectSharedStateConflict,
        };
        let mut divergence = divergence;
        let mut conflict_records = Vec::new();
        if matches!(divergence, BranchMergeDivergence::TargetAdvanced) {
            for source_node in &source_nodes {
                if !target_graph.is_alive(*source_node) {
                    continue;
                }
                let source_entry = source_state.authority.graph.get_entry(*source_node)?;
                let target_entry = target_graph.get_entry(*source_node)?;
                let source_cmp = merge_comparable(
                    source_entry.get_runtime_artifact_state(),
                    source_entry.retained_diagnostic_artifact(),
                );
                let target_cmp = merge_comparable(
                    target_entry.get_runtime_artifact_state(),
                    target_entry.retained_diagnostic_artifact(),
                );
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
                    records: conflict_records,
                },
            ));
        }

        let mut node_plan = Vec::new();
        let mut adoption_core = Vec::new();
        let mut adoption_policy = Vec::new();
        for source_node in source_nodes {
            let source_entry = source_state.authority.graph.get_entry(source_node)?;
            let source_runtime = source_entry.get_runtime_artifact_state();
            let source_authority = source_runtime.map(|runtime| runtime.merge_authority.clone());
            if target_graph.is_alive(source_node) {
                node_map.insert(source_node, source_node);
                let target_entry = target_graph.get_entry(source_node)?;
                let source_cmp = merge_comparable(source_runtime, source_entry.retained_diagnostic_artifact());
                let target_cmp = merge_comparable(
                    target_entry.get_runtime_artifact_state(),
                    target_entry.retained_diagnostic_artifact(),
                );
                let decision = if source_cmp.is_some() && source_cmp == target_cmp {
                    NodeReconciliationDecision::MarkEquivalentUnchanged
                } else {
                    NodeReconciliationDecision::AdoptSourceAuthority
                };
                node_plan.push(NodeMergePlan {
                    source_node,
                    shape: NodeReconciliationShape::ExistingTargetNode {
                        target_node: source_node,
                    },
                    source_state: NodeMergeInputState {
                        current_artifact_id: source_runtime.and_then(|runtime| runtime.lineage_artifact_id),
                        comparable: source_cmp,
                        authority: source_authority.clone(),
                        exists_in_branch: true,
                    },
                    target_state: NodeMergeInputState {
                        current_artifact_id: target_entry
                            .get_runtime_artifact_state()
                            .and_then(|runtime| runtime.lineage_artifact_id),
                        comparable: target_cmp,
                        authority: target_entry
                            .get_runtime_artifact_state()
                            .map(|runtime| runtime.merge_authority.clone()),
                        exists_in_branch: true,
                    },
                    decision,
                });
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
                node_plan.push(NodeMergePlan {
                    source_node,
                    shape: NodeReconciliationShape::SourceOnlyIntroduction,
                    source_state: NodeMergeInputState {
                        current_artifact_id: source_runtime.and_then(|runtime| runtime.lineage_artifact_id),
                        comparable: merge_comparable(
                            source_runtime,
                            source_entry.retained_diagnostic_artifact(),
                        ),
                        authority: Some(authority.clone()),
                        exists_in_branch: true,
                    },
                    target_state: NodeMergeInputState {
                        current_artifact_id: None,
                        comparable: None,
                        authority: None,
                        exists_in_branch: false,
                    },
                    decision,
                });
                if matches!(decision, NodeReconciliationDecision::AdoptSourceAuthority) {
                    adoption_core.push(SourceNodeAdoptionPlanCore {
                        source_node,
                        target_identity: TargetNodeIdentityIntent::AllocateTargetNode,
                        authority,
                        entry_contract: AdoptedNodeContract {
                            eval_config: source_entry.get_eval_config().clone(),
                        },
                        dependency_topology: AdoptionDependencyTopology {
                            dependencies: source_state.authority.graph.dependencies_of(source_node)?.to_vec(),
                        },
                        dependency_snapshot_ref: AdoptionDependencySnapshotRef {
                            snapshot: source_state.authority.graph.get_dep_snapshot(source_node)?.clone(),
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

        Ok(BranchMergePlan {
            source_branch_id: request.source_branch.id,
            target_branch_id: request.target_branch.id,
            merge_kind,
            divergence,
            merge_strategy,
            reconciliation_policy,
            candidate_scope,
            source_journal,
            target_overlap_journal,
            source_snapshot_id,
            target_snapshot_id_before,
            merge_base: Some(BranchMergeBase {
                source_branch_id: request.source_branch.id,
                target_branch_id: request.target_branch.id,
                forked_from_snapshot_id: merge_base_snapshot,
                source_snapshot_id,
                target_snapshot_id_before,
            }),
            node_map,
            node_plan,
            adoption_core,
            adoption_policy,
        })
    }

    fn execute_branch_merge_plan(
        &mut self,
        request: &BranchMergeRequest,
        plan: &BranchMergePlan,
    ) -> Result<BranchMergeExecutionSummary, SignalError> {
        let source_state = if request.source_branch.id == self.graph.current_branch().id {
            self.capture_branch_state()
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
            self.capture_branch_state()
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

        let mut node_map = plan.node_map.clone();
        let mut dependency_remaps = Vec::new();
        let mut records = Vec::new();
        let mut touched = BTreeSet::new();
        let mut repaired_sources = BTreeSet::new();
        let target_snapshot_before = plan.target_snapshot_id_before;

        for (core, policy) in plan.adoption_core.iter().zip(plan.adoption_policy.iter()) {
            let (materialized, remaps) = adopt_source_node_into_target(
                &mut target_state.authority.graph,
                &source_state.authority.graph,
                core,
                policy,
                &node_map,
            )?;
            node_map.insert(core.source_node, materialized.target_node);
            touched.insert(materialized.target_node);
            repaired_sources.extend(remaps.iter().map(|record| record.target_dependency));
            dependency_remaps.extend(remaps);
        }

        for node_plan in &plan.node_plan {
            match node_plan.shape {
                NodeReconciliationShape::ExistingTargetNode { target_node } => {
                    let source_entry = source_state.authority.graph.get_entry(node_plan.source_node)?.clone();
                    let mut replacement = source_entry.clone();
                    replacement.set_dependencies_id(
                        target_state.authority.graph.get_entry(target_node)?.get_dependencies_id(),
                    );
                    replacement.set_subscribers_id(
                        target_state.authority.graph.get_entry(target_node)?.get_subscribers_id(),
                    );
                    replacement.set_dep_snapshot_id(
                        target_state.authority.graph.get_entry(target_node)?.get_dep_snapshot_id(),
                    );
                    if matches!(node_plan.decision, NodeReconciliationDecision::AdoptSourceAuthority)
                    {
                        repaired_sources.extend(
                            target_state
                                .authority
                                .graph
                                .dependencies_of(target_node)?
                                .iter()
                                .map(|edge| edge.source()),
                        );
                        let mapped_edges = source_state
                            .authority
                            .graph
                            .dependencies_of(node_plan.source_node)?
                            .iter()
                            .map(|edge| {
                                let mapped = node_map.resolve(edge.source()).ok_or_else(|| {
                                    SignalError::invalid_input(format!(
                                        "merge plan has unresolved dependency remap {} for node {}",
                                        edge.source(),
                                        node_plan.source_node
                                    ))
                                })?;
                                Ok(match edge.scope_ref().cloned() {
                                    Some(scope) => {
                                        DependencyEdge::with_partition_scope(mapped, edge.aspect(), scope)
                                    }
                                    None => DependencyEdge::new(mapped, edge.aspect()),
                                })
                            })
                            .collect::<Result<Vec<_>, SignalError>>()?;
                        repaired_sources.extend(mapped_edges.iter().map(|edge| edge.source()));
                        target_state.authority.graph.replace_entry(target_node, replacement)?;
                        target_state.authority.graph.set_dependencies(target_node, mapped_edges)?;
                        touched.insert(target_node);
                    }
                }
                NodeReconciliationShape::SourceOnlyIntroduction => {
                    if let Some(mapped) = node_map.resolve(node_plan.source_node) {
                        touched.insert(mapped);
                    }
                }
            }
        }

        let merged_snapshot = {
            let policy = target_state.authority.graph.runtime_policy();
            let meta = target_state
                .authority
                .graph
                .diagnostics_state_mut()
                .allocate_snapshot_meta(policy);
            target_state
                .authority
                .graph
                .diagnostics_state_mut()
                .set_branch_head_snapshot(request.target_branch.id, meta.snapshot_id);
            meta.snapshot_id
        };

        let target_snapshot_after = Some(merged_snapshot);
        target_state.ancestry.latest_merge_reference = Some(LatestMergeReference {
            source_branch_id: request.source_branch.id,
            source_snapshot_id: plan.source_snapshot_id,
            target_snapshot_id_before: target_snapshot_before,
            target_snapshot_id_after: target_snapshot_after,
            merge_kind: plan.merge_kind,
            merge_strategy: plan.merge_strategy,
        });
        target_state.mutation_ledger.clear_all(target_snapshot_after);
        target_state.authority.graph.clear_branch_mutation_nodes();

        for node_plan in &plan.node_plan {
            let target_node = match node_plan.shape {
                NodeReconciliationShape::ExistingTargetNode { target_node } => Some(target_node),
                NodeReconciliationShape::SourceOnlyIntroduction => node_map.resolve(node_plan.source_node),
            };
            let target_entry = target_node.and_then(|node| target_state.authority.graph.get_entry(node).ok());
            let action = match node_plan.shape {
                NodeReconciliationShape::SourceOnlyIntroduction => {
                    if matches!(node_plan.decision, NodeReconciliationDecision::SkipNonAdoptableSource)
                    {
                        ArtifactMergeAction::SkippedNonAdoptable
                    } else {
                        ArtifactMergeAction::IntroducedIntoTarget
                    }
                }
                NodeReconciliationShape::ExistingTargetNode { .. } => match node_plan.decision {
                    NodeReconciliationDecision::MarkEquivalentUnchanged => {
                        ArtifactMergeAction::EquivalentUnchanged
                    }
                    NodeReconciliationDecision::PreserveTarget => ArtifactMergeAction::PreservedTarget,
                    NodeReconciliationDecision::AdoptSourceAuthority => ArtifactMergeAction::Adopted,
                    NodeReconciliationDecision::ReplaceTargetAuthority => ArtifactMergeAction::Replaced,
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
                source_node: node_plan.source_node,
                target_node,
                source_artifact_id: node_plan.source_state.current_artifact_id,
                target_artifact_id_before: node_plan.target_state.current_artifact_id,
                target_artifact_id_after: target_entry
                    .and_then(|entry| entry.get_runtime_artifact_state())
                    .and_then(|runtime| runtime.lineage_artifact_id),
                action,
                basis,
                source_comparable: node_plan.source_state.comparable.clone(),
                target_comparable: target_entry.and_then(|entry| {
                    merge_comparable(
                        entry.get_runtime_artifact_state(),
                        entry.retained_diagnostic_artifact(),
                    )
                }),
            });
        }

        records.sort_by_key(|record| (record.source_node.index(), record.source_node.generation()));
        let touched_set = MergeTouchedNodeSet {
            nodes: touched.into_iter().collect(),
        };
        let counters = BranchMergeCounters {
            candidate_node_count: plan.node_plan.len() as u64,
            examined_node_count: plan.node_plan.len() as u64,
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
                .node_plan
                .iter()
                .filter(|node| matches!(node.shape, NodeReconciliationShape::SourceOnlyIntroduction))
                .count() as u64,
            target_only_count: 0,
            dependency_remap_count: dependency_remaps.len() as u64,
            subscriber_repair_breadth: repaired_sources.len() as u64,
            merge_lineage_record_count: (records.len() + 1) as u64,
            replay_event_count: 1,
            branch_wide_scan_performed: matches!(
                plan.candidate_scope,
                MergeCandidateScope::WholeLiveAuthoritySurface
            ),
        };
        let summary = BranchMergeExecutionSummary {
            source_branch_id: plan.source_branch_id,
            target_branch_id: plan.target_branch_id,
            merge_kind: plan.merge_kind,
            divergence: plan.divergence,
            merge_strategy: plan.merge_strategy,
            reconciliation_policy: plan.reconciliation_policy,
            candidate_scope: plan.candidate_scope.clone(),
            merge_base: plan.merge_base.clone(),
            source_snapshot_id: plan.source_snapshot_id,
            target_snapshot_id_before: target_snapshot_before,
            target_snapshot_id_after: target_snapshot_after,
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

        if request.target_branch.id == self.graph.current_branch().id {
            self.load_branch_state(target_state.clone());
        }
        self.branches
            .insert_branch(request.target_branch.id, target_state.clone());
        if request.source_branch.id == self.graph.current_branch().id {
            let mut updated_source_state = self.capture_branch_state();
            updated_source_state
                .mutation_ledger
                .clear_merged_nodes(merged_source_nodes.iter().copied(), plan.source_snapshot_id);
            updated_source_state.authority.graph.clear_branch_mutation_nodes();
            self.load_branch_state(updated_source_state);
        } else if let Some(source_state) = self
            .branches
            .branch_state_mut_with_allocator_sync(request.source_branch.id)
        {
            source_state.mutation_ledger.clear_merged_nodes(
                merged_source_nodes.iter().copied(),
                plan.source_snapshot_id,
            );
            source_state.authority.graph.clear_branch_mutation_nodes();
        }
        self.branches.insert_snapshot(
            merged_snapshot,
            target_state,
        );
        let branch_catalog = if request.target_branch.id == self.graph.current_branch().id {
            self.graph.diagnostics_state().branch_catalog().clone()
        } else {
            self.branches
                .branch_state(request.target_branch.id)
                .map(|state| state.authority.graph.diagnostics_state().branch_catalog().clone())
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

fn summarize_conflict_records(
    records: &[BranchMergeConflictRecord],
) -> BranchMergeConflictSummary {
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
