#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCandidateAdmissionCounters {
    candidate_proof_checks: usize,
    snapshot_compatibility_checks: usize,
    runtime_posture_checks: usize,
    artifact_comparisons: usize,
    plan_lowering_attempts: usize,
}

impl WorthUiCandidateAdmissionCounters {
    pub(crate) fn record_candidate_proof_check(&mut self) {
        self.candidate_proof_checks += 1;
    }

    pub(crate) fn record_snapshot_compatibility_check(&mut self) {
        self.snapshot_compatibility_checks += 1;
    }

    pub(crate) fn record_runtime_posture_check(&mut self) {
        self.runtime_posture_checks += 1;
    }

    pub fn candidate_proof_checks(self) -> usize {
        self.candidate_proof_checks
    }

    pub fn snapshot_compatibility_checks(self) -> usize {
        self.snapshot_compatibility_checks
    }

    pub fn runtime_posture_checks(self) -> usize {
        self.runtime_posture_checks
    }

    pub fn artifact_comparisons(self) -> usize {
        self.artifact_comparisons
    }

    pub fn plan_lowering_attempts(self) -> usize {
        self.plan_lowering_attempts
    }
}
