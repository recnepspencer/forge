use crate::capabilities::{
    CommitEnvelopeSource, DurabilityRead, PatchStreamCommitRef, PatchStreamSource,
};
use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::CommitId;
use crate::publication::patch::data::PatchStreamPosition;
use crate::runtime::RelationalRuntime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RetainedCanonicalEnvelopeGap {
    pub position: PatchStreamPosition,
    pub commit_id: CommitId,
}

pub(crate) fn retained_canonical_envelopes_after(
    runtime: &RelationalRuntime,
    after_position: Option<PatchStreamPosition>,
    max_commits: usize,
) -> Result<Vec<CanonicalCommitEnvelope>, RetainedCanonicalEnvelopeGap> {
    let commit_refs = runtime.patch_stream_commits_after(after_position, max_commits);
    resolve_retained_commit_refs(runtime, &commit_refs)
}

pub(crate) fn retained_canonical_envelope_at_position(
    runtime: &RelationalRuntime,
    position: PatchStreamPosition,
    preloaded_durable_envelopes: Option<&[CanonicalCommitEnvelope]>,
) -> Option<CanonicalCommitEnvelope> {
    runtime
        .commit_envelope_at_patch_stream_position(position)
        .cloned()
        .or_else(|| {
            let durable = preloaded_durable_envelopes
                .map(|envelopes| envelopes.to_vec())
                .unwrap_or_else(|| durable_canonical_envelopes(runtime));
            durable
                .into_iter()
                .find(|envelope| envelope.patch.position == position)
        })
}

pub(crate) fn durable_canonical_envelopes(
    runtime: &RelationalRuntime,
) -> Vec<CanonicalCommitEnvelope> {
    if runtime.durable_checkpoints().is_empty() && runtime.durable_log().is_empty() {
        return Vec::new();
    }

    let recovery_plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut envelopes = recovery_plan
        .checkpoint
        .map(|checkpoint| checkpoint.envelopes)
        .unwrap_or_default();
    envelopes.extend(recovery_plan.tail_log);
    envelopes.sort_by_key(|envelope| envelope.patch.position);
    envelopes
}

fn resolve_retained_commit_refs(
    runtime: &RelationalRuntime,
    commit_refs: &[PatchStreamCommitRef],
) -> Result<Vec<CanonicalCommitEnvelope>, RetainedCanonicalEnvelopeGap> {
    let mut durable_envelopes = None;
    let mut envelopes = Vec::with_capacity(commit_refs.len());

    for commit_ref in commit_refs {
        if let Some(envelope) = runtime.commit_envelope(commit_ref.commit_id).cloned() {
            envelopes.push(envelope);
            continue;
        }

        let durable = durable_envelopes.get_or_insert_with(|| durable_canonical_envelopes(runtime));
        if let Some(envelope) = durable
            .iter()
            .find(|envelope| envelope.patch.position == commit_ref.position)
            .cloned()
        {
            envelopes.push(envelope);
            continue;
        }

        return Err(RetainedCanonicalEnvelopeGap {
            position: commit_ref.position,
            commit_id: commit_ref.commit_id,
        });
    }

    Ok(envelopes)
}
