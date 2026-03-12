use std::collections::BTreeSet;

use crate::capabilities::{CommitEnvelopeSource, DurabilityRead};
use crate::durability::data::RecoveryPlan;
use crate::history::data::CommitId;
use crate::replay::data::{
    CanonicalCommitEnvelope, ReplayFailureClass, ReplayObservableSurface,
};

pub(super) fn load_replay_envelope(
    source: &impl CommitEnvelopeSource,
    commit_id: CommitId,
) -> Option<CanonicalCommitEnvelope> {
    source.commit_envelope(commit_id).cloned()
}

pub(super) fn promised_replay_surfaces(
    envelope: &CanonicalCommitEnvelope,
) -> Vec<ReplayObservableSurface> {
    let mut surfaces = vec![
        ReplayObservableSurface::Snapshot,
        ReplayObservableSurface::Patch,
        ReplayObservableSurface::Diagnostics,
        ReplayObservableSurface::History,
        ReplayObservableSurface::BranchHead,
    ];
    if !envelope.lineage_event_ids.is_empty() {
        surfaces.push(ReplayObservableSurface::Lineage);
    }
    if !envelope.index_generation_ids.is_empty() {
        surfaces.push(ReplayObservableSurface::DerivedIndexes);
    }
    surfaces
}

pub(super) fn replay_chain(
    history: &impl CommitEnvelopeSource,
    commit_id: CommitId,
) -> Result<Vec<CommitId>, ReplayFailureClass> {
    let mut ordered = Vec::new();
    let mut visiting = BTreeSet::new();
    visit_replay_chain(history, commit_id, &mut visiting, &mut ordered)?;
    Ok(ordered)
}

pub(super) fn replay_recovery_plan_for_chain(
    source: &(impl CommitEnvelopeSource + DurabilityRead),
    config: &crate::logic::runtime::RelationalRuntimeConfig,
    chain: &[CommitId],
) -> RecoveryPlan {
    let checkpoint = source
        .durable_checkpoints()
        .iter()
        .rev()
        .find(|checkpoint| {
            checkpoint
                .coverage
                .up_to_commit
                .as_ref()
                .map(|commit| chain.contains(&commit.commit_id))
                .unwrap_or(false)
        })
        .cloned();
    let tail_start = checkpoint
        .as_ref()
        .and_then(|checkpoint| checkpoint.coverage.up_to_commit.as_ref())
        .map(|commit| commit.commit_id);
    let tail_log = chain
        .iter()
        .copied()
        .filter(|commit_id| tail_start.is_none_or(|start| *commit_id > start))
        .filter_map(|commit_id| source.commit_envelope(commit_id).cloned())
        .collect();
    RecoveryPlan {
        config: config.clone(),
        store: source.durable_store().cloned(),
        checkpoint_manifest: checkpoint.as_ref().and_then(|_| None),
        checkpoint,
        tail_log,
        cursor: crate::durability::data::RecoveryCursor {
            checkpoint_id: None,
            segment_ids: Vec::new(),
        },
        integrity_report: crate::durability::data::RecoveryIntegrityReport {
            selected_checkpoint_id: None,
            skipped_corrupt_checkpoints: Vec::new(),
            verified_segment_ids: Vec::new(),
            corrupt_segment_id: None,
        },
        compatibility: crate::durability::data::RecoveryCompatibilityCheck {
            schema_match: true,
            profile_match: true,
            runtime_name_match: true,
        },
    }
}

fn visit_replay_chain(
    history: &impl CommitEnvelopeSource,
    commit_id: CommitId,
    visiting: &mut BTreeSet<CommitId>,
    ordered: &mut Vec<CommitId>,
) -> Result<(), ReplayFailureClass> {
    if ordered.contains(&commit_id) {
        return Ok(());
    }
    let Some(envelope) = history.commit_envelope(commit_id) else {
        return Err(ReplayFailureClass::MissingParentChain);
    };
    if !visiting.insert(commit_id) {
        return Err(ReplayFailureClass::MissingParentChain);
    }
    for parent in &envelope.commit.parents {
        visit_replay_chain(history, *parent, visiting, ordered)?;
    }
    visiting.remove(&commit_id);
    ordered.push(commit_id);
    Ok(())
}
