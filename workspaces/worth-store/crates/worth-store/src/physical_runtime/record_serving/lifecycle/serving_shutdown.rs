use crate::physical_runtime::MediaShutdownOutcome;

use super::super::{RecordPublicationResidueObservation, RecordServingCounterSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordServingTerminalPosture {
    NoInspectionRequired,
    InspectionRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordServingOwnerDisposition {
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordServingTerminalObservation {
    posture: RecordServingTerminalPosture,
    owner: RecordServingOwnerDisposition,
    residue: RecordPublicationResidueObservation,
    counters: RecordServingCounterSnapshot,
}

impl RecordServingTerminalObservation {
    pub(in crate::physical_runtime) fn new(
        inspection_required: bool,
        residue: RecordPublicationResidueObservation,
        counters: RecordServingCounterSnapshot,
    ) -> Self {
        assert_eq!(
            counters.owner_live(),
            0,
            "record-owner release must precede terminal observation"
        );
        Self {
            posture: if inspection_required {
                RecordServingTerminalPosture::InspectionRequired
            } else {
                RecordServingTerminalPosture::NoInspectionRequired
            },
            owner: RecordServingOwnerDisposition::Released,
            residue,
            counters,
        }
    }

    pub const fn posture(self) -> RecordServingTerminalPosture {
        self.posture
    }

    pub const fn owner(self) -> RecordServingOwnerDisposition {
        self.owner
    }

    pub const fn residue(self) -> RecordPublicationResidueObservation {
        self.residue
    }

    pub const fn counters(self) -> RecordServingCounterSnapshot {
        self.counters
    }
}

pub struct ServingShutdownOutcome<Terminal> {
    media: MediaShutdownOutcome<Terminal>,
    records: RecordServingTerminalObservation,
    residency: worth_store_buffer_pool::PhysicalResidencyShutdown,
}

impl<Terminal> ServingShutdownOutcome<Terminal> {
    pub(in crate::physical_runtime) const fn new(
        media: MediaShutdownOutcome<Terminal>,
        records: RecordServingTerminalObservation,
        residency: worth_store_buffer_pool::PhysicalResidencyShutdown,
    ) -> Self {
        Self {
            media,
            records,
            residency,
        }
    }

    pub const fn terminal(&self) -> &Terminal {
        self.media.terminal()
    }

    pub const fn media(&self) -> &MediaShutdownOutcome<Terminal> {
        &self.media
    }

    pub const fn records(&self) -> RecordServingTerminalObservation {
        self.records
    }

    pub const fn residency(&self) -> worth_store_buffer_pool::PhysicalResidencyShutdown {
        self.residency
    }
}
