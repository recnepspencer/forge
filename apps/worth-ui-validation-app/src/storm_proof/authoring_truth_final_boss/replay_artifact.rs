#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationAuthoringTruthFinalBossReplayArtifact {
    authored_delta_digest: u64,
    runtime_change_digest: u64,
    compile_boundary_digest: u64,
    visible_result_digest: u64,
    projection_digest: u64,
    final_active_artifact_digest: u64,
    final_active_plan_digest: u64,
    final_capability_snapshot_digest: u64,
    final_authoring_snapshot_digest: Option<u64>,
    final_last_valid_artifact_digest: u64,
    final_last_valid_plan_digest: u64,
}

impl ValidationAuthoringTruthFinalBossReplayArtifact {
    pub fn new(
        authored_delta_digest: u64,
        runtime_change_digest: u64,
        compile_boundary_digest: u64,
        visible_result_digest: u64,
        projection_digest: u64,
        final_active_artifact_digest: u64,
        final_active_plan_digest: u64,
        final_capability_snapshot_digest: u64,
        final_authoring_snapshot_digest: Option<u64>,
        final_last_valid_artifact_digest: u64,
        final_last_valid_plan_digest: u64,
    ) -> Self {
        Self {
            authored_delta_digest,
            runtime_change_digest,
            compile_boundary_digest,
            visible_result_digest,
            projection_digest,
            final_active_artifact_digest,
            final_active_plan_digest,
            final_capability_snapshot_digest,
            final_authoring_snapshot_digest,
            final_last_valid_artifact_digest,
            final_last_valid_plan_digest,
        }
    }

    pub fn certify_replay(first: Self, second: Self) -> bool {
        first == second
    }

    pub fn runtime_change_digest(self) -> u64 {
        self.runtime_change_digest
    }
}
