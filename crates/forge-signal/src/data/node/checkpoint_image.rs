use serde::{Deserialize, Serialize};

use crate::data::aspect::{Aspect, AspectMask, PartitionVersionMap};
use crate::data::dependency::DependencySnapshotId;
use crate::data::graph::{DependencySetId, SubscriberSetId};
use crate::data::output::PartitionSubscription;
use crate::data::trace::{
    CausalityMetadata, ExecutionTraceStamp, RetainedDiagnosticArtifact, RuntimeArtifactState,
};

use super::{NodeEvaluationConfig, NodeState};

#[derive(Debug, Clone, PartialEq)]
pub(super) struct CheckpointNodeImageParts {
    pub(super) state: NodeState,
    pub(super) dirty_aspects: AspectMask,
    pub(super) dirty_partition_scopes: Vec<(Aspect, PartitionSubscription)>,
    pub(super) aspect_versions: PartitionVersionMap,
    pub(super) dependencies_id: DependencySetId,
    pub(super) subscribers_id: SubscriberSetId,
    pub(super) dep_snapshot_id: DependencySnapshotId,
    pub(super) tombstoned: bool,
    pub(super) runtime_artifact_state: Option<RuntimeArtifactState>,
    pub(super) retained_artifact: Option<RetainedDiagnosticArtifact>,
    pub(super) causality: Option<CausalityMetadata>,
    pub(super) execution_trace: Option<ExecutionTraceStamp>,
    pub(super) eval_config: NodeEvaluationConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckpointNodeImage {
    state: NodeState,
    dirty_aspects: AspectMask,
    #[serde(default)]
    dirty_partition_scopes: Vec<(Aspect, PartitionSubscription)>,
    aspect_versions: PartitionVersionMap,
    dependencies_id: DependencySetId,
    subscribers_id: SubscriberSetId,
    dep_snapshot_id: DependencySnapshotId,
    tombstoned: bool,
    #[serde(default)]
    runtime_artifact_state: Option<RuntimeArtifactState>,
    #[serde(default)]
    retained_artifact: Option<RetainedDiagnosticArtifact>,
    #[serde(default)]
    causality: Option<CausalityMetadata>,
    #[serde(default)]
    execution_trace: Option<ExecutionTraceStamp>,
    #[serde(default)]
    eval_config: NodeEvaluationConfig,
}

impl CheckpointNodeImage {
    pub(super) fn from_parts(parts: CheckpointNodeImageParts) -> Self {
        Self {
            state: parts.state,
            dirty_aspects: parts.dirty_aspects,
            dirty_partition_scopes: parts.dirty_partition_scopes,
            aspect_versions: parts.aspect_versions,
            dependencies_id: parts.dependencies_id,
            subscribers_id: parts.subscribers_id,
            dep_snapshot_id: parts.dep_snapshot_id,
            tombstoned: parts.tombstoned,
            runtime_artifact_state: parts.runtime_artifact_state,
            retained_artifact: parts.retained_artifact,
            causality: parts.causality,
            execution_trace: parts.execution_trace,
            eval_config: parts.eval_config,
        }
    }

    pub(super) fn into_parts(self) -> CheckpointNodeImageParts {
        CheckpointNodeImageParts {
            state: self.state,
            dirty_aspects: self.dirty_aspects,
            dirty_partition_scopes: self.dirty_partition_scopes,
            aspect_versions: self.aspect_versions,
            dependencies_id: self.dependencies_id,
            subscribers_id: self.subscribers_id,
            dep_snapshot_id: self.dep_snapshot_id,
            tombstoned: self.tombstoned,
            runtime_artifact_state: self.runtime_artifact_state,
            retained_artifact: self.retained_artifact,
            causality: self.causality,
            execution_trace: self.execution_trace,
            eval_config: self.eval_config,
        }
    }

    pub(crate) fn runtime_artifact_state(&self) -> Option<&RuntimeArtifactState> {
        self.runtime_artifact_state.as_ref()
    }

    pub(crate) fn runtime_artifact_state_mut(&mut self) -> Option<&mut RuntimeArtifactState> {
        self.runtime_artifact_state.as_mut()
    }

    pub(crate) fn retained_artifact(&self) -> Option<&RetainedDiagnosticArtifact> {
        self.retained_artifact.as_ref()
    }

    pub(crate) fn causality(&self) -> Option<&CausalityMetadata> {
        self.causality.as_ref()
    }

    pub(crate) fn set_dependencies_id(&mut self, dependencies_id: DependencySetId) {
        self.dependencies_id = dependencies_id;
    }

    pub(crate) fn set_subscribers_id(&mut self, subscribers_id: SubscriberSetId) {
        self.subscribers_id = subscribers_id;
    }

    pub(crate) fn set_dep_snapshot_id(&mut self, dep_snapshot_id: DependencySnapshotId) {
        self.dep_snapshot_id = dep_snapshot_id;
    }

    pub(crate) fn clear_dependency_handles_for_adoption(&mut self) {
        self.dependencies_id = DependencySetId::EMPTY;
        self.subscribers_id = SubscriberSetId::EMPTY;
        self.dep_snapshot_id = DependencySnapshotId::EMPTY;
    }

    pub(crate) fn set_eval_config(&mut self, eval_config: NodeEvaluationConfig) {
        self.eval_config = eval_config;
    }

    pub(crate) fn set_runtime_artifact_state(
        &mut self,
        runtime_artifact_state: Option<RuntimeArtifactState>,
    ) {
        self.runtime_artifact_state = runtime_artifact_state;
    }

    pub(crate) fn set_retained_artifact(
        &mut self,
        retained_artifact: Option<RetainedDiagnosticArtifact>,
    ) {
        self.retained_artifact = retained_artifact;
    }

    pub(crate) fn set_causality(&mut self, causality: Option<CausalityMetadata>) {
        self.causality = causality;
    }
}
