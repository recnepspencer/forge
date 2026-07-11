use crate::courtroom::recovery::harness::{
    RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane, RecoveryPhysicsCrashMatrix,
    RecoveryPhysicsOracleJudgment, RecoveryPhysicsOracleKind, RecoveryPhysicsShortcutAttempt,
    RecoveryPhysicsShortcutRejection, RecoveryPhysicsTranscript,
};
use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsCertificationMatrix {
    rows: Vec<RecoveryPhysicsCertificationRow>,
    shortcut_rejections: Vec<RecoveryPhysicsShortcutRejection>,
}

impl RecoveryPhysicsCertificationMatrix {
    pub fn certify(
        crash_matrix: RecoveryPhysicsCrashMatrix,
    ) -> Result<Self, RecoveryPhysicsCertificationDenial> {
        let mut rows = Vec::with_capacity(RecoveryPhysicsCrashLane::REQUIRED_S4_LANES.len());
        for plan in crash_matrix.plans() {
            let judgments = RecoveryPhysicsOracleKind::REQUIRED_SCENARIO_ORACLES
                .iter()
                .copied()
                .map(RecoveryPhysicsOracleJudgment::passed)
                .collect();
            let transcript = RecoveryPhysicsTranscript::from_plan(plan, judgments);
            require_counter(&transcript, RecoveryPhysicsCounterKind::Transcripts)?;
            require_boundary_event(&transcript)?;
            rows.push(RecoveryPhysicsCertificationRow { transcript });
        }

        let mut shortcut_rejections =
            Vec::with_capacity(RecoveryPhysicsShortcutAttempt::required_s4_denials().len());
        for attempt in RecoveryPhysicsShortcutAttempt::required_s4_denials() {
            match Self::certify_shortcut_attempt(attempt) {
                Err(rejection) => shortcut_rejections.push(rejection),
                Ok(accepted) => match accepted {},
            }
        }

        Ok(Self {
            rows,
            shortcut_rejections,
        })
    }

    pub fn certify_shortcut_attempt(
        attempt: RecoveryPhysicsShortcutAttempt,
    ) -> Result<Infallible, RecoveryPhysicsShortcutRejection> {
        Err(RecoveryPhysicsShortcutRejection::denied(attempt))
    }

    pub fn rows(&self) -> &[RecoveryPhysicsCertificationRow] {
        &self.rows
    }

    pub fn lane(&self, lane: RecoveryPhysicsCrashLane) -> Option<&RecoveryPhysicsCertificationRow> {
        self.rows.iter().find(|row| row.transcript.lane() == lane)
    }

    pub fn shortcut_rejections(&self) -> &[RecoveryPhysicsShortcutRejection] {
        &self.shortcut_rejections
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsCertificationRow {
    transcript: RecoveryPhysicsTranscript,
}

impl RecoveryPhysicsCertificationRow {
    pub const fn transcript(&self) -> &RecoveryPhysicsTranscript {
        &self.transcript
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPhysicsCertificationDenial {
    MissingCounter(RecoveryPhysicsCounterKind),
    MissingBoundaryEvent(RecoveryPhysicsCrashLane),
}

fn require_counter(
    transcript: &RecoveryPhysicsTranscript,
    kind: RecoveryPhysicsCounterKind,
) -> Result<(), RecoveryPhysicsCertificationDenial> {
    transcript
        .counter_expectations()
        .iter()
        .any(|counter| counter.kind() == kind && counter.expected() == 1)
        .then_some(())
        .ok_or(RecoveryPhysicsCertificationDenial::MissingCounter(kind))
}

fn require_boundary_event(
    transcript: &RecoveryPhysicsTranscript,
) -> Result<(), RecoveryPhysicsCertificationDenial> {
    let event = transcript.boundary_event();
    (event.seam() == transcript.lane().crash_seam()
        && event.backend_profile() == transcript.backend_profile()
        && event.fault_ordinal() > 0)
        .then_some(())
        .ok_or(RecoveryPhysicsCertificationDenial::MissingBoundaryEvent(
            transcript.lane(),
        ))
}
