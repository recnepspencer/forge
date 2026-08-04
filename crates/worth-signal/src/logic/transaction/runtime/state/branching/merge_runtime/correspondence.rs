use std::collections::{BTreeMap, BTreeSet};

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::reuse::ReuseStrategy;
use crate::logic::transaction::runtime::{
    BranchMergeFailureKind, ConservativeOverlapExpansion, IdentityCorrespondenceStatus,
    IdentityMatchPolicy, IdentityMatcherName, LoweredFoundationalMergeRequest,
    LoweredIdentityCorrespondencePlan, MergeNodeMap, ProofMinimalOverlapBasis,
};

use super::super::super::merge::BranchMutationJournalSlice;
use super::super::super::runtime_state::SignalRuntime;
use super::artifact_projection::node_merge_projection;
use super::candidates::CandidateDiscovery;
use super::correspondence_evidence as evidence;

pub(super) struct CorrespondenceResolution {
    pub(super) identity_matches: BTreeMap<NodeId, NodeId>,
    pub(super) identity_correspondence: LoweredIdentityCorrespondencePlan,
    pub(super) target_overlap_journal: BranchMutationJournalSlice,
    pub(super) target_only_nodes: Vec<NodeId>,
    pub(super) proof_minimal_overlap: ProofMinimalOverlapBasis,
    pub(super) conservative_overlap: ConservativeOverlapExpansion,
    pub(super) node_map: MergeNodeMap,
}

pub(super) struct CorrespondencePhaseInput<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
    pub(super) request: &'a LoweredFoundationalMergeRequest,
    pub(super) candidates: &'a CandidateDiscovery<D, I, T>,
    pub(super) matcher_name: &'a IdentityMatcherName,
    pub(super) matcher_policy: IdentityMatchPolicy,
}

pub(super) fn lower_correspondence<D, I, E, Ctx, T>(
    input: CorrespondencePhaseInput<'_, D, I, E, Ctx, T>,
) -> Result<CorrespondenceResolution, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let target_state = input
        .candidates
        .branch_states
        .target_state_owned
        .as_ref()
        .or_else(|| {
            input
                .runtime
                .branches
                .branch_state(input.candidates.branch_states.target_branch_id)
        })
        .ok_or_else(|| SignalError::invalid_input("merge target branch state disappeared"))?;
    let identity_outcome = resolve_identity_matches(CorrespondenceInput {
        matcher_name: input.matcher_name,
        policy: input.matcher_policy,
        source_graph: input.candidates.branch_states.source_state.graph(),
        target_graph: target_state.graph(),
        source_nodes: &input.candidates.source_nodes,
        target_identity_journal: &input.candidates.target_identity_journal,
    })
    .map_err(|error| {
        let rewritten = super::super::super::merge::rewrite_identity_scoped_admission_error(
            input.request,
            error,
        );
        match &rewritten {
            SignalError::BranchMergeFailed {
                kind: BranchMergeFailureKind::ScopedMergeDenied,
                ..
            } => {
                input
                    .runtime
                    .telemetry
                    .transaction
                    .scoped_merge_denial_count += 1
            }
            SignalError::BranchMergeFailed {
                kind: BranchMergeFailureKind::ScopedMergeUnavailable,
                ..
            } => {
                input
                    .runtime
                    .telemetry
                    .transaction
                    .scoped_merge_unavailable_count += 1
            }
            _ => {}
        }
        rewritten
    })?;
    let identity_matches = identity_outcome.matches;
    let identity_correspondence = identity_outcome.correspondence;
    let matched_target_nodes = identity_matches.values().copied().collect::<BTreeSet<_>>();
    let mut proof_minimal_overlap_nodes = Vec::new();
    let mut conservative_overlap_nodes = input.candidates.conservative_overlap_nodes.clone();
    let mut node_map = input.candidates.node_map.clone();
    for source_node in &input.candidates.source_nodes {
        if let Some(target_node) = identity_matches.get(source_node).copied() {
            proof_minimal_overlap_nodes.push(*source_node);
            conservative_overlap_nodes.insert(*source_node);
            conservative_overlap_nodes.insert(target_node);
            node_map.insert(*source_node, target_node);
        }
    }
    let target_overlap_journal = BranchMutationJournalSlice {
        records: input
            .candidates
            .target_identity_journal
            .records
            .iter()
            .filter(|record| matched_target_nodes.contains(&record.node))
            .cloned()
            .collect(),
    };
    let target_only_nodes = input
        .candidates
        .target_identity_journal
        .records
        .iter()
        .filter(|record| !matched_target_nodes.contains(&record.node))
        .map(|record| record.node)
        .collect();
    Ok(CorrespondenceResolution {
        identity_matches,
        identity_correspondence,
        target_overlap_journal,
        target_only_nodes,
        proof_minimal_overlap: ProofMinimalOverlapBasis {
            shared_nodes: proof_minimal_overlap_nodes,
        },
        conservative_overlap: ConservativeOverlapExpansion {
            expanded_nodes: conservative_overlap_nodes.into_iter().collect(),
            support_nodes: input
                .candidates
                .conservative_support_nodes
                .iter()
                .copied()
                .collect(),
        },
        node_map,
    })
}

pub(super) struct CorrespondenceInput<'a> {
    pub(super) matcher_name: &'a IdentityMatcherName,
    pub(super) policy: IdentityMatchPolicy,
    pub(super) source_graph: &'a SignalGraph,
    pub(super) target_graph: &'a SignalGraph,
    pub(super) source_nodes: &'a [NodeId],
    pub(super) target_identity_journal: &'a BranchMutationJournalSlice,
}

pub(super) fn resolve_identity_matches(
    input: CorrespondenceInput<'_>,
) -> Result<evidence::IdentityResolutionOutcome, SignalError> {
    let mut matches = BTreeMap::new();
    let mut used_target_nodes = BTreeSet::new();
    let mut records = Vec::new();
    let mut source_lookup_count = 0u64;
    let mut rejected_admissibility_count = 0u64;

    for source_node in input.source_nodes {
        if input.target_graph.is_alive(*source_node) {
            matches.insert(*source_node, *source_node);
            used_target_nodes.insert(*source_node);
            let source_projection = node_merge_projection(input.source_graph, *source_node)?;
            let target_projection = node_merge_projection(input.target_graph, *source_node)?;
            records.push(evidence::exact_node_record(
                *source_node,
                source_projection.as_ref(),
                target_projection.as_ref(),
            ));
        }
    }

    if !matches!(
        input.policy,
        IdentityMatchPolicy::OutputIdentityWithinTargetJournal
    ) {
        for source_node in input.source_nodes {
            if matches.contains_key(source_node) {
                continue;
            }
            records.push(evidence::unmatched_record(
                *source_node,
                node_merge_projection(input.source_graph, *source_node)?,
                IdentityCorrespondenceStatus::UnmatchedNoCandidate,
            ));
        }
        return Ok(evidence::IdentityResolutionOutcome {
            matches,
            correspondence: evidence::correspondence_plan(
                input.target_identity_journal,
                source_lookup_count,
                0,
                rejected_admissibility_count,
                records,
            ),
        });
    }

    let mut target_index: BTreeMap<
        crate::data::output::OutputIdentity,
        Vec<(NodeId, Option<crate::data::output::OutputIdentity>)>,
    > = BTreeMap::new();
    for record in &input.target_identity_journal.records {
        let projection = node_merge_projection(input.target_graph, record.node)?;
        if let Some(identity) = evidence::output_identity(projection.as_ref()) {
            target_index
                .entry(identity.clone())
                .or_default()
                .push((record.node, Some(identity)));
        }
    }
    let ambiguous_match_count = 0u64;

    for source_node in input.source_nodes {
        if matches.contains_key(source_node) {
            continue;
        }
        source_lookup_count += 1;
        let Some(source_projection) = node_merge_projection(input.source_graph, *source_node)?
        else {
            records.push(evidence::unmatched_record(
                *source_node,
                None,
                IdentityCorrespondenceStatus::UnmatchedNoCandidate,
            ));
            continue;
        };
        let Some(source_output_identity) = source_projection.comparable.output_identity.clone()
        else {
            records.push(evidence::unmatched_record(
                *source_node,
                Some(source_projection),
                IdentityCorrespondenceStatus::UnmatchedNoCandidate,
            ));
            continue;
        };
        let source_contract = input
            .source_graph
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
            let target_contract = input
                .target_graph
                .node_eval_config(target_node)?
                .contract
                .clone();
            let source_binding = input.source_graph.node_schema_binding(*source_node)?;
            let target_binding = input.target_graph.node_schema_binding(target_node)?;
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
            let candidate_target_nodes: Vec<NodeId> =
                candidates.iter().map(|(node, _)| *node).collect();
            records.push(evidence::ambiguous_record(
                *source_node,
                source_output_identity.clone(),
                candidate_target_nodes.clone(),
            ));
            let correspondence = evidence::correspondence_plan(
                input.target_identity_journal,
                source_lookup_count,
                1,
                rejected_admissibility_count,
                records,
            );
            return Err(evidence::ambiguous_match_error(
                input.matcher_name,
                *source_node,
                source_output_identity,
                candidate_target_nodes,
                correspondence,
            ));
        }
        if let Some((target_node, target_output_identity)) = candidates.first().cloned() {
            matches.insert(*source_node, target_node);
            used_target_nodes.insert(target_node);
            records.push(evidence::matched_output_identity_record(
                *source_node,
                source_output_identity,
                target_node,
                target_output_identity,
            ));
        } else {
            let status = if admissibility_rejection.is_some() {
                IdentityCorrespondenceStatus::UnmatchedRejectedAdmissibility
            } else {
                IdentityCorrespondenceStatus::UnmatchedNoCandidate
            };
            records.push(evidence::rejected_output_identity_record(
                *source_node,
                source_output_identity,
                status,
                admissibility_rejection,
            ));
        }
    }

    Ok(evidence::IdentityResolutionOutcome {
        matches,
        correspondence: evidence::correspondence_plan(
            input.target_identity_journal,
            source_lookup_count,
            ambiguous_match_count,
            rejected_admissibility_count,
            records,
        ),
    })
}
