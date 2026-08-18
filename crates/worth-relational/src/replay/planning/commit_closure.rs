use std::collections::BTreeSet;

use crate::capabilities::CommitEnvelopeSource;
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::CommitId;
use crate::replay::data::ReplayFailureClass;
use crate::runtime::RelationalRuntime;

pub(in crate::replay) fn load_replay_envelope(
    source: &impl CommitEnvelopeSource,
    commit_id: CommitId,
) -> Option<CanonicalCommitEnvelope> {
    source.commit_envelope(commit_id).cloned()
}

pub(in crate::replay) fn replay_commit_closure_by_commit_id_order(
    runtime: &RelationalRuntime,
    history: &impl CommitEnvelopeSource,
    commit_id: CommitId,
) -> Result<Vec<CommitId>, ReplayFailureClass> {
    let mut ordered = Vec::new();
    let mut visiting = BTreeSet::new();
    let mut nodes_visited = 0usize;
    let mut parent_checks = 0usize;
    let result = visit_replay_commit_closure(
        history,
        commit_id,
        &mut visiting,
        &mut ordered,
        &mut nodes_visited,
        &mut parent_checks,
    );
    runtime
        .performance_access()
        .count_merge_history_replay_planning(nodes_visited, parent_checks);
    result.map(|_| {
        include_unparented_metadata_commits(runtime, history, commit_id, &mut ordered);
        ordered
    })
}

/// Metadata-only lineage publications carry the same branch truth version as
/// their anchor commit, so they are intentionally not parents of the next
/// versioned commit. Replay still needs their generation movement before it
/// can admit the next carried branch-cell checkpoint. Include only metadata
/// whose exact parent closure is already present; this does not invent history
/// or select a head from a raw branch name.
fn include_unparented_metadata_commits(
    runtime: &RelationalRuntime,
    history: &impl CommitEnvelopeSource,
    target_commit_id: CommitId,
    ordered: &mut Vec<CommitId>,
) {
    let Some(target_envelope) = history.commit_envelope(target_commit_id) else {
        return;
    };
    let target_branch = target_envelope.branch_context.clone();
    let mut metadata = runtime
        .history
        .commit_envelopes
        .values()
        .filter(|envelope| {
            envelope.authority_kind
                == crate::history::data::CanonicalCommitAuthorityKind::MetadataOnlyLineage
                && envelope.branch_context == target_branch
                && envelope.commit.commit_id.0 < target_commit_id.0
                && !ordered.contains(&envelope.commit.commit_id)
                && history.commit_envelope(envelope.commit.commit_id).is_some()
        })
        .map(|envelope| envelope.commit.commit_id)
        .collect::<Vec<_>>();
    metadata.sort_unstable();
    let mut target_index = ordered
        .iter()
        .position(|commit_id| *commit_id == target_commit_id)
        .unwrap_or(ordered.len());
    for metadata_id in metadata {
        let Some(metadata_envelope) = history.commit_envelope(metadata_id) else {
            continue;
        };
        let Some(parent_index) = metadata_envelope
            .commit
            .ordered_parents()
            .as_slice()
            .iter()
            .map(|parent| ordered.iter().position(|id| id == parent))
            .collect::<Option<Vec<_>>>()
            .and_then(|positions| positions.into_iter().max())
        else {
            continue;
        };
        let insert_at = (parent_index + 1).min(target_index);
        ordered.insert(insert_at, metadata_id);
        target_index += 1;
    }
}

fn visit_replay_commit_closure(
    history: &impl CommitEnvelopeSource,
    commit_id: CommitId,
    visiting: &mut BTreeSet<CommitId>,
    ordered: &mut Vec<CommitId>,
    nodes_visited: &mut usize,
    parent_checks: &mut usize,
) -> Result<(), ReplayFailureClass> {
    if ordered.contains(&commit_id) {
        return Ok(());
    }
    let Some(envelope) = history.commit_envelope(commit_id) else {
        return Err(ReplayFailureClass::MissingAuthoritativeParentClosure);
    };
    if !visiting.insert(commit_id) {
        return Err(ReplayFailureClass::MissingAuthoritativeParentClosure);
    }
    *nodes_visited += 1;
    // Replay walks parents in authoritative published order. This traversal
    // must not normalize, sort, or reinterpret the post-publication parent
    // list.
    for parent in envelope.commit.ordered_parents().as_slice() {
        *parent_checks += 1;
        visit_replay_commit_closure(
            history,
            *parent,
            visiting,
            ordered,
            nodes_visited,
            parent_checks,
        )?;
    }
    visiting.remove(&commit_id);
    ordered.push(commit_id);
    Ok(())
}
