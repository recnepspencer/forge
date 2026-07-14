use std::collections::BTreeSet;

use worth_relational::facade::history::CommitId;

use crate::{backend::records::StoreState, retention::RetainedAuthoritativeRange};

pub(super) fn retained_ranges_for_policy(
    state: &StoreState,
    policy: &crate::ConservativeRetentionPolicy,
    closure_commit_set: &BTreeSet<CommitId>,
) -> Vec<RetainedAuthoritativeRange> {
    let mut branch_ids = policy
        .branch_history_windows()
        .iter()
        .map(|window| window.branch_id().clone())
        .collect::<Vec<_>>();
    branch_ids.extend(
        state
            .branch_head_records
            .values()
            .filter(|record| record.head_commit_id.is_some())
            .map(|record| record.branch_id.clone()),
    );
    branch_ids.sort();
    branch_ids.dedup();

    branch_ids
        .into_iter()
        .filter_map(|branch_id| {
            let commit_ids = state
                .branch_commit_sequences(&branch_id)
                .into_iter()
                .map(|(_, commit_id)| commit_id)
                .filter(|commit_id| closure_commit_set.contains(commit_id))
                .collect::<Vec<_>>();
            (!commit_ids.is_empty()).then(|| RetainedAuthoritativeRange::new(branch_id, commit_ids))
        })
        .collect()
}

pub(super) fn expired_ranges_for_policy(
    state: &StoreState,
    policy: &crate::ConservativeRetentionPolicy,
    closure_commit_set: &BTreeSet<CommitId>,
) -> Vec<crate::PolicyExpiredAuthorityRange> {
    policy
        .branch_history_windows()
        .iter()
        .filter_map(|window| {
            let commits = state
                .branch_commit_sequences(window.branch_id())
                .into_iter()
                .map(|(_, commit_id)| commit_id)
                .collect::<Vec<_>>();
            let retained_for_branch = commits
                .iter()
                .copied()
                .filter(|commit_id| closure_commit_set.contains(commit_id))
                .collect::<Vec<_>>();
            let expired = commits
                .into_iter()
                .filter(|commit_id| !closure_commit_set.contains(commit_id))
                .collect::<Vec<_>>();
            (!expired.is_empty()).then(|| {
                crate::PolicyExpiredAuthorityRange::new(
                    window.branch_id().clone(),
                    retained_for_branch.first().copied(),
                    expired,
                )
            })
        })
        .collect()
}
