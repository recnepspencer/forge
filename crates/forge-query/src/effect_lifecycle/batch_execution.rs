use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;

use super::batch::LoweredEffectBatchExecutionPlan;
use super::counters::EffectLifecycleCounters;
use super::execution::{
    EffectExecutionDenialKind, ExecutedEffectAuthorityArtifact, ExecutedEffectPlan,
};
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
    batch_digest: String,
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
        let batch_digest =
            hash_parts(
                &std::iter::once("executed_effect_batch_plan_v2".to_string())
                    .chain(std::iter::once(format!(
                        "authority:{}",
                        authority_lane.as_str()
                    )))
                    .chain(std::iter::once(format!("basis:{}", basis_family.as_str())))
                    .chain(std::iter::once(format!(
                        "owner:{}",
                        authority_owner.as_str()
                    )))
                    .chain(std::iter::once(format!(
                        "aggregate:{}",
                        aggregate_artifact_digest(&aggregate_artifact)
                    )))
                    .chain(components.iter().map(|component| {
                        format!("component:{}", component.effect_execution_digest())
                    }))
                    .collect::<Vec<_>>(),
            );
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
            batch_digest,
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

    pub fn batch_digest(&self) -> &str {
        &self.batch_digest
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }

    pub fn receipt(&self) -> EffectExecutionReceipt {
        EffectExecutionReceipt::from_batch(self.clone())
    }
}

fn aggregate_artifact_digest(artifact: &ExecutedEffectAuthorityArtifact) -> String {
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
