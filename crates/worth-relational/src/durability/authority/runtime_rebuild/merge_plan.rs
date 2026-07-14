use crate::history::data::OrderedParentList;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::transactions::data::{merge_commit_mutation_plan_token, MergeCommitMutationPlan};

pub(super) fn merge_commit_mutation_plan_from_envelope(
    envelope: &CanonicalCommitEnvelope,
) -> Option<MergeCommitMutationPlan> {
    let merge_execution_authority = envelope.merge_execution_authority.as_ref()?;
    if !merge_execution_authority.retains_consistent_proof_packet_authority() {
        return None;
    }
    let packet = merge_execution_authority.execution_summary.proof_packet();
    Some(MergeCommitMutationPlan {
        transaction_id: envelope.merged_plan.transaction_id,
        target_branch: packet.request().target_branch().clone(),
        source_branch: packet.request().source_branch().clone(),
        merge_parent_branches: envelope.merge_parent_branches.clone().into(),
        requested_merge_parent_count: envelope.merge_parent_branches.len(),
        parent_commits: OrderedParentList::from_authoritative(
            envelope.commit.ordered_parents().as_slice().to_vec(),
        ),
        merge_base_commits: envelope.merge_base_commits.clone().into(),
        merged_plan: envelope.merged_plan.clone(),
        structural_summary: merge_execution_authority.structural_summary.clone(),
        merge_execution_summary: merge_execution_authority.execution_summary.clone(),
        proof_token: merge_commit_mutation_plan_token(),
    })
}
