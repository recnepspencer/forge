use crate::data::handle::NodeId;
use crate::data::output::OutputIdentity;
use crate::logic::transaction::runtime::{
    BranchMergeFailureEvidence, BranchMergeFailureKind, BranchMergeIdentityFailureEvidence,
    IdentityCorrespondenceBasis, IdentityCorrespondenceRecord, IdentityCorrespondenceStatus,
    IdentityMatcherName, LoweredIdentityCorrespondencePlan,
};

use super::super::super::merge::BranchMutationJournalSlice;
use super::artifact_projection::NodeMergeProjection;

pub(super) struct IdentityResolutionOutcome {
    pub(super) matches: std::collections::BTreeMap<NodeId, NodeId>,
    pub(super) correspondence: LoweredIdentityCorrespondencePlan,
}

pub(super) fn output_identity(projection: Option<&NodeMergeProjection>) -> Option<OutputIdentity> {
    projection.and_then(|projection| projection.comparable.output_identity.clone())
}

pub(super) fn exact_node_record(
    source_node: NodeId,
    source_projection: Option<&NodeMergeProjection>,
    target_projection: Option<&NodeMergeProjection>,
) -> IdentityCorrespondenceRecord {
    IdentityCorrespondenceRecord {
        source_node,
        target_node: Some(source_node),
        basis: Some(IdentityCorrespondenceBasis::ExactNodeId),
        status: IdentityCorrespondenceStatus::Matched,
        source_output_identity: output_identity(source_projection),
        target_output_identity: output_identity(target_projection),
        candidate_count: 1,
        candidate_target_nodes: vec![source_node],
        admissibility_rejection: None,
    }
}

pub(super) fn unmatched_record(
    source_node: NodeId,
    projection: Option<NodeMergeProjection>,
    status: IdentityCorrespondenceStatus,
) -> IdentityCorrespondenceRecord {
    IdentityCorrespondenceRecord {
        source_node,
        target_node: None,
        basis: None,
        status,
        source_output_identity: output_identity(projection.as_ref()),
        target_output_identity: None,
        candidate_count: 0,
        candidate_target_nodes: Vec::new(),
        admissibility_rejection: None,
    }
}

pub(super) fn ambiguous_record(
    source_node: NodeId,
    source_output_identity: OutputIdentity,
    candidate_target_nodes: Vec<NodeId>,
) -> IdentityCorrespondenceRecord {
    IdentityCorrespondenceRecord {
        source_node,
        target_node: None,
        basis: Some(IdentityCorrespondenceBasis::OutputIdentityTargetJournal),
        status: IdentityCorrespondenceStatus::AmbiguousCandidates,
        source_output_identity: Some(source_output_identity),
        target_output_identity: None,
        candidate_count: candidate_target_nodes.len() as u32,
        candidate_target_nodes,
        admissibility_rejection: None,
    }
}

pub(super) fn matched_output_identity_record(
    source_node: NodeId,
    source_output_identity: OutputIdentity,
    target_node: NodeId,
    target_output_identity: Option<OutputIdentity>,
) -> IdentityCorrespondenceRecord {
    IdentityCorrespondenceRecord {
        source_node,
        target_node: Some(target_node),
        basis: Some(IdentityCorrespondenceBasis::OutputIdentityTargetJournal),
        status: IdentityCorrespondenceStatus::Matched,
        source_output_identity: Some(source_output_identity),
        target_output_identity,
        candidate_count: 1,
        candidate_target_nodes: vec![target_node],
        admissibility_rejection: None,
    }
}

pub(super) fn rejected_output_identity_record(
    source_node: NodeId,
    source_output_identity: OutputIdentity,
    status: IdentityCorrespondenceStatus,
    admissibility_rejection: Option<String>,
) -> IdentityCorrespondenceRecord {
    IdentityCorrespondenceRecord {
        source_node,
        target_node: None,
        basis: None,
        status,
        source_output_identity: Some(source_output_identity),
        target_output_identity: None,
        candidate_count: 0,
        candidate_target_nodes: Vec::new(),
        admissibility_rejection,
    }
}

pub(super) fn ambiguous_match_error(
    matcher_name: &IdentityMatcherName,
    source_node: NodeId,
    source_output_identity: OutputIdentity,
    candidate_target_nodes: Vec<NodeId>,
    correspondence: LoweredIdentityCorrespondencePlan,
) -> crate::data::error::SignalError {
    crate::data::error::SignalError::branch_merge_failed_with_evidence(
        BranchMergeFailureKind::UnsupportedMergeStrategy,
        format!(
            "identity matcher found ambiguous target journal correspondence for source node {} and output identity",
            source_node
        ),
        BranchMergeFailureEvidence::Identity(BranchMergeIdentityFailureEvidence {
            identity_matcher_name: matcher_name.clone(),
            source_node,
            source_output_identity: Some(source_output_identity),
            candidate_target_nodes,
            correspondence,
        }),
    )
}

pub(super) fn correspondence_plan(
    target_identity_journal: &BranchMutationJournalSlice,
    source_lookup_count: u64,
    ambiguous_match_count: u64,
    rejected_admissibility_count: u64,
    records: Vec<IdentityCorrespondenceRecord>,
) -> LoweredIdentityCorrespondencePlan {
    LoweredIdentityCorrespondencePlan {
        target_candidate_count: target_identity_journal.records.len() as u64,
        source_lookup_count,
        ambiguous_match_count,
        rejected_admissibility_count,
        records,
    }
}
