use worth_store_physical_integrity::{QuarantineHandoffPosture, QuarantineRecord};
use worth_store_recovery_physics::{
    RecoveryLayoutReadmissionAdmissionDenial, RecoveryLayoutReadmissionOutcomeView,
};

use super::QuarantineReadmissionState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineRecordObservation {
    receipt_digest: String,
    handoff: QuarantineHandoffPosture,
    proves_repair: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuarantineReadmissionOutcomeObservation {
    states: [QuarantineReadmissionState; 2],
}

pub fn map_quarantine_record(record: &QuarantineRecord) -> QuarantineRecordObservation {
    QuarantineRecordObservation {
        receipt_digest: record
            .receipt()
            .foundational_basis()
            .digest()
            .as_str()
            .to_owned(),
        handoff: record.handoff_posture(),
        proves_repair: record.proves_repair(),
    }
}

impl QuarantineRecordObservation {
    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub const fn handoff(&self) -> QuarantineHandoffPosture {
        self.handoff
    }

    pub const fn proves_repair(&self) -> bool {
        self.proves_repair
    }

    pub fn states(&self) -> impl Iterator<Item = QuarantineReadmissionState> {
        [
            QuarantineReadmissionState::Proposed,
            QuarantineReadmissionState::Sealed,
        ]
        .into_iter()
    }
}

pub const fn map_quarantine_readmission_outcome(
    outcome: RecoveryLayoutReadmissionOutcomeView<'_>,
) -> QuarantineReadmissionOutcomeObservation {
    let final_state = match outcome {
        RecoveryLayoutReadmissionOutcomeView::Readmitted(_) => {
            QuarantineReadmissionState::Readmitted
        }
        RecoveryLayoutReadmissionOutcomeView::Denied(
            RecoveryLayoutReadmissionAdmissionDenial::NoForegroundAuthority,
        ) => QuarantineReadmissionState::RetainedForAudit,
        RecoveryLayoutReadmissionOutcomeView::Denied(_) => QuarantineReadmissionState::Denied,
    };
    QuarantineReadmissionOutcomeObservation {
        states: [
            QuarantineReadmissionState::RecoveryVerificationPending,
            final_state,
        ],
    }
}

impl QuarantineReadmissionOutcomeObservation {
    pub fn states(&self) -> impl Iterator<Item = QuarantineReadmissionState> + '_ {
        self.states.iter().copied()
    }
}
