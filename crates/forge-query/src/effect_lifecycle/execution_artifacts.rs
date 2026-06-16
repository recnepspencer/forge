use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{CommitResult, MergeExecutionOutcome};
use forge_runtime_bridge::facade::{
    BridgeAdmittedWritebackExecution, BridgeAdmittedWritebackExecutionReceipt,
    BridgeWritebackAuthorityOutcome, BridgeWritebackFailureClass, BridgeWritebackOutcomeClass,
    RuntimeBridge, TruthWritebackReceipt,
};

use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

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
    lowered_effect_execution_plan_identity: ForgeQueryEvidenceIdentity,
    denial_identity: ForgeQueryEvidenceIdentity,
    counters: EffectLifecycleCounters,
}

impl EffectExecutionDenial {
    pub(crate) fn new(
        lowered: &LoweredEffectExecutionPlan,
        denial_kind: EffectExecutionDenialKind,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let lowered_effect_execution_plan_identity =
            lowered.lowered_effect_execution_plan_identity().clone();
        let denial_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "effect_execution_denial_v1",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("plan"),
                    &lowered_effect_execution_plan_identity,
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), denial_kind.as_str())
                .field_shape(ForgeQueryEvidenceTag::new("message"), message.as_str())
                .seal();
        Self {
            denial_kind,
            message,
            lowered_effect_execution_plan_identity,
            denial_identity,
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

    pub fn lowered_effect_execution_plan_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.lowered_effect_execution_plan_identity
    }

    pub fn lowered_effect_execution_plan_for_reporting(&self) -> &str {
        self.lowered_effect_execution_plan_identity.as_str()
    }

    pub fn denial_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.denial_identity
    }

    pub fn denial_for_reporting(&self) -> &str {
        self.denial_identity.as_str()
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
    effect_execution_identity: ForgeQueryEvidenceIdentity,
    counters: EffectLifecycleCounters,
}

impl ExecutedEffectPlan {
    pub(crate) fn new(
        lowered: LoweredEffectExecutionPlan,
        artifact: ExecutedEffectAuthorityArtifact,
        effect_execution_width: usize,
    ) -> Self {
        let effect_execution_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "executed_effect_plan_v1",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("plan"),
                    lowered.lowered_effect_execution_plan_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("artifact"),
                    &executed_authority_artifact_identity(&artifact),
                )
                .seal();
        let counters = EffectLifecycleCounters::executed(
            lowered.counters().effect_support_row_count(),
            lowered.counters().effect_lowering_width(),
            lowered.counters().effect_executor_rediscovery_count(),
            effect_execution_width,
        );
        Self {
            lowered,
            artifact,
            effect_execution_identity,
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

    pub fn effect_execution_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.effect_execution_identity
    }

    pub fn effect_execution_for_reporting(&self) -> &str {
        self.effect_execution_identity.as_str()
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }

    pub fn receipt(&self) -> EffectExecutionReceipt {
        EffectExecutionReceipt::from_scalar(self.clone())
    }
}

pub(crate) fn executed_authority_artifact_identity(
    artifact: &ExecutedEffectAuthorityArtifact,
) -> ForgeQueryEvidenceIdentity {
    match artifact {
        ExecutedEffectAuthorityArtifact::Mutation(result) => {
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "executed_effect_mutation_authority_artifact_v1",
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("commit_id"),
                    result.outcome.commit.commit_id.0 as usize,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("version_id"),
                    result.outcome.commit.version_id.0 as usize,
                )
                .seal()
        }
        ExecutedEffectAuthorityArtifact::Merge(result) => {
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "executed_effect_merge_authority_artifact_v1",
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("commit_id"),
                    result.commit.outcome.commit.commit_id.0 as usize,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("version_id"),
                    result.commit.outcome.commit.version_id.0 as usize,
                )
                .seal()
        }
        ExecutedEffectAuthorityArtifact::Writeback { execution } => {
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "executed_effect_writeback_authority_artifact_v1",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("execution"),
                    &executed_writeback_execution_identity(execution),
                )
                .seal()
        }
    }
}

fn executed_writeback_execution_identity(
    execution: &BridgeAdmittedWritebackExecution,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "executed_writeback_execution_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("outcome"),
            &writeback_bridge_evidence_identity("outcome", execution.outcome()),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("authority_receipt"),
            &writeback_bridge_receipt_evidence_identity(
                "authority_receipt",
                execution.authority_receipt(),
            ),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("execution_receipt"),
            &writeback_bridge_execution_receipt_evidence_identity(
                "execution_receipt",
                execution.execution_receipt(),
            ),
        )
        .seal()
}

pub(crate) fn writeback_bridge_evidence_identity(
    role: &str,
    outcome: &BridgeWritebackAuthorityOutcome,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_writeback_bridge_outcome_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_shape(
            ForgeQueryEvidenceTag::new("outcome_class"),
            writeback_outcome_class_label(outcome.outcome_class()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("idempotence_digest"),
            outcome.idempotence_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authoritative_artifact_digest"),
            outcome.authoritative_artifact_digest(),
        )
        .seal()
}

pub(crate) fn writeback_bridge_receipt_evidence_identity(
    role: &str,
    receipt: &TruthWritebackReceipt,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_writeback_bridge_receipt_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_shape(
            ForgeQueryEvidenceTag::new("outcome_class"),
            writeback_outcome_class_label(receipt.outcome_class()),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("failure_class"),
            receipt.failure_class().map(writeback_failure_class_label),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("request_digest"),
            receipt.request_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authoritative_artifact_digest"),
            receipt.authoritative_artifact_digest(),
        )
        .seal()
}

pub(crate) fn writeback_bridge_execution_receipt_evidence_identity(
    role: &str,
    receipt: &BridgeAdmittedWritebackExecutionReceipt,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_writeback_bridge_execution_receipt_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_shape(
            ForgeQueryEvidenceTag::new("request_digest"),
            receipt.request_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("contract_digest"),
            receipt.contract_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lowered_policy_digest"),
            receipt.lowered_policy_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("writeback_effect_artifact_digest"),
            receipt.writeback_effect_artifact_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_intent_digest"),
            receipt.effect_intent_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("idempotence_digest"),
            receipt.idempotence_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_outcome_digest"),
            receipt.authority_outcome_digest(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("authority_receipt"),
            &writeback_bridge_receipt_evidence_identity(
                "authority_receipt",
                receipt.authority_receipt(),
            ),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("replay_bundle_digest"),
            receipt.replay_bundle_digest(),
        )
        .seal()
}

fn writeback_outcome_class_label(outcome_class: BridgeWritebackOutcomeClass) -> &'static str {
    match outcome_class {
        BridgeWritebackOutcomeClass::CanonicalNoop => "canonical-noop",
        BridgeWritebackOutcomeClass::AuthoritativeCommit => "authoritative-commit",
        BridgeWritebackOutcomeClass::Rejected => "rejected",
    }
}

fn writeback_failure_class_label(failure_class: BridgeWritebackFailureClass) -> &'static str {
    match failure_class {
        BridgeWritebackFailureClass::WritebackNotRequested => "writeback-not-requested",
        BridgeWritebackFailureClass::PolicyRejected => "policy-rejected",
        BridgeWritebackFailureClass::StrategyUnavailable => "strategy-unavailable",
        BridgeWritebackFailureClass::FamilyBindingMismatch => "family-binding-mismatch",
        BridgeWritebackFailureClass::StrategyDescriptorMismatch => "strategy-descriptor-mismatch",
        BridgeWritebackFailureClass::IdempotenceBasisMismatch => "idempotence-basis-mismatch",
        BridgeWritebackFailureClass::StaleTruthBasis => "stale-truth-basis",
        BridgeWritebackFailureClass::InvariantRejected => "invariant-rejected",
        BridgeWritebackFailureClass::MergeAuthorityRejected => "merge-authority-rejected",
        BridgeWritebackFailureClass::StrategyFailed => "strategy-failed",
        BridgeWritebackFailureClass::StrategyPanicked => "strategy-panicked",
        BridgeWritebackFailureClass::ReplayMismatch => "replay-mismatch",
        BridgeWritebackFailureClass::AuthorityDenied => "authority-denied",
        BridgeWritebackFailureClass::PreviewWritebackRejected => "preview-writeback-rejected",
    }
}
