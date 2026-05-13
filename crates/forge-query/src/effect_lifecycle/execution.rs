use forge_relational::facade::commit_strategies::{
    StrategyCommitRequestError, StrategyExecutionError, StrategyLoweringError,
};
use forge_relational::facade::merge::{MergeExecutionError, MergeExecutionPreparationError};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{
    CommitResult, MergeExecutionOutcome, TransactionOptions,
};
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::identity::hash_parts;

use super::counters::EffectLifecycleCounters;
use super::lowering::{LoweredEffectExecutionArtifact, LoweredEffectExecutionPlan};
use super::planning::EffectAuthorityOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExecutionDenialKind {
    MissingRelationalAuthority,
    MissingBridgeAuthority,
    WritebackContractAssemblyRequired,
    RelationalStrategyCanonicalizationFailed,
    RelationalStrategyExecutionFailed,
    RelationalStrategyAuthorityLoweringFailed,
    RelationalStrategyAuthorityValidationFailed,
    RelationalCommitFailed,
    MergePreparationFailed,
    MergeExecutionFailed,
}

impl EffectExecutionDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingRelationalAuthority => "missing_relational_authority",
            Self::MissingBridgeAuthority => "missing_bridge_authority",
            Self::WritebackContractAssemblyRequired => "writeback_contract_assembly_required",
            Self::RelationalStrategyCanonicalizationFailed => {
                "relational_strategy_canonicalization_failed"
            }
            Self::RelationalStrategyExecutionFailed => "relational_strategy_execution_failed",
            Self::RelationalStrategyAuthorityLoweringFailed => {
                "relational_strategy_authority_lowering_failed"
            }
            Self::RelationalStrategyAuthorityValidationFailed => {
                "relational_strategy_authority_validation_failed"
            }
            Self::RelationalCommitFailed => "relational_commit_failed",
            Self::MergePreparationFailed => "merge_preparation_failed",
            Self::MergeExecutionFailed => "merge_execution_failed",
        }
    }
}

#[derive(Debug)]
pub struct EffectExecutionAuthority<'a> {
    relational: Option<&'a mut RelationalRuntime>,
    _bridge: Option<&'a RuntimeBridge>,
}

impl<'a> EffectExecutionAuthority<'a> {
    pub fn relational(runtime: &'a mut RelationalRuntime) -> Self {
        Self {
            relational: Some(runtime),
            _bridge: None,
        }
    }

    pub fn bridge(runtime: &'a RuntimeBridge) -> Self {
        Self {
            relational: None,
            _bridge: Some(runtime),
        }
    }

    pub fn combined(relational: &'a mut RelationalRuntime, bridge: &'a RuntimeBridge) -> Self {
        Self {
            relational: Some(relational),
            _bridge: Some(bridge),
        }
    }

    fn relational_runtime(&mut self) -> Option<&mut RelationalRuntime> {
        self.relational.as_deref_mut()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectExecutionDenial {
    denial_kind: EffectExecutionDenialKind,
    message: String,
    lowered_effect_execution_plan_digest: String,
    denial_digest: String,
    counters: EffectLifecycleCounters,
}

impl EffectExecutionDenial {
    fn new(
        lowered: &LoweredEffectExecutionPlan,
        denial_kind: EffectExecutionDenialKind,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let denial_digest = hash_parts(&[
            "effect_execution_denial_v1".to_string(),
            format!("plan:{}", lowered.lowered_effect_execution_plan_digest()),
            format!("kind:{}", denial_kind.as_str()),
            format!("message:{message}"),
        ]);
        Self {
            denial_kind,
            message,
            lowered_effect_execution_plan_digest: lowered
                .lowered_effect_execution_plan_digest()
                .to_string(),
            denial_digest,
            counters: EffectLifecycleCounters::execution_denied(
                lowered.counters().effect_support_row_count(),
                lowered.counters().effect_lowering_width(),
                lowered.counters().effect_executor_rediscovery_count(),
            ),
        }
    }

    pub fn denial_kind(&self) -> EffectExecutionDenialKind {
        self.denial_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn lowered_effect_execution_plan_digest(&self) -> &str {
        &self.lowered_effect_execution_plan_digest
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutedEffectAuthorityArtifact {
    Mutation(CommitResult),
    Merge(MergeExecutionOutcome),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutedEffectPlan {
    lowered: LoweredEffectExecutionPlan,
    artifact: ExecutedEffectAuthorityArtifact,
    effect_execution_digest: String,
    counters: EffectLifecycleCounters,
}

impl ExecutedEffectPlan {
    fn new(
        lowered: LoweredEffectExecutionPlan,
        artifact: ExecutedEffectAuthorityArtifact,
        effect_execution_width: usize,
    ) -> Self {
        let effect_execution_digest = hash_parts(&[
            "executed_effect_plan_v1".to_string(),
            format!("plan:{}", lowered.lowered_effect_execution_plan_digest()),
            format!("artifact:{}", executed_artifact_digest(&artifact)),
        ]);
        let counters = EffectLifecycleCounters::executed(
            lowered.counters().effect_support_row_count(),
            lowered.counters().effect_lowering_width(),
            lowered.counters().effect_executor_rediscovery_count(),
            effect_execution_width,
        );
        Self {
            lowered,
            artifact,
            effect_execution_digest,
            counters,
        }
    }

    pub fn lowered(&self) -> &LoweredEffectExecutionPlan {
        &self.lowered
    }

    pub fn artifact(&self) -> &ExecutedEffectAuthorityArtifact {
        &self.artifact
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.lowered.authority_owner()
    }

    pub fn as_mutation(&self) -> Option<&CommitResult> {
        match &self.artifact {
            ExecutedEffectAuthorityArtifact::Mutation(result) => Some(result),
            ExecutedEffectAuthorityArtifact::Merge(_) => None,
        }
    }

    pub fn as_merge(&self) -> Option<&MergeExecutionOutcome> {
        match &self.artifact {
            ExecutedEffectAuthorityArtifact::Mutation(_) => None,
            ExecutedEffectAuthorityArtifact::Merge(result) => Some(result),
        }
    }

    pub fn effect_execution_digest(&self) -> &str {
        &self.effect_execution_digest
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}

impl LoweredEffectExecutionPlan {
    pub fn execute_with(
        self,
        authority: EffectExecutionAuthority<'_>,
    ) -> Result<ExecutedEffectPlan, EffectExecutionDenial> {
        execute_lowered_effect_plan(self, authority)
    }
}

pub fn execute_lowered_effect_plan(
    lowered: LoweredEffectExecutionPlan,
    mut authority: EffectExecutionAuthority<'_>,
) -> Result<ExecutedEffectPlan, EffectExecutionDenial> {
    match lowered.artifact() {
        LoweredEffectExecutionArtifact::Mutation(declaration) => {
            let runtime = authority.relational_runtime().ok_or_else(|| {
                EffectExecutionDenial::new(
                    &lowered,
                    EffectExecutionDenialKind::MissingRelationalAuthority,
                    "lowered relational mutation execution requires a relational runtime authority",
                )
            })?;
            let commit = execute_lowered_mutation(runtime, declaration)
                .map_err(|(kind, message)| EffectExecutionDenial::new(&lowered, kind, message))?;
            Ok(ExecutedEffectPlan::new(
                lowered,
                ExecutedEffectAuthorityArtifact::Mutation(commit),
                1,
            ))
        }
        LoweredEffectExecutionArtifact::Merge(declaration) => {
            let runtime = authority.relational_runtime().ok_or_else(|| {
                EffectExecutionDenial::new(
                    &lowered,
                    EffectExecutionDenialKind::MissingRelationalAuthority,
                    "lowered relational merge execution requires a relational runtime authority",
                )
            })?;
            let outcome = execute_lowered_merge(runtime, declaration)
                .map_err(|(kind, message)| EffectExecutionDenial::new(&lowered, kind, message))?;
            Ok(ExecutedEffectPlan::new(
                lowered,
                ExecutedEffectAuthorityArtifact::Merge(outcome),
                1,
            ))
        }
        LoweredEffectExecutionArtifact::Writeback(_) => Err(EffectExecutionDenial::new(
                &lowered,
                EffectExecutionDenialKind::WritebackContractAssemblyRequired,
                "lowered query writeback declarations are not yet executable in 9.3.3 without the bridge-owned admitted contract, derived effect, and idempotence chain",
            )),
    }
}

fn execute_lowered_mutation(
    runtime: &mut RelationalRuntime,
    declaration: &crate::workflow::LoweredMutationIntentDeclaration,
) -> Result<CommitResult, (EffectExecutionDenialKind, String)> {
    let canonical = runtime
        .commit_strategies()
        .canonicalize_request(declaration.strategy_request())
        .map_err(|error| {
            lower_runtime_error(
                error,
                EffectExecutionDenialKind::RelationalStrategyCanonicalizationFailed,
            )
        })?;
    let snapshot = runtime.snapshots().snapshot();
    let execution = runtime
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
        .lower_execution(&canonical, &execution, TransactionOptions::default())
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

fn execute_lowered_merge(
    runtime: &mut RelationalRuntime,
    declaration: &crate::workflow::LoweredMergeWorkflowDeclaration,
) -> Result<MergeExecutionOutcome, (EffectExecutionDenialKind, String)> {
    let prepared = runtime
        .prepare_merge_execution(declaration.merge_request().clone())
        .map_err(|error| {
            lower_runtime_error(error, EffectExecutionDenialKind::MergePreparationFailed)
        })?;
    runtime.execute_prepared_merge(prepared).map_err(|error| {
        lower_runtime_error(error, EffectExecutionDenialKind::MergeExecutionFailed)
    })
}

fn lower_runtime_error(
    error: impl std::fmt::Debug,
    kind: EffectExecutionDenialKind,
) -> (EffectExecutionDenialKind, String) {
    (kind, format!("{error:?}"))
}

fn executed_artifact_digest(artifact: &ExecutedEffectAuthorityArtifact) -> String {
    match artifact {
        ExecutedEffectAuthorityArtifact::Mutation(result) => {
            format!(
                "commit:{}:{}",
                result.outcome.commit.commit_id.0, result.outcome.commit.version_id.0
            )
        }
        ExecutedEffectAuthorityArtifact::Merge(result) => {
            format!(
                "merge:{}:{}",
                result.commit.outcome.commit.commit_id.0, result.commit.outcome.commit.version_id.0
            )
        }
    }
}

#[allow(dead_code)]
fn _type_anchor(
    _: StrategyCommitRequestError,
    _: StrategyExecutionError,
    _: StrategyLoweringError,
    _: MergeExecutionPreparationError,
    _: MergeExecutionError,
) {
}
