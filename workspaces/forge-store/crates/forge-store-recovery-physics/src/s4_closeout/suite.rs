use super::{
    closeout_evidence::REQUIRED_CRASH_SEAMS, RecoveryPhysicsCertificationBundle,
    RecoveryPhysicsCloseoutDenial, RecoveryPhysicsCloseoutEvidence,
    RecoveryPhysicsCloseoutSuiteRequirement, S4CrashHarnessTranscriptSource,
    S4LoweredCrashHarnessEvidence,
};
use crate::s4_closeout::suite_lane::REQUIRED_S4_CLOSEOUT_LANES;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalCheckpointLsnRecoveryPhysicsSuite {
    requirements: Vec<RecoveryPhysicsCloseoutSuiteRequirement>,
}

impl WalCheckpointLsnRecoveryPhysicsSuite {
    pub fn from_required_s4_lanes() -> Self {
        Self {
            requirements: REQUIRED_S4_CLOSEOUT_LANES
                .iter()
                .copied()
                .map(RecoveryPhysicsCloseoutSuiteRequirement::complete)
                .collect(),
        }
    }

    pub fn certify(
        &self,
        evidence: RecoveryPhysicsCloseoutEvidence,
    ) -> Result<RecoveryPhysicsCertificationBundle, RecoveryPhysicsCloseoutDenial> {
        self.require_complete_suite_matrix()?;
        require_required_crash_seams(&evidence)?;
        require_deterministic_crash_recovery(&evidence)?;
        require_required_shortcut_rejections(&evidence)?;
        let work_bound = evidence.boundedness().work_bound();
        Ok(RecoveryPhysicsCertificationBundle::new(
            evidence,
            work_bound,
            self.requirements.clone(),
            REQUIRED_CRASH_SEAMS.to_vec(),
        ))
    }

    pub fn requirements(&self) -> &[RecoveryPhysicsCloseoutSuiteRequirement] {
        &self.requirements
    }

    pub fn admit_lowered_crash_harness_transcript(
        &self,
        source: S4CrashHarnessTranscriptSource,
    ) -> Result<S4LoweredCrashHarnessEvidence, RecoveryPhysicsCloseoutDenial> {
        self.require_complete_suite_matrix()?;
        S4LoweredCrashHarnessEvidence::from_recovery_harness_transcript(source)
    }

    fn require_complete_suite_matrix(&self) -> Result<(), RecoveryPhysicsCloseoutDenial> {
        if self.requirements.len() != REQUIRED_S4_CLOSEOUT_LANES.len() {
            return Err(RecoveryPhysicsCloseoutDenial::MissingSuiteLane);
        }
        if self.requirements.iter().all(|requirement| {
            requirement.is_complete() && REQUIRED_S4_CLOSEOUT_LANES.contains(&requirement.lane())
        }) {
            return Ok(());
        }
        Err(RecoveryPhysicsCloseoutDenial::MissingSuiteLane)
    }
}

fn require_required_crash_seams(
    evidence: &RecoveryPhysicsCloseoutEvidence,
) -> Result<(), RecoveryPhysicsCloseoutDenial> {
    if REQUIRED_CRASH_SEAMS.iter().all(|seam| {
        evidence
            .crash_observations()
            .iter()
            .any(|row| row.seam() == *seam)
    }) {
        return Ok(());
    }
    Err(RecoveryPhysicsCloseoutDenial::MissingCrashSeam)
}

fn require_deterministic_crash_recovery(
    evidence: &RecoveryPhysicsCloseoutEvidence,
) -> Result<(), RecoveryPhysicsCloseoutDenial> {
    let state = evidence.receipt().execution().recovered_state();
    if evidence.crash_observations().iter().all(|row| {
        row.recovered_root() == state.recovered_physical_root()
            && row.page_lsn_frontier() == state.page_lsn_frontier()
            && row.source_decision_digest() == state.source_decision_digest()
            && row.counters() == evidence.receipt().counters()
    }) {
        return Ok(());
    }
    Err(RecoveryPhysicsCloseoutDenial::NondeterministicCrashRecovery)
}

fn require_required_shortcut_rejections(
    evidence: &RecoveryPhysicsCloseoutEvidence,
) -> Result<(), RecoveryPhysicsCloseoutDenial> {
    if evidence
        .shortcut_rejections()
        .all_required_shortcuts_denied()
    {
        return Ok(());
    }
    Err(RecoveryPhysicsCloseoutDenial::MissingSyntheticShortcutRejection)
}
