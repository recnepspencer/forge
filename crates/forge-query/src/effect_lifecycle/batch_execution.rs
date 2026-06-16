use crate::basis_lifecycle::BasisFamily;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::batch::LoweredEffectBatchExecutionPlan;
use super::counters::EffectLifecycleCounters;
use super::execution::{
    EffectExecutionDenialKind, ExecutedEffectAuthorityArtifact, ExecutedEffectPlan,
};
use super::execution_artifacts::executed_authority_artifact_identity;
use super::planning::EffectAuthorityOwner;
use super::receipt::EffectExecutionReceipt;
use super::taxonomy::EffectAuthorityLane;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectBatchExecutionDenialKind {
    ComponentExecutionDenied(EffectExecutionDenialKind),
    AggregateExecutionDenied(EffectExecutionDenialKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectBatchExecutionDenial {
    component_index: Option<usize>,
    kind: EffectBatchExecutionDenialKind,
    message: String,
}

impl EffectBatchExecutionDenial {
    pub(crate) fn aggregate(kind: EffectExecutionDenialKind, message: impl Into<String>) -> Self {
        Self {
            component_index: None,
            kind: EffectBatchExecutionDenialKind::AggregateExecutionDenied(kind),
            message: message.into(),
        }
    }

    pub fn component_index(&self) -> Option<usize> {
        self.component_index
    }

    pub fn kind(&self) -> &EffectBatchExecutionDenialKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutedEffectBatchPlan {
    lowered: LoweredEffectBatchExecutionPlan,
    authority_lane: EffectAuthorityLane,
    basis_family: BasisFamily,
    authority_owner: EffectAuthorityOwner,
    aggregate_artifact: ExecutedEffectAuthorityArtifact,
    components: Vec<ExecutedEffectPlan>,
    batch_identity: ForgeQueryEvidenceIdentity,
    counters: EffectLifecycleCounters,
}

impl ExecutedEffectBatchPlan {
    pub(crate) fn new(
        lowered: LoweredEffectBatchExecutionPlan,
        authority_lane: EffectAuthorityLane,
        basis_family: BasisFamily,
        authority_owner: EffectAuthorityOwner,
        aggregate_artifact: ExecutedEffectAuthorityArtifact,
        components: Vec<ExecutedEffectPlan>,
    ) -> Self {
        let batch_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "executed_effect_batch_plan_v2",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("authority"),
                    authority_lane.as_str(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("basis"), basis_family.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("owner"),
                    authority_owner.as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("lowered"),
                    lowered.batch_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("aggregate"),
                    &executed_authority_artifact_identity(&aggregate_artifact),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("component"),
                    components
                        .iter()
                        .map(ExecutedEffectPlan::effect_execution_identity),
                )
                .seal();
        let counters = EffectLifecycleCounters::executed_batch(
            components.len(),
            components
                .iter()
                .map(|component| component.counters().effect_lowering_width())
                .sum(),
            components
                .iter()
                .map(|component| component.counters().effect_executor_rediscovery_count())
                .sum(),
            1,
        );
        Self {
            lowered,
            authority_lane,
            basis_family,
            authority_owner,
            aggregate_artifact,
            components,
            batch_identity,
            counters,
        }
    }

    pub fn authority_lane(&self) -> EffectAuthorityLane {
        self.authority_lane
    }

    pub fn lowered(&self) -> &LoweredEffectBatchExecutionPlan {
        &self.lowered
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.authority_owner
    }

    pub fn aggregate_artifact(&self) -> &ExecutedEffectAuthorityArtifact {
        &self.aggregate_artifact
    }

    pub fn aggregate_mutation(
        &self,
    ) -> Option<&forge_relational::facade::transactions::CommitResult> {
        match &self.aggregate_artifact {
            ExecutedEffectAuthorityArtifact::Mutation(result) => Some(result),
            _ => None,
        }
    }

    pub fn components(&self) -> &[ExecutedEffectPlan] {
        &self.components
    }

    pub fn batch_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.batch_identity
    }

    pub fn batch_for_reporting(&self) -> &str {
        self.batch_identity.as_str()
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }

    pub fn receipt(&self) -> EffectExecutionReceipt {
        EffectExecutionReceipt::from_batch(self.clone())
    }
}
