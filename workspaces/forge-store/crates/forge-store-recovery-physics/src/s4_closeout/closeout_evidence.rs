use crate::{
    BoundedRecoveryReceipt, FoundationalRecoveryEvidenceBundle, RecoverySourceDecisionTrace,
};

use super::{
    CrashSeamRecoveryObservation, RecoveryBoundednessEvidence, S4CrashFaultSchedulerEvidence,
    S4RecoveryCrashSeam, SyntheticRecoveryShortcutEvidence,
    SyntheticRecoveryShortcutRejectionReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPhysicsCloseoutDenial {
    MissingSuiteLane,
    MissingCrashSeam,
    NondeterministicCrashRecovery,
    MissingSyntheticShortcutRejection,
    MissingBoundedRecoveryCounters,
    UnboundedRecoveryPlan,
    SourceTraceDoesNotMatchRecoveredState,
    FoundationalEvidenceDoesNotMatchRecovery,
    BoundednessAuthorityMismatch,
    FreshRuntimeCrashEvidenceMismatch,
    MissingCrashFaultSchedulerEvidence,
    SameProcessCrashObservation,
    UnsupportedSyntheticShortcutEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsCloseoutEvidence {
    receipt: BoundedRecoveryReceipt,
    source_trace: RecoverySourceDecisionTrace,
    foundational_evidence: FoundationalRecoveryEvidenceBundle,
    boundedness: RecoveryBoundednessEvidence,
    crash_observations: Vec<CrashSeamRecoveryObservation>,
    shortcut_rejections: SyntheticRecoveryShortcutRejectionReport,
}

impl RecoveryPhysicsCloseoutEvidence {
    pub(crate) fn from_collector(
        receipt: BoundedRecoveryReceipt,
        source_trace: RecoverySourceDecisionTrace,
        foundational_evidence: FoundationalRecoveryEvidenceBundle,
        boundedness: RecoveryBoundednessEvidence,
        crash_observations: Vec<CrashSeamRecoveryObservation>,
        shortcut_rejections: SyntheticRecoveryShortcutRejectionReport,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        let evidence = Self {
            receipt,
            source_trace,
            foundational_evidence,
            boundedness,
            crash_observations,
            shortcut_rejections,
        };
        evidence.require_source_trace_matches_recovered_state()?;
        evidence.require_foundational_evidence_matches_recovered_state()?;
        evidence.require_boundedness_matches_receipt()?;
        Ok(evidence)
    }

    pub const fn receipt(&self) -> &BoundedRecoveryReceipt {
        &self.receipt
    }

    pub const fn source_trace(&self) -> &RecoverySourceDecisionTrace {
        &self.source_trace
    }

    pub const fn foundational_evidence(&self) -> &FoundationalRecoveryEvidenceBundle {
        &self.foundational_evidence
    }

    pub const fn boundedness(&self) -> RecoveryBoundednessEvidence {
        self.boundedness
    }

    pub fn crash_observations(&self) -> &[CrashSeamRecoveryObservation] {
        &self.crash_observations
    }

    pub const fn shortcut_rejections(&self) -> &SyntheticRecoveryShortcutRejectionReport {
        &self.shortcut_rejections
    }

    fn require_source_trace_matches_recovered_state(
        &self,
    ) -> Result<(), RecoveryPhysicsCloseoutDenial> {
        let expected = format!(
            "{:?}:{}:{}",
            self.source_trace.kind(),
            self.source_trace.profile(),
            self.source_trace.candidate_count()
        );
        if expected
            == self
                .receipt
                .execution()
                .recovered_state()
                .source_decision_digest()
        {
            return Ok(());
        }
        Err(RecoveryPhysicsCloseoutDenial::SourceTraceDoesNotMatchRecoveredState)
    }

    fn require_foundational_evidence_matches_recovered_state(
        &self,
    ) -> Result<(), RecoveryPhysicsCloseoutDenial> {
        let state = self.receipt.execution().recovered_state();
        if self
            .foundational_evidence
            .receipt()
            .recovered_physical_root()
            == state.recovered_physical_root()
            && self
                .foundational_evidence
                .performance()
                .exact_counter_assertions()
                > 0
        {
            return Ok(());
        }
        Err(RecoveryPhysicsCloseoutDenial::FoundationalEvidenceDoesNotMatchRecovery)
    }

    fn require_boundedness_matches_receipt(&self) -> Result<(), RecoveryPhysicsCloseoutDenial> {
        if self.boundedness.counters() == self.receipt.counters() {
            return Ok(());
        }
        Err(RecoveryPhysicsCloseoutDenial::UnboundedRecoveryPlan)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsCloseoutCollector {
    receipt: BoundedRecoveryReceipt,
    source_trace: RecoverySourceDecisionTrace,
    foundational_evidence: FoundationalRecoveryEvidenceBundle,
    boundedness: RecoveryBoundednessEvidence,
    crash_observations: Vec<CrashSeamRecoveryObservation>,
    shortcut_evidence: Vec<SyntheticRecoveryShortcutEvidence>,
}

impl RecoveryPhysicsCloseoutCollector {
    pub fn from_executed_recovery(
        receipt: BoundedRecoveryReceipt,
        source_trace: RecoverySourceDecisionTrace,
        foundational_evidence: FoundationalRecoveryEvidenceBundle,
        boundedness: RecoveryBoundednessEvidence,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        let collector = Self {
            receipt,
            source_trace,
            foundational_evidence,
            boundedness,
            crash_observations: Vec::new(),
            shortcut_evidence: Vec::new(),
        };
        collector.require_source_trace_matches_recovered_state()?;
        collector.require_foundational_evidence_matches_recovered_state()?;
        collector.require_boundedness_matches_receipt()?;
        Ok(collector)
    }

    pub fn record_crash_recovery(
        mut self,
        evidence: S4CrashFaultSchedulerEvidence,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        self.crash_observations
            .push(CrashSeamRecoveryObservation::from_fault_scheduler_evidence(
                evidence,
            ));
        Ok(self)
    }

    pub fn record_synthetic_shortcut_denial(
        mut self,
        evidence: SyntheticRecoveryShortcutEvidence,
    ) -> Self {
        self.shortcut_evidence.push(evidence);
        self
    }

    pub fn finish(self) -> Result<RecoveryPhysicsCloseoutEvidence, RecoveryPhysicsCloseoutDenial> {
        let shortcut_rejections =
            SyntheticRecoveryShortcutRejectionReport::from_denial_evidence(self.shortcut_evidence)?;
        RecoveryPhysicsCloseoutEvidence::from_collector(
            self.receipt,
            self.source_trace,
            self.foundational_evidence,
            self.boundedness,
            self.crash_observations,
            shortcut_rejections,
        )
    }

    fn require_source_trace_matches_recovered_state(
        &self,
    ) -> Result<(), RecoveryPhysicsCloseoutDenial> {
        let expected = format!(
            "{:?}:{}:{}",
            self.source_trace.kind(),
            self.source_trace.profile(),
            self.source_trace.candidate_count()
        );
        if expected
            == self
                .receipt
                .execution()
                .recovered_state()
                .source_decision_digest()
        {
            return Ok(());
        }
        Err(RecoveryPhysicsCloseoutDenial::SourceTraceDoesNotMatchRecoveredState)
    }

    fn require_foundational_evidence_matches_recovered_state(
        &self,
    ) -> Result<(), RecoveryPhysicsCloseoutDenial> {
        let state = self.receipt.execution().recovered_state();
        if self
            .foundational_evidence
            .receipt()
            .recovered_physical_root()
            == state.recovered_physical_root()
            && self
                .foundational_evidence
                .performance()
                .exact_counter_assertions()
                > 0
        {
            return Ok(());
        }
        Err(RecoveryPhysicsCloseoutDenial::FoundationalEvidenceDoesNotMatchRecovery)
    }

    fn require_boundedness_matches_receipt(&self) -> Result<(), RecoveryPhysicsCloseoutDenial> {
        if self.boundedness.counters() == self.receipt.counters() {
            return Ok(());
        }
        Err(RecoveryPhysicsCloseoutDenial::UnboundedRecoveryPlan)
    }
}

pub(crate) const REQUIRED_CRASH_SEAMS: [S4RecoveryCrashSeam; 8] = [
    S4RecoveryCrashSeam::WalAppend,
    S4RecoveryCrashSeam::PageFlush,
    S4RecoveryCrashSeam::CheckpointManifestWrite,
    S4RecoveryCrashSeam::CheckpointCutover,
    S4RecoveryCrashSeam::CompactionCutover,
    S4RecoveryCrashSeam::Acknowledgment,
    S4RecoveryCrashSeam::DirectorySync,
    S4RecoveryCrashSeam::RenameDurability,
];
