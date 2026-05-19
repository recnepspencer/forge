use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{CommitResult, MergeExecutionOutcome};
use forge_runtime_bridge::facade::{
    BridgeAdmittedWritebackExecution, BridgeWritebackAuthorityOutcome, RuntimeBridge,
    TruthWritebackReceipt,
};

use crate::identity::hash_parts;

use super::counters::EffectLifecycleCounters;
use super::lowering::LoweredEffectExecutionPlan;
use super::planning::EffectAuthorityOwner;
use super::receipt::EffectExecutionReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExecutionDenialKind {
    AuthorityOverrideRejected,
    MissingRelationalAuthority,
    MissingBridgeAuthority,
    BridgePolicyAdmissionFailed,
    BridgeWritebackExecutionFailed,
    RelationalAuthorityBindingMalformed,
    RelationalExactBasisStale,
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
            Self::AuthorityOverrideRejected => "authority_override_rejected",
            Self::MissingRelationalAuthority => "missing_relational_authority",
            Self::MissingBridgeAuthority => "missing_bridge_authority",
            Self::BridgePolicyAdmissionFailed => "bridge_policy_admission_failed",
            Self::BridgeWritebackExecutionFailed => "bridge_writeback_execution_failed",
            Self::RelationalAuthorityBindingMalformed => "relational_authority_binding_malformed",
            Self::RelationalExactBasisStale => "relational_exact_basis_stale",
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
    bridge: Option<&'a RuntimeBridge>,
}

impl<'a> EffectExecutionAuthority<'a> {
    pub fn relational(runtime: &'a mut RelationalRuntime) -> Self {
        Self {
            relational: Some(runtime),
            bridge: None,
        }
    }

    pub fn bridge(runtime: &'a RuntimeBridge) -> Self {
        Self {
            relational: None,
            bridge: Some(runtime),
        }
    }

    pub fn combined(relational: &'a mut RelationalRuntime, bridge: &'a RuntimeBridge) -> Self {
        Self {
            relational: Some(relational),
            bridge: Some(bridge),
        }
    }

    pub(crate) fn relational_runtime(&mut self) -> Option<&mut RelationalRuntime> {
        self.relational.as_deref_mut()
    }

    pub(crate) fn has_relational_authority(&self) -> bool {
        self.relational.is_some()
    }

    pub(crate) fn bridge_runtime(&self) -> Option<&RuntimeBridge> {
        self.bridge
    }

    pub(crate) fn has_bridge_authority(&self) -> bool {
        self.bridge.is_some()
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
    pub(crate) fn new(
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
    Writeback {
        execution: BridgeAdmittedWritebackExecution,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutedEffectPlan {
    lowered: LoweredEffectExecutionPlan,
    artifact: ExecutedEffectAuthorityArtifact,
    effect_execution_digest: String,
    counters: EffectLifecycleCounters,
}

impl ExecutedEffectPlan {
    pub(crate) fn new(
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
            ExecutedEffectAuthorityArtifact::Writeback { .. } => None,
        }
    }

    pub fn as_merge(&self) -> Option<&MergeExecutionOutcome> {
        match &self.artifact {
            ExecutedEffectAuthorityArtifact::Mutation(_) => None,
            ExecutedEffectAuthorityArtifact::Merge(result) => Some(result),
            ExecutedEffectAuthorityArtifact::Writeback { .. } => None,
        }
    }

    pub fn as_writeback(
        &self,
    ) -> Option<(&BridgeWritebackAuthorityOutcome, &TruthWritebackReceipt)> {
        match &self.artifact {
            ExecutedEffectAuthorityArtifact::Writeback { execution } => {
                Some((execution.outcome(), execution.authority_receipt()))
            }
            _ => None,
        }
    }

    pub fn writeback_execution(&self) -> Option<&BridgeAdmittedWritebackExecution> {
        match &self.artifact {
            ExecutedEffectAuthorityArtifact::Writeback { execution } => Some(execution),
            _ => None,
        }
    }

    pub fn effect_execution_digest(&self) -> &str {
        &self.effect_execution_digest
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }

    pub fn receipt(&self) -> EffectExecutionReceipt {
        EffectExecutionReceipt::from_scalar(self.clone())
    }
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
        ExecutedEffectAuthorityArtifact::Writeback { execution } => {
            format!("writeback:{}", execution.digest())
        }
    }
}
