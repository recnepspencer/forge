use forge_relational::facade::commit_strategies::{
    CanonicalStrategyCommitRequest, StrategyExecutionDraft,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{CommitResult, TransactionOptions};

use crate::workflow::LoweredMutationIntentDeclaration;

use super::execution::{lower_runtime_error, EffectExecutionDenialKind};

pub(super) fn execute_lowered_mutation(
    runtime: &mut RelationalRuntime,
    declaration: &LoweredMutationIntentDeclaration,
) -> Result<CommitResult, (EffectExecutionDenialKind, String)> {
    ensure_exact_basis_freshness(runtime, declaration)?;
    let transaction_options = mutation_transaction_options(declaration)?;
    let canonical: CanonicalStrategyCommitRequest = runtime
        .commit_strategies()
        .canonicalize_request(declaration.strategy_request())
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyCanonicalizationFailed,
            )
        })?;
    let snapshot = runtime.snapshots().snapshot();
    let execution: StrategyExecutionDraft = runtime
        .commit_strategies()
        .execute(&canonical, &snapshot)
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyExecutionFailed,
            )
        })?;
    let mut commit_authority = runtime.commit_strategies_authority();
    let lowered = commit_authority
        .lower_execution(&canonical, &execution, transaction_options)
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyAuthorityLoweringFailed,
            )
        })?;
    let validated = commit_authority
        .validate_lowered_plan(lowered)
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyAuthorityValidationFailed,
            )
        })?;
    commit_authority
        .execute_validated_commit(validated)
        .map_err(|error| {
            lower_runtime_error(error, EffectExecutionDenialKind::RelationalCommitFailed)
        })
}

pub(super) fn mutation_transaction_options(
    declaration: &LoweredMutationIntentDeclaration,
) -> Result<TransactionOptions, (EffectExecutionDenialKind, String)> {
    mutation_target_branch(declaration).map(|target_branch| TransactionOptions {
        target_branch: Some(target_branch),
        ..TransactionOptions::default()
    })
}

pub(crate) fn mutation_target_branch(
    declaration: &LoweredMutationIntentDeclaration,
) -> Result<BranchId, (EffectExecutionDenialKind, String)> {
    parse_target_branch_binding(declaration.authority_binding().binding_digest())
}

pub(super) fn ensure_exact_basis_freshness(
    runtime: &RelationalRuntime,
    declaration: &LoweredMutationIntentDeclaration,
) -> Result<(), (EffectExecutionDenialKind, String)> {
    let Some(expected_snapshot_token) = declaration.authority_binding().runtime_snapshot_token()
    else {
        return Ok(());
    };
    let target_branch = mutation_target_branch(declaration)?;
    let observed_snapshot_token = current_branch_snapshot_token(runtime, &target_branch)?;
    if expected_snapshot_token == observed_snapshot_token {
        return Ok(());
    }
    Err((
        EffectExecutionDenialKind::RelationalExactBasisStale,
        format!(
            "lowered relational mutation execution preserved runtime snapshot `{expected_snapshot_token}` for branch `{}` but current authority state is `{observed_snapshot_token}`",
            target_branch.0
        ),
    ))
}

fn current_branch_snapshot_token(
    runtime: &RelationalRuntime,
    branch: &BranchId,
) -> Result<String, (EffectExecutionDenialKind, String)> {
    let version_id = runtime
        .history()
        .branch_head(branch)
        .map(|head| head.version_id.0)
        .ok_or_else(|| {
            (
                EffectExecutionDenialKind::RelationalAuthorityBindingMalformed,
                format!(
                    "lowered relational mutation execution could not resolve a current head for branch `{}`",
                    branch.0
                ),
            )
        })?;
    Ok(format!("snapshot-{version_id}"))
}

fn parse_target_branch_binding(
    binding_digest: &str,
) -> Result<BranchId, (EffectExecutionDenialKind, String)> {
    if binding_digest == "runtime-current-head" {
        return Ok(BranchId("main".to_string()));
    }
    if let Some(branch_identity) = binding_digest.strip_prefix("relational-branch:") {
        return Ok(branch_id_from_identity(branch_identity));
    }
    if let Some((_, branch_identity)) = binding_digest.rsplit_once(":branch:") {
        return Ok(branch_id_from_identity(branch_identity));
    }
    Err((
        EffectExecutionDenialKind::RelationalAuthorityBindingMalformed,
        format!(
            "lowered relational mutation execution could not derive a target branch from authority binding `{binding_digest}`"
        ),
    ))
}

fn branch_id_from_identity(branch_identity: &str) -> BranchId {
    let branch_identity = branch_identity
        .split_once(":snapshot:")
        .map_or(branch_identity, |(branch_identity, _)| branch_identity);
    BranchId(branch_identity.to_string())
}
