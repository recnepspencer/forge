use crate::config::data::CompiledLanePolicy;
use crate::runtime::RelationalRuntime;
use crate::simulation::data::{CompiledArtifactAuthorityStatus, CompiledExecutionArtifact};

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

    pub fn compiled_artifact_authority_status(
        &self,
        artifact_id: u64,
    ) -> CompiledArtifactAuthorityStatus {
        if self.runtime.config.execution.compiled_lane_policy
            != CompiledLanePolicy::DerivedCompiledLane
        {
            return CompiledArtifactAuthorityStatus::CompiledLaneDisabled;
        }
        let Some(artifact) = self.runtime.services.compiled_artifact(artifact_id) else {
            return CompiledArtifactAuthorityStatus::MissingSourceCommit;
        };
        let Some(commit) = self
            .runtime
            .history
            .recorded_commit_envelope(artifact.source_commit_id)
        else {
            return CompiledArtifactAuthorityStatus::MissingSourceCommit;
        };
        if commit.commit.version_id != artifact.source_version_id {
            return CompiledArtifactAuthorityStatus::StaleVersion;
        }
        if self.runtime.current_version_id() != artifact.source_version_id {
            return CompiledArtifactAuthorityStatus::StaleVersion;
        }
        CompiledArtifactAuthorityStatus::Authoritative
    }
}
