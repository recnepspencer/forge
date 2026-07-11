use crate::{PageLsn, RecoveryCounterSnapshot, RecoverySourceDecisionTrace, RedoExecutionReceipt};

use super::RecoveryPhysicsCloseoutReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecoveryPhysicsStabilityAssumption {
    PersistedBytesAreReadStableForIsolationStartup,
    DirectoryAndRenameDurabilityAlreadyAdmitted,
    BackendProfileMatchesDurabilityReceipts,
    NoS5PhysicalIsolationClaim,
    NoIoQosClaim,
    NoBlobLifecycleClaim,
    NoRepairForensicsClaim,
    NoSecurityAuthenticityClaim,
    NoFullPhysicalDatabaseCertificationClaim,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5PhysicalIsolationRecoveryReadiness {
    recovered_root: String,
    admitted_page_lsn_frontier: Option<PageLsn>,
    replay_receipt: RedoExecutionReceipt,
    source_precedence_trace: RecoverySourceDecisionTrace,
    counters: RecoveryCounterSnapshot,
    stability_assumptions: Vec<RecoveryPhysicsStabilityAssumption>,
}

impl S5PhysicalIsolationRecoveryReadiness {
    pub(crate) fn from_closeout_bundle(
        report: &RecoveryPhysicsCloseoutReport,
        replay_receipt: RedoExecutionReceipt,
        source_precedence_trace: RecoverySourceDecisionTrace,
    ) -> Self {
        Self {
            recovered_root: report.recovered_root().to_string(),
            admitted_page_lsn_frontier: report.admitted_page_lsn_frontier(),
            replay_receipt,
            source_precedence_trace,
            counters: report.counters(),
            stability_assumptions: required_stability_assumptions(),
        }
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.admitted_page_lsn_frontier
    }

    pub const fn replay_receipt(&self) -> &RedoExecutionReceipt {
        &self.replay_receipt
    }

    pub const fn source_precedence_trace(&self) -> &RecoverySourceDecisionTrace {
        &self.source_precedence_trace
    }

    pub const fn counters(&self) -> RecoveryCounterSnapshot {
        self.counters
    }

    pub fn stability_assumptions(&self) -> &[RecoveryPhysicsStabilityAssumption] {
        &self.stability_assumptions
    }

    pub fn admit_for_s5_startup(
        &self,
    ) -> Result<S5RecoveryReadinessAdmission, S5RecoveryReadinessDenial> {
        S5RecoveryReadinessAdmission::admit(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5RecoveryReadinessAdmission {
    recovered_root: String,
    admitted_page_lsn_frontier: Option<PageLsn>,
    replayed_frames: usize,
    source_candidate_count: usize,
}

impl S5RecoveryReadinessAdmission {
    fn admit(
        readiness: &S5PhysicalIsolationRecoveryReadiness,
    ) -> Result<Self, S5RecoveryReadinessDenial> {
        require_replay_matches_handoff(readiness)?;
        require_all_stability_assumptions(readiness)?;
        Ok(Self {
            recovered_root: readiness.recovered_root.clone(),
            admitted_page_lsn_frontier: readiness.admitted_page_lsn_frontier,
            replayed_frames: readiness.counters.replayed_frames(),
            source_candidate_count: readiness.source_precedence_trace.candidate_count(),
        })
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.admitted_page_lsn_frontier
    }

    pub const fn replayed_frames(&self) -> usize {
        self.replayed_frames
    }

    pub const fn source_candidate_count(&self) -> usize {
        self.source_candidate_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S5RecoveryReadinessDenial {
    ReplayRootMismatch,
    ReplayPageLsnFrontierMismatch,
    ReplaySourceDecisionMismatch,
    MissingStabilityAssumption(RecoveryPhysicsStabilityAssumption),
}

fn require_replay_matches_handoff(
    readiness: &S5PhysicalIsolationRecoveryReadiness,
) -> Result<(), S5RecoveryReadinessDenial> {
    let state = readiness.replay_receipt.recovered_state();
    if state.recovered_physical_root() != readiness.recovered_root {
        return Err(S5RecoveryReadinessDenial::ReplayRootMismatch);
    }
    if state.page_lsn_frontier() != readiness.admitted_page_lsn_frontier {
        return Err(S5RecoveryReadinessDenial::ReplayPageLsnFrontierMismatch);
    }
    if state.source_replay_basis() != readiness.source_precedence_trace.replay_basis() {
        return Err(S5RecoveryReadinessDenial::ReplaySourceDecisionMismatch);
    }
    Ok(())
}

fn require_all_stability_assumptions(
    readiness: &S5PhysicalIsolationRecoveryReadiness,
) -> Result<(), S5RecoveryReadinessDenial> {
    for assumption in required_stability_assumptions() {
        if !readiness.stability_assumptions.contains(&assumption) {
            return Err(S5RecoveryReadinessDenial::MissingStabilityAssumption(
                assumption,
            ));
        }
    }
    Ok(())
}

fn required_stability_assumptions() -> Vec<RecoveryPhysicsStabilityAssumption> {
    vec![
        RecoveryPhysicsStabilityAssumption::PersistedBytesAreReadStableForIsolationStartup,
        RecoveryPhysicsStabilityAssumption::DirectoryAndRenameDurabilityAlreadyAdmitted,
        RecoveryPhysicsStabilityAssumption::BackendProfileMatchesDurabilityReceipts,
        RecoveryPhysicsStabilityAssumption::NoS5PhysicalIsolationClaim,
        RecoveryPhysicsStabilityAssumption::NoIoQosClaim,
        RecoveryPhysicsStabilityAssumption::NoBlobLifecycleClaim,
        RecoveryPhysicsStabilityAssumption::NoRepairForensicsClaim,
        RecoveryPhysicsStabilityAssumption::NoSecurityAuthenticityClaim,
        RecoveryPhysicsStabilityAssumption::NoFullPhysicalDatabaseCertificationClaim,
    ]
}
