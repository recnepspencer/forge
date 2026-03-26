use std::collections::BTreeSet;

use crate::capabilities::{CommitEnvelopeSource, DurabilityRead};
use crate::logic::runtime::RelationalRuntime;
use crate::durability::data::RecoveryPlan;
use crate::history::data::CommitId;
use crate::replay::data::{
    CanonicalCommitEnvelope, ReplayFailureClass, ReplayObservableSurface, ReplayVerificationLayer,
};
use crate::schema::data::DescriptorSemanticsVersion;

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
    if envelope.has_lineage_authority() {
        surfaces.push(ReplayObservableSurface::Lineage);
    }
    if !envelope.index_generations.is_empty() {
        surfaces.push(ReplayObservableSurface::DerivedIndexes);
    }
    surfaces
}

pub(super) fn replay_commit_closure_by_commit_id_order(
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
    result.map(|_| ordered)
}

pub(super) fn replay_recovery_plan_for_chain(
    source: &(impl CommitEnvelopeSource + DurabilityRead),
    config: &crate::logic::runtime::RelationalRuntimeConfig,
    chain: &[CommitId],
    verification_mode: crate::replay::data::ReplayVerificationMode,
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
    RecoveryPlan::new(
        config.clone(),
        source.durable_store().cloned(),
        checkpoint.as_ref().and_then(|_| None),
        checkpoint,
        tail_log,
        crate::durability::data::RecoveryCursor {
            checkpoint_id: None,
            segment_ids: Vec::new(),
        },
        crate::durability::data::RecoveryIntegrityReport {
            selected_checkpoint_id: None,
            skipped_corrupt_checkpoints: Vec::new(),
            verified_segment_ids: Vec::new(),
            corrupt_segment_id: None,
        },
        crate::durability::data::RecoveryCompatibilityCheck::verified_at(
            ReplayVerificationLayer::DigestParity,
        ),
        replay_verification_mode_to_recovery_mode(verification_mode),
        chain
            .last()
            .and_then(|commit_id| source.commit_envelope(*commit_id))
            .map(|envelope| envelope.descriptor_semantics_version)
            .unwrap_or_else(DescriptorSemanticsVersion::default),
    )
}

fn replay_verification_mode_to_recovery_mode(
    mode: crate::replay::data::ReplayVerificationMode,
) -> crate::durability::data::RecoveryVerificationMode {
    match mode {
        crate::replay::data::ReplayVerificationMode::NormalRecoveryVerification => {
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification
        }
        crate::replay::data::ReplayVerificationMode::AuditRecoveryVerification => {
            crate::durability::data::RecoveryVerificationMode::AuditRecoveryVerification
        }
        crate::replay::data::ReplayVerificationMode::CorruptionDiagnosisReplay => {
            crate::durability::data::RecoveryVerificationMode::CorruptionDiagnosisReplay
        }
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
