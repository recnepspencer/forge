use std::sync::Arc;

use crate::history::data::CanonicalCommitEnvelope;
use crate::indexes::data::DerivedIndexArtifacts;
use crate::runtime::RelationalRuntime;

pub(crate) fn checkpoint_derived_index_artifacts(
    runtime: &RelationalRuntime,
) -> DerivedIndexArtifacts {
    DerivedIndexArtifacts::new(runtime.index_access().generations_snapshot())
}

pub(crate) fn restore_checkpoint_derived_index_artifacts(
    runtime: &mut RelationalRuntime,
    artifacts: &DerivedIndexArtifacts,
) {
    for generation in artifacts.generations() {
        runtime
            .indexes
            .generations
            .entry(generation.index_id)
            .or_default()
            .push(generation.clone());
    }
}

pub(crate) fn apply_envelope_derived_index_artifacts(
    runtime: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
) {
    if envelope.derived_index_artifacts().is_empty() {
        return;
    }

    for generation in envelope.derived_index_artifacts().generations() {
        let generations = runtime
            .indexes
            .generations
            .entry(generation.index_id)
            .or_default();
        if let Some(existing) = generations
            .iter_mut()
            .find(|candidate| candidate.generation_id == generation.generation_id)
        {
            *existing = generation.clone();
        } else {
            generations.push(generation.clone());
            generations.sort_by_key(|candidate| candidate.generation_id);
        }
    }

    let commit_envelope = runtime
        .history
        .commit_envelopes
        .get_mut(&envelope.commit.commit_id)
        .expect("authoritative durable envelope must be present after restoration");
    Arc::make_mut(commit_envelope)
        .append_index_generations_canonical(envelope.derived_index_artifacts().generations());
}
