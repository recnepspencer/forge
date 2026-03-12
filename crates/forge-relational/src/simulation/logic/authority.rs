use serde_json::json;

use crate::config::data::CompiledLanePolicy;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::CommitId;
use crate::identity::data::PartitionId;
use crate::logic::runtime::RelationalRuntime;
use crate::simulation::data::{
    CompiledArtifactCompatibility, CompiledArtifactError, CompiledExecutionArtifact,
    TopologyFreezeMode,
};

pub struct SimulationAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn simulation_authority(&mut self) -> SimulationAuthority<'_> {
        SimulationAuthority::new(self)
    }
}

impl<'runtime> SimulationAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn compile_execution_artifact(
        &mut self,
        commit_id: CommitId,
        mut partition_ids: Vec<PartitionId>,
    ) -> Result<CompiledExecutionArtifact, CompiledArtifactError> {
        if self.runtime.config.execution.compiled_lane_policy
            != CompiledLanePolicy::DerivedCompiledLane
        {
            return Err(CompiledArtifactError {
                compatibility: CompiledArtifactCompatibility::CompiledLaneDisabled,
                detail: "compiled execution lane is disabled for this profile".to_string(),
            });
        }
        let Some(envelope) = self.runtime.history.commit_envelopes.get(&commit_id).cloned() else {
            return Err(CompiledArtifactError {
                compatibility: CompiledArtifactCompatibility::MissingSourceCommit,
                detail: format!("missing source commit {}", commit_id.0),
            });
        };
        partition_ids.sort();
        partition_ids.dedup();
        let compiled_record_count = envelope.patch.records.len();
        let artifact = CompiledExecutionArtifact {
            artifact_id: self.runtime.services.next_compiled_artifact_id(),
            source_commit_id: commit_id,
            source_version_id: envelope.commit.version_id,
            source_branch_id: envelope.branch_context.clone(),
            partition_ids,
            topology_freeze_mode: TopologyFreezeMode::FreezeAtCommit,
            compiled_record_count,
        };
        self.runtime.services.store_compiled_artifact(artifact.clone());
        self.runtime.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "compiled execution artifact created".to_string(),
                fields: json!({
                    "artifact_id": artifact.artifact_id,
                    "source_commit_id": artifact.source_commit_id.0,
                    "source_version_id": artifact.source_version_id.0,
                    "partition_ids": artifact.partition_ids.iter().map(|id| id.0).collect::<Vec<_>>(),
                    "compiled_record_count": artifact.compiled_record_count,
                }),
            }],
        );
        Ok(artifact)
    }
}
