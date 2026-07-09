use worth_relational::facade::commit_strategies::{
    CanonicalStrategyCommitRequest, StrategyExecutionDraft,
};
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::transactions::{CommitResult, MutationIntent, WorkerIntentBatch};

use crate::workflow::LoweredMutationIntentDeclaration;

use super::execution::{lower_runtime_error, EffectExecutionDenialKind};
use super::execution_relational_scalar::{
    ensure_exact_basis_freshness, mutation_transaction_options,
};

pub(super) fn execute_lowered_mutation_batch(
    runtime: &mut RelationalRuntime,
    declarations: &[LoweredMutationIntentDeclaration],
) -> Result<CommitResult, (EffectExecutionDenialKind, String)> {
    for declaration in declarations {
        ensure_exact_basis_freshness(runtime, declaration)?;
    }
    let snapshot = runtime.snapshots().snapshot();
    let transaction_options = declarations
        .first()
        .ok_or_else(|| {
            (
                EffectExecutionDenialKind::RelationalAuthorityBindingMalformed,
                "batch-native mutation execution requires at least one declaration".to_string(),
            )
        })
        .and_then(mutation_transaction_options)?;
    let lowered_components = declarations
        .iter()
        .map(|declaration| lower_batch_component(runtime, declaration, &snapshot))
        .collect::<Result<Vec<_>, _>>()?;
    let mut batch = WorkerIntentBatch::new("worth-query-effect-batch");
    for intents in lowered_components {
        for intent in intents {
            batch.intents.push(intent);
        }
    }
    let mut txn = runtime.begin_transaction(transaction_options);
    txn.push_batch(batch);
    txn.commit().map_err(|error| {
        lower_runtime_error(error, EffectExecutionDenialKind::RelationalCommitFailed)
    })
}

fn lower_batch_component(
    runtime: &mut RelationalRuntime,
    declaration: &LoweredMutationIntentDeclaration,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
) -> Result<Vec<MutationIntent>, (EffectExecutionDenialKind, String)> {
    let canonical: CanonicalStrategyCommitRequest = runtime
        .commit_strategies()
        .canonicalize_request(declaration.strategy_request())
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyCanonicalizationFailed,
            )
        })?;
    let execution: StrategyExecutionDraft = runtime
        .commit_strategies()
        .execute(&canonical, snapshot)
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyExecutionFailed,
            )
        })?;
    let mut authority = runtime.commit_strategies_authority();
    let lowered = authority
        .lower_execution(
            &canonical,
            &execution,
            mutation_transaction_options(declaration)?,
        )
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyAuthorityLoweringFailed,
            )
        })?;
    Ok(lowered.merged_plan().merged_intents.clone())
}
