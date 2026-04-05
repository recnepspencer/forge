use crate::config::data::CompiledLanePolicy;
use crate::logic::runtime::RelationalRuntime;
use crate::simulation::data::{CompiledArtifactCompatibility, CompiledExecutionArtifact};

pub struct SimulationAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn simulation_access(&self) -> SimulationAccess<'_> {
        SimulationAccess::new(self)
    }
}

impl<'runtime> SimulationAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn compiled_artifact(&self, artifact_id: u64) -> Option<&CompiledExecutionArtifact> {
        self.runtime.services.compiled_artifact(artifact_id)
    }

    pub fn compiled_artifact_compatibility(
        &self,
        artifact_id: u64,
    ) -> CompiledArtifactCompatibility {
        if self.runtime.config.execution.compiled_lane_policy
            != CompiledLanePolicy::DerivedCompiledLane
        {
            return CompiledArtifactCompatibility::CompiledLaneDisabled;
        }
        let Some(artifact) = self.runtime.services.compiled_artifact(artifact_id) else {
            return CompiledArtifactCompatibility::MissingSourceCommit;
        };
        let Some(commit) = self
            .runtime
            .history
            .commit_envelopes
            .get(&artifact.source_commit_id)
        else {
            return CompiledArtifactCompatibility::MissingSourceCommit;
        };
        if commit.commit.version_id != artifact.source_version_id {
            return CompiledArtifactCompatibility::StaleVersion;
        }
        if self.runtime.current_version_id() != artifact.source_version_id {
            return CompiledArtifactCompatibility::StaleVersion;
        }
        CompiledArtifactCompatibility::Compatible
    }
}
