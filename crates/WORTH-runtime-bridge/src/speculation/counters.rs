#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeSpeculationCounters {
    preview_session_count_touched: usize,
    branch_binding_proof_width: usize,
    admissibility_proof_width: usize,
    preview_artifact_count: usize,
    discard_artifact_count: usize,
    destroyed_artifact_count: usize,
    retained_non_authoritative_artifact_count: usize,
    promotion_proof_checks: usize,
    replay_bundle_width: usize,
}

impl BridgeSpeculationCounters {
    pub fn for_preview_execution(
        preview_artifact_count: usize,
        destroyed_artifact_count: usize,
        retained_non_authoritative_artifact_count: usize,
    ) -> Self {
        Self {
            preview_session_count_touched: 1,
            branch_binding_proof_width: 2,
            admissibility_proof_width: 0,
            preview_artifact_count,
            discard_artifact_count: destroyed_artifact_count,
            destroyed_artifact_count,
            retained_non_authoritative_artifact_count,
            promotion_proof_checks: 0,
            replay_bundle_width: 0,
        }
    }

    pub fn for_discard(
        discard_artifact_count: usize,
        destroyed_artifact_count: usize,
        retained_non_authoritative_artifact_count: usize,
    ) -> Self {
        Self {
            preview_session_count_touched: 1,
            branch_binding_proof_width: 2,
            admissibility_proof_width: 0,
            preview_artifact_count: 0,
            discard_artifact_count,
            destroyed_artifact_count,
            retained_non_authoritative_artifact_count,
            promotion_proof_checks: 0,
            replay_bundle_width: retained_non_authoritative_artifact_count,
        }
    }

    pub fn for_promotion(
        admissibility_proof_width: usize,
        promotion_proof_checks: usize,
        replay_bundle_width: usize,
    ) -> Self {
        Self {
            preview_session_count_touched: 1,
            branch_binding_proof_width: 2,
            admissibility_proof_width,
            preview_artifact_count: 0,
            discard_artifact_count: 0,
            destroyed_artifact_count: 0,
            retained_non_authoritative_artifact_count: 0,
            promotion_proof_checks,
            replay_bundle_width,
        }
    }

    pub fn for_replay(preview_session_count_touched: usize, replay_bundle_width: usize) -> Self {
        Self {
            preview_session_count_touched,
            branch_binding_proof_width: 0,
            admissibility_proof_width: 0,
            preview_artifact_count: 0,
            discard_artifact_count: 0,
            destroyed_artifact_count: 0,
            retained_non_authoritative_artifact_count: 0,
            promotion_proof_checks: 0,
            replay_bundle_width,
        }
    }

    pub fn preview_session_count_touched(&self) -> usize {
        self.preview_session_count_touched
    }

    pub fn branch_binding_proof_width(&self) -> usize {
        self.branch_binding_proof_width
    }

    pub fn admissibility_proof_width(&self) -> usize {
        self.admissibility_proof_width
    }

    pub fn preview_artifact_count(&self) -> usize {
        self.preview_artifact_count
    }

    pub fn discard_artifact_count(&self) -> usize {
        self.discard_artifact_count
    }

    pub fn destroyed_artifact_count(&self) -> usize {
        self.destroyed_artifact_count
    }

    pub fn retained_non_authoritative_artifact_count(&self) -> usize {
        self.retained_non_authoritative_artifact_count
    }

    pub fn promotion_proof_checks(&self) -> usize {
        self.promotion_proof_checks
    }

    pub fn replay_bundle_width(&self) -> usize {
        self.replay_bundle_width
    }
}
