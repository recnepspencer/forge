use super::support::*;
use crate::writeback::{
    BridgeMutationAuthorityBundleError, BridgeWritebackAuthorityOutcome,
    BridgeWritebackExecutionRecord, BridgeWritebackIdempotenceBasis,
};

struct ExecutedMutationChain {
    causality: BridgeWritebackNativeCausalityInputs,
    effect: BridgeDerivedWritebackEffect,
    feedback: crate::facade::BridgeWritebackFeedbackProvenance,
    idempotence: BridgeWritebackIdempotenceBasis,
    outcome: BridgeWritebackAuthorityOutcome,
    execution_record: BridgeWritebackExecutionRecord,
}

struct SplicedMutationArtifacts<'a> {
    causality: &'a BridgeWritebackNativeCausalityInputs,
    effect: &'a BridgeDerivedWritebackEffect,
    feedback: &'a crate::facade::BridgeWritebackFeedbackProvenance,
    execution_record: &'a BridgeWritebackExecutionRecord,
    outcome: &'a BridgeWritebackAuthorityOutcome,
}

#[test]
fn mutation_authority_rejects_cross_execution_artifact_splicing() {
    let first = execute_mutation_chain("first", "same-patch");
    let second = execute_mutation_chain("second", "same-patch");

    assert_chain_error(
        SplicedMutationArtifacts {
            causality: &second.causality,
            effect: &first.effect,
            feedback: &first.feedback,
            execution_record: &first.execution_record,
            outcome: &first.outcome,
        },
        BridgeMutationAuthorityBundleError::CausalityEffectMismatch,
    );
    assert_chain_error(
        SplicedMutationArtifacts {
            causality: &first.causality,
            effect: &first.effect,
            feedback: &second.feedback,
            execution_record: &first.execution_record,
            outcome: &first.outcome,
        },
        BridgeMutationAuthorityBundleError::FeedbackEffectMismatch,
    );
    assert_chain_error(
        SplicedMutationArtifacts {
            causality: &first.causality,
            effect: &first.effect,
            feedback: &first.feedback,
            execution_record: &second.execution_record,
            outcome: &second.outcome,
        },
        BridgeMutationAuthorityBundleError::ExecutionRecordEffectMismatch,
    );
    assert_chain_error(
        SplicedMutationArtifacts {
            causality: &first.causality,
            effect: &first.effect,
            feedback: &first.feedback,
            execution_record: &first.execution_record,
            outcome: &second.outcome,
        },
        BridgeMutationAuthorityBundleError::ExecutionRecordOutcomeMismatch,
    );
}

#[test]
fn mutation_authority_rejects_a_non_commit_outcome() {
    let chain = execute_mutation_chain("non-commit", "same-patch");
    let canonical_noop = BridgeWritebackAuthorityOutcome::canonical_noop(&chain.idempotence);

    assert_chain_error(
        SplicedMutationArtifacts {
            causality: &chain.causality,
            effect: &chain.effect,
            feedback: &chain.feedback,
            execution_record: &chain.execution_record,
            outcome: &canonical_noop,
        },
        BridgeMutationAuthorityBundleError::NonAuthoritativeOutcome,
    );
}

fn execute_mutation_chain(chain_label: &str, effect_value: &str) -> ExecutedMutationChain {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::admit_bridge_owned(format!(
                    "writeback:artifact-chain:{chain_label}"
                )),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                chain_label,
            ),
            &lowered_policy,
        )
        .expect("artifact-chain writeback declaration should admit");
    let effect_intent =
        writeback_effect_intent(BridgeWritebackEffectClass::ProjectedStateDiff, effect_value);
    let causality = mutation_causality_basis(
        BridgeWritebackCausalityIdentity::admit_bridge_owned(format!(
            "causality:artifact-chain:{chain_label}"
        )),
        chain_label,
        &effect_intent,
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::admit_bridge_owned(format!(
            "effect:artifact-chain:{chain_label}"
        )),
        effect_intent,
    );
    let feedback = crate::facade::BridgeWritebackFeedbackProvenance::new(&effect);
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned(format!(
            "idempotence:artifact-chain:{chain_label}"
        )),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (outcome, _) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("artifact-chain authority execution should commit");
    let execution_record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("artifact-chain execution record should be retained");
    ExecutedMutationChain {
        causality,
        effect,
        feedback,
        idempotence,
        outcome,
        execution_record,
    }
}

fn assert_chain_error(
    artifacts: SplicedMutationArtifacts<'_>,
    expected: BridgeMutationAuthorityBundleError,
) {
    let error =
        crate::writeback::BridgeMutationAuthorityBundle::from_successful_writeback_artifacts(
            crate::writeback::SuccessfulWritebackArtifactChain {
                causality: artifacts.causality,
                effect: artifacts.effect,
                feedback: artifacts.feedback,
                execution_record: artifacts.execution_record,
                outcome: artifacts.outcome,
            },
        )
        .expect_err("spliced mutation artifacts must not mint Bridge authority");
    assert_eq!(error, expected);
}
