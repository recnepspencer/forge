use worth_relational::facade::commit_strategies::{
    CanonicalStrategyCommitRequest, StrategyExecutionDraft,
};
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::transactions::{CommitResult, MutationIntent, WorkerIntentBatch};

use crate::workflow::LoweredMutationIntentDeclaration;

use super::execution::{lower_runtime_error, EffectExecutionDenialKind};
use super::execution_relational_scalar::{
    ensure_exact_basis_freshness, mutation_target_branch, mutation_transaction_options,
    open_exact_basis_snapshot,
};

pub(super) fn execute_lowered_mutation_batch(
    runtime: &mut RelationalRuntime,
    declarations: &[LoweredMutationIntentDeclaration],
) -> Result<CommitResult, (EffectExecutionDenialKind, String)> {
    for declaration in declarations {
        ensure_exact_basis_freshness(runtime, declaration)?;
    }
    let transaction_options = declarations
        .first()
        .ok_or_else(|| {
            (
                EffectExecutionDenialKind::RelationalAuthorityBindingMalformed,
                "batch-native mutation execution requires at least one declaration".to_string(),
            )
        })
        .and_then(|declaration| mutation_transaction_options(runtime, declaration))?;
    let target_branch = mutation_target_branch(
        declarations
            .first()
            .expect("non-empty mutation batch was established above"),
    )?;
    let snapshot = open_exact_basis_snapshot(runtime, &target_branch)?;
    let lowered_components = declarations
        .iter()
        .map(|declaration| lower_batch_component(runtime, declaration, &snapshot))
        .collect::<Result<Vec<_>, _>>();
    let released = runtime.snapshots().release_snapshot(&snapshot);
    if !released {
        return Err((
            EffectExecutionDenialKind::RelationalAuthorityBindingMalformed,
            "exact Relational batch strategy snapshot could not be released".to_string(),
        ));
    }
    let lowered_components = lowered_components?;
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
    let transaction_options = mutation_transaction_options(runtime, declaration)?;
    let mut authority = runtime.commit_strategies_authority();
    let lowered = authority
        .lower_execution(&canonical, &execution, transaction_options)
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyAuthorityLoweringFailed,
            )
        })?;
    Ok(lowered.merged_plan().merged_intents.clone())
}
