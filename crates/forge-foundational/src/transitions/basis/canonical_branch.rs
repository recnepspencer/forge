use crate::canonicalization::CanonicalBasisEntry;
use crate::transitions::{
    FoundationalBranchCandidateArtifact, FoundationalBranchLocalStateKind,
    FoundationalStagedBranchArtifact,
};

use super::canonical_shared::{bool_entry, text_entry, u64_entry};

pub(super) fn candidate_entries<T>(
    candidate: &FoundationalBranchCandidateArtifact<T>,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = base_branch_entries(
        candidate.branch_local_state_kind(),
        candidate.branch_id().as_str(),
        candidate.candidate_id().handle().get(),
        candidate.fork_basis().forked_from_branch().as_str(),
        candidate.fork_basis().fork_epoch().get(),
        candidate.observation_basis().basis_id().get(),
        candidate.observation_basis().observed_epoch().get(),
    );
    append_optional_branch_entries(
        candidate
            .fork_observation_basis()
            .map(|basis| (basis.basis_id().get(), basis.fork_epoch().get())),
        candidate.comparison_basis().map(|basis| {
            (
                basis.basis_id().get(),
                basis.compared_against_branch().as_str(),
            )
        }),
        &mut entries,
    );
    entries
}

pub(super) fn staged_entries<T>(
    staged: &FoundationalStagedBranchArtifact<T>,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = base_branch_entries(
        staged.branch_local_state_kind(),
        staged.branch_id().as_str(),
        staged.candidate_id().handle().get(),
        staged.fork_basis().forked_from_branch().as_str(),
        staged.fork_basis().fork_epoch().get(),
        staged.observation_basis().basis_id().get(),
        staged.observation_basis().observed_epoch().get(),
    );
    append_optional_branch_entries(
        staged
            .fork_observation_basis()
            .map(|basis| (basis.basis_id().get(), basis.fork_epoch().get())),
        staged.comparison_basis().map(|basis| {
            (
                basis.basis_id().get(),
                basis.compared_against_branch().as_str(),
            )
        }),
        &mut entries,
    );
    entries
}

fn base_branch_entries(
    kind: FoundationalBranchLocalStateKind,
    branch_id: &str,
    candidate_id: u64,
    forked_from_branch: &str,
    fork_epoch: u64,
    observation_basis: u64,
    observed_epoch: u64,
) -> Vec<CanonicalBasisEntry> {
    vec![
        text_entry("branch.shape", branch_kind_token(kind)),
        text_entry("branch.branch_id", branch_id),
        u64_entry("branch.candidate_id", candidate_id),
        text_entry("branch.fork_branch", forked_from_branch),
        u64_entry("branch.fork_epoch", fork_epoch),
        u64_entry("branch.observation_basis", observation_basis),
        u64_entry("branch.observed_epoch", observed_epoch),
    ]
}

fn append_optional_branch_entries(
    fork_observation: Option<(u64, u64)>,
    comparison: Option<(u64, &str)>,
    entries: &mut Vec<CanonicalBasisEntry>,
) {
    entries.push(bool_entry(
        "branch.has_fork_observation_basis",
        fork_observation.is_some(),
    ));
    if let Some((basis_id, fork_epoch)) = fork_observation {
        entries.push(u64_entry("branch.fork_observation_basis", basis_id));
        entries.push(u64_entry("branch.fork_observation_epoch", fork_epoch));
    }
    entries.push(bool_entry(
        "branch.has_comparison_basis",
        comparison.is_some(),
    ));
    if let Some((basis_id, compared_branch)) = comparison {
        entries.push(u64_entry("branch.comparison_basis", basis_id));
        entries.push(text_entry(
            "branch.compared_against_branch",
            compared_branch,
        ));
    }
}

fn branch_kind_token(kind: FoundationalBranchLocalStateKind) -> &'static str {
    match kind {
        FoundationalBranchLocalStateKind::Candidate => "candidate",
        FoundationalBranchLocalStateKind::Staged => "staged",
    }
}
