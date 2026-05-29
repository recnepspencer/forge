use crate::capabilities::{CommitEnvelopeSource, DurabilityRead};
use crate::durability::data::RecoveryPlan;
use crate::history::data::CommitId;
use crate::replay::data::ReplayVerificationLayer;
use crate::schema::data::DescriptorSemanticsVersion;

pub(in crate::replay::logic) fn replay_recovery_plan_for_chain(
    source: &(impl CommitEnvelopeSource + DurabilityRead),
    config: &crate::logic::runtime::RelationalRuntimeConfig,
    commit_strategy_executors: crate::commit_strategies::FrozenCommitStrategyExecutorRegistry,
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
    let target_commit_id = chain.last().copied();
    let mut restore_authoritative_envelope_commit_ids = chain
        .iter()
        .copied()
        .filter(|commit_id| Some(*commit_id) != target_commit_id)
        .chain(
            chain
                .iter()
                .copied()
                .filter_map(|commit_id| source.commit_envelope(commit_id))
                .filter(|envelope| envelope.strategy_artifacts.is_some())
                .map(|envelope| envelope.commit.commit_id),
        )
        .collect::<Vec<_>>();
    restore_authoritative_envelope_commit_ids.sort_unstable();
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
        restore_authoritative_envelope_commit_ids,
    )
    .with_commit_strategy_executors(commit_strategy_executors)
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
