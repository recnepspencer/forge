use forge_relational::facade::commit_strategies::{
    CanonicalStrategyCommitRequest, StrategyExecutionDraft,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{CommitResult, TransactionOptions};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

use crate::memory_workspace::ForgeQuerySnapshotIdentity;
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
    declaration
        .authority_binding()
        .runtime_target_branch()
        .cloned()
        .ok_or_else(|| {
            (
                EffectExecutionDenialKind::RelationalAuthorityBindingMalformed,
                format!(
                    "lowered relational mutation execution requires a typed target branch for authority binding `{}`",
                    declaration.authority_binding().binding_digest()
                ),
            )
        })
}

pub(super) fn ensure_exact_basis_freshness(
    runtime: &RelationalRuntime,
    declaration: &LoweredMutationIntentDeclaration,
) -> Result<(), (EffectExecutionDenialKind, String)> {
    let Some(expected_snapshot_identity) =
        declaration.authority_binding().runtime_snapshot_identity()
    else {
        return Ok(());
    };
    let target_branch = mutation_target_branch(declaration)?;
    let observed_snapshot_identity = current_branch_snapshot_identity(runtime, &target_branch)?;
    let expected_snapshot_evidence = expected_snapshot_identity.evidence_identity();
    let observed_snapshot_evidence = observed_snapshot_identity.evidence_identity();
    if expected_snapshot_evidence == observed_snapshot_evidence {
        return Ok(());
    }
    Err((
        EffectExecutionDenialKind::RelationalExactBasisStale,
        format!(
            "lowered relational mutation execution preserved runtime snapshot `{}` for branch `{}` but current authority state is `{}`",
            expected_snapshot_evidence.reporting_projection(),
            target_branch.0
            ,
            observed_snapshot_evidence.reporting_projection()
        ),
    ))
}

fn current_branch_snapshot_identity(
    runtime: &RelationalRuntime,
    branch: &BranchId,
) -> Result<ForgeQuerySnapshotIdentity, (EffectExecutionDenialKind, String)> {
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
    Ok(ForgeQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(stable_branch_snapshot_id(branch), version_id),
    ))
}

pub(crate) fn stable_branch_snapshot_id(branch: &BranchId) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in b"effect-current-branch-snapshot"
        .iter()
        .copied()
        .chain([0])
        .chain(branch.0.bytes())
    {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211);
    }
    acc
}
