use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectVersion;
use crate::data::output::{
    ComputationFamily, ComputationKey, IntoNodeEvaluationResult, MemoizedResultOrigin,
    NodeEvaluationResult, StructuralMemoKey,
};
use crate::data::proof::PartitionScopeSet;
use crate::data::reuse::PersistentCorrespondenceEvidence;
use crate::data::temporal::LoweredTemporalEligibility;
use crate::data::trace::CausalityMetadata;

use super::capture::PreparedDependencyCapture;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PreparedTraceData {
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub causality: Option<CausalityMetadata>,
    #[serde(default)]
    pub temporal_eligibility: Option<LoweredTemporalEligibility>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PreparedEvaluationOutcome {
    #[default]
    Evaluate,
    ValidatedClean,
    DeferredByInvalidation,
    DeferredByCondition,
    RevertedCleanByCondition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PreparedEvaluationOrigin {
    #[default]
    DirectPrecompute,
    MemoizedReuse,
    CrossIdentityPersistentReuse,
    PartialArtifactSplice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PreparedMemoDecision {
    #[default]
    None,
    Hit,
    Miss,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PreparedKeyedContext {
    #[serde(default)]
    pub family: Option<ComputationFamily>,
    #[serde(default)]
    pub key: Option<ComputationKey>,
    #[serde(default)]
    pub memo_key: Option<StructuralMemoKey>,
    pub memoized_origin: MemoizedResultOrigin,
    #[serde(default)]
    pub persistent_correspondence: Option<PersistentCorrespondenceEvidence>,
    #[serde(default)]
    pub composition_regions: PartitionScopeSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedEvaluation {
    pub result: NodeEvaluationResult,
    pub dependencies: PreparedDependencyCapture,
    pub trace_data: PreparedTraceData,
    pub outcome: PreparedEvaluationOutcome,
    pub origin: PreparedEvaluationOrigin,
    pub memo_decision: PreparedMemoDecision,
    #[serde(default)]
    pub keyed: Option<PreparedKeyedContext>,
}

impl PreparedEvaluation {
    pub fn from_result(result: impl IntoNodeEvaluationResult) -> Self {
        Self {
            result: result.into_evaluation_result(),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::Evaluate,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub fn validated_clean() -> Self {
        Self {
            result: NodeEvaluationResult::from_version(AspectVersion::zero()),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::ValidatedClean,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub fn deferred_by_condition() -> Self {
        Self {
            result: NodeEvaluationResult::from_version(AspectVersion::zero()),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::DeferredByCondition,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub(crate) fn deferred_by_invalidation() -> Self {
        Self {
            result: NodeEvaluationResult::from_version(AspectVersion::zero()),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::DeferredByInvalidation,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub fn deferred_by_time(temporal_eligibility: LoweredTemporalEligibility) -> Self {
        Self::deferred_by_condition().with_temporal_eligibility(temporal_eligibility)
    }

    pub fn reverted_clean_by_condition() -> Self {
        Self {
            result: NodeEvaluationResult::from_version(AspectVersion::zero()),
            dependencies: PreparedDependencyCapture::default(),
            trace_data: PreparedTraceData::default(),
            outcome: PreparedEvaluationOutcome::RevertedCleanByCondition,
            origin: PreparedEvaluationOrigin::DirectPrecompute,
            memo_decision: PreparedMemoDecision::None,
            keyed: None,
        }
    }

    pub fn with_dependencies(mut self, dependencies: PreparedDependencyCapture) -> Self {
        self.dependencies = dependencies.into_sorted_unique();
        self
    }

    pub fn with_origin(mut self, origin: PreparedEvaluationOrigin) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_temporal_eligibility(
        mut self,
        temporal_eligibility: LoweredTemporalEligibility,
    ) -> Self {
        self.trace_data.temporal_eligibility = Some(temporal_eligibility);
        self
    }

    pub fn with_memo_decision(mut self, memo_decision: PreparedMemoDecision) -> Self {
        self.memo_decision = memo_decision;
        self
    }

    pub fn with_keyed(mut self, keyed: PreparedKeyedContext) -> Self {
        self.keyed = Some(keyed);
        self
    }
}
