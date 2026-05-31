use crate::capabilities::RuntimeConfigSource;
use crate::commit_strategies::data::StrategyCommitArtifactBundle;
use crate::history::data::{BranchId, CommitId};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{
    CanonicalCommitEnvelope, ReplayMismatch, ReplayMismatchClass, ReplayObservableSurface,
    ReplayVerificationLayer, ReplayVerificationMode,
};
use crate::transactions::data::TransactionOptions;

use super::super::planning::replay_commit_closure_by_commit_id_order;

pub(super) fn verify_strategy_reexecution_surface(
    runtime: &mut RelationalRuntime,
    mismatches: &mut Vec<ReplayMismatch>,
    envelope: &CanonicalCommitEnvelope,
    commit_closure: &[CommitId],
    verification_mode: ReplayVerificationMode,
) {
    let Some(expected_artifacts) = envelope.strategy_artifacts.as_ref() else {
        return;
    };
    let basis_commits = match (
        expected_artifacts.validated_against_commit_id(),
        expected_artifacts.validated_against_version_id(),
    ) {
        (Some(basis_commit_id), _) => {
            match replay_commit_closure_by_commit_id_order(runtime, runtime, basis_commit_id) {
                Ok(chain) => chain,
                Err(failure) => {
                    mismatches.push(strategy_mismatch(
                        ReplayMismatchClass::StrategyExecutorUnavailable,
                        format!(
                        "failed to rebuild strategy replay validation basis closure: {failure:?}"
                    ),
                        expected_artifacts,
                        None,
                    ));
                    return;
                }
            }
        }
        (None, Some(validated_version_id)) if validated_version_id.0 > 0 => {
            let history = runtime.history();
            let Some(basis_commit_id) = history
                .commit_envelope_for_version(validated_version_id)
                .map(|envelope| envelope.commit.commit_id)
            else {
                mismatches.push(strategy_mismatch(
                    ReplayMismatchClass::StrategyExecutorUnavailable,
                    format!(
                        "failed to resolve strategy replay validation basis version {:?}",
                        validated_version_id
                    ),
                    expected_artifacts,
                    None,
                ));
                return;
            };
            match replay_commit_closure_by_commit_id_order(runtime, runtime, basis_commit_id) {
                Ok(chain) => chain,
                Err(failure) => {
                    mismatches.push(strategy_mismatch(
                        ReplayMismatchClass::StrategyExecutorUnavailable,
                        format!(
                            "failed to rebuild strategy replay validation basis closure: {failure:?}"
                        ),
                        expected_artifacts,
                        None,
                    ));
                    return;
                }
            }
        }
        _ => commit_closure[..commit_closure.len().saturating_sub(1)].to_vec(),
    };
    let basis_plan = super::super::planning::replay_recovery_plan_for_chain(
        runtime,
        runtime.runtime_config(),
        runtime.commit_strategy_executor_registry().clone(),
        &basis_commits,
        verification_mode,
    );
    let mut basis_runtime = match RelationalRuntime::rebuild_runtime_from_plan(basis_plan) {
        Ok(runtime) => runtime,
        Err(error) => {
            mismatches.push(strategy_mismatch(
                ReplayMismatchClass::StrategyExecutorUnavailable,
                format!(
                    "failed to rebuild pre-commit strategy replay basis: {}",
                    error.detail
                ),
                expected_artifacts,
                None,
            ));
            return;
        }
    };
    if let Err(detail) =
        ensure_strategy_replay_basis_branch(&mut basis_runtime, envelope, expected_artifacts)
    {
        mismatches.push(strategy_mismatch(
            ReplayMismatchClass::StrategyExecutorUnavailable,
            detail,
            expected_artifacts,
            None,
        ));
        return;
    }
    let replay_request = expected_artifacts.replay_request();
    let snapshot = basis_runtime.visibility_authority().snapshot();
    let execution = match basis_runtime
        .commit_strategies()
        .execute(&replay_request, &snapshot)
    {
        Ok(execution) => execution,
        Err(error) => {
            let mismatch_class = match error {
                crate::commit_strategies::StrategyExecutionError::UnknownStrategyId { .. }
                | crate::commit_strategies::StrategyExecutionError::UnboundStrategyExecutor {
                    ..
                } => ReplayMismatchClass::StrategyExecutorUnavailable,
                _ => ReplayMismatchClass::StrategyExecutionFailure,
            };
            mismatches.push(strategy_mismatch(
                mismatch_class,
                format!("strategy replay re-execution failed: {error:?}"),
                expected_artifacts,
                None,
            ));
            return;
        }
    };
    let lowered = match basis_runtime.commit_strategies_authority().lower_execution(
        &replay_request,
        &execution,
        TransactionOptions {
            target_branch: Some(envelope.branch_context.clone()),
            merge_parent_branches: envelope.merge_parent_branches.clone(),
            ..TransactionOptions::default()
        },
    ) {
        Ok(lowered) => lowered,
        Err(error) => {
            mismatches.push(strategy_mismatch(
                ReplayMismatchClass::StrategyLoweringDrift,
                format!("strategy replay re-lowering failed: {error:?}"),
                expected_artifacts,
                None,
            ));
            return;
        }
    };
    let descriptor = basis_runtime
        .commit_strategy_registry()
        .get_by_id(replay_request.strategy_id())
        .expect("replayed strategy request should resolve to a registered descriptor")
        .descriptor()
        .clone();
    let observed = if expected_artifacts.preview_validation_summary().is_some()
        || expected_artifacts.preview_validation_cost().is_some()
        || expected_artifacts.validated_against_version_id().is_some()
    {
        match basis_runtime
            .commit_strategies_authority()
            .validate_lowered_plan(lowered.clone())
        {
            Ok(validated) => StrategyCommitArtifactBundle::from_lowered(
                validated.lowered_plan(),
                &descriptor,
                basis_runtime.runtime_config(),
            )
            .with_preview_validation(
                validated.validation_summary(),
                validated.preview_validation_cost(),
                validated.validated_against_commit_id(),
                validated.validated_against_version_id(),
            ),
            Err(error) => {
                mismatches.push(strategy_mismatch(
                    ReplayMismatchClass::StrategyLoweringDrift,
                    format!("strategy replay re-validation failed: {error:?}"),
                    expected_artifacts,
                    None,
                ));
                return;
            }
        }
    } else {
        StrategyCommitArtifactBundle::from_lowered(
            &lowered,
            &descriptor,
            basis_runtime.runtime_config(),
        )
    };
    if observed.lowering_provenance() == expected_artifacts.lowering_provenance()
        && observed.lowering_summary() == expected_artifacts.lowering_summary()
        && observed.preview_validation_summary() == expected_artifacts.preview_validation_summary()
        && observed.preview_validation_cost() == expected_artifacts.preview_validation_cost()
        && observed.validated_against_commit_id()
            == expected_artifacts.validated_against_commit_id()
        && observed.validated_against_version_id()
            == expected_artifacts.validated_against_version_id()
        && execution.output().digest() == expected_artifacts.replay_descriptor().output_digest()
        && execution.mutation_program().digest()
            == expected_artifacts
                .replay_descriptor()
                .mutation_program_digest()
        && execution.summary().unmasked_entity_record_reads
            == expected_artifacts
                .lowering_summary()
                .unmasked_entity_record_reads()
        && execution.summary().unmasked_relation_record_reads
            == expected_artifacts
                .lowering_summary()
                .unmasked_relation_record_reads()
        && execution.summary().projected_partition_reads
            == expected_artifacts
                .lowering_summary()
                .projected_partition_reads()
    {
        runtime
            .performance_access()
            .count_replay_verification_layer(ReplayVerificationLayer::DeepArtifactParity);
        return;
    }
    runtime
        .performance_access()
        .count_replay_verification_layer(ReplayVerificationLayer::DeepArtifactParity);
    mismatches.push(strategy_mismatch(
        ReplayMismatchClass::StrategyLoweringDrift,
        "strategy replay re-execution or re-lowering diverged from the committed strategy proof"
            .to_string(),
        expected_artifacts,
        Some(format!("{:?}", observed.replay_descriptor())),
    ));
}

fn ensure_strategy_replay_basis_branch(
    basis_runtime: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
    expected_artifacts: &StrategyCommitArtifactBundle,
) -> Result<(), String> {
    if envelope.branch_context == BranchId("main".to_string())
        || basis_runtime
            .history()
            .branch_head(&envelope.branch_context)
            .is_some()
    {
        return Ok(());
    }

    let source_branch = expected_artifacts
        .validated_against_commit_id()
        .and_then(|commit_id| {
            basis_runtime
                .replay()
                .canonical_commit_envelope(commit_id)
                .map(|basis| basis.branch_context.clone())
        })
        .unwrap_or_else(|| BranchId("main".to_string()));

    basis_runtime
        .history_authority()
        .create_branch(envelope.branch_context.clone(), &source_branch)
        .map_err(|error| {
            format!(
                "failed to reconstruct strategy replay basis branch {:?} from {:?}: {error:?}",
                envelope.branch_context, source_branch
            )
        })
}

fn strategy_mismatch(
    class: ReplayMismatchClass,
    detail: String,
    expected_artifacts: &StrategyCommitArtifactBundle,
    observed: Option<String>,
) -> ReplayMismatch {
    ReplayMismatch {
        class,
        history_drift_class: None,
        surface: ReplayObservableSurface::Strategy,
        verification_layer: ReplayVerificationLayer::DeepArtifactParity,
        detail,
        expected: Some(format!("{:?}", expected_artifacts.replay_descriptor())),
        observed,
    }
}
