use worth_store_physical_backend::OwnershipReleaseOutcome;

use super::{
    phase::{PhysicalStoreClosePhase, CLOSE_PHASES},
    progress::PhysicalStoreCloseProgressOwner,
};
use crate::physical_runtime::{
    instance::PhysicalStoreInstanceParts,
    record_serving::{RecordServingTerminalPosture, ServingShutdownOutcome},
    AbortedRuntime, ClosedRuntime, PhysicalSignalShutdownOutcome,
};

pub enum PhysicalStoreCloseOutcome {
    Closed {
        shutdown: ServingShutdownOutcome<ClosedRuntime>,
        phases: [PhysicalStoreClosePhase; 7],
    },
    InspectionRequired {
        shutdown: ServingShutdownOutcome<ClosedRuntime>,
        phases: [PhysicalStoreClosePhase; 7],
    },
}

pub struct PhysicalStoreAbortOutcome {
    shutdown: ServingShutdownOutcome<AbortedRuntime>,
    phases: [PhysicalStoreClosePhase; 7],
}

impl PhysicalStoreCloseOutcome {
    pub(super) fn from_shutdown(shutdown: ServingShutdownOutcome<ClosedRuntime>) -> Self {
        if shutdown_requires_inspection(&shutdown) {
            Self::InspectionRequired {
                shutdown,
                phases: CLOSE_PHASES,
            }
        } else {
            Self::Closed {
                shutdown,
                phases: CLOSE_PHASES,
            }
        }
    }

    pub const fn phases(&self) -> &[PhysicalStoreClosePhase; 7] {
        match self {
            Self::Closed { phases, .. } | Self::InspectionRequired { phases, .. } => phases,
        }
    }

    pub const fn shutdown(&self) -> &ServingShutdownOutcome<ClosedRuntime> {
        match self {
            Self::Closed { shutdown, .. } | Self::InspectionRequired { shutdown, .. } => shutdown,
        }
    }

    pub const fn requires_inspection(&self) -> bool {
        matches!(self, Self::InspectionRequired { .. })
    }

    pub fn into_shutdown(self) -> ServingShutdownOutcome<ClosedRuntime> {
        match self {
            Self::Closed { shutdown, .. } | Self::InspectionRequired { shutdown, .. } => shutdown,
        }
    }
}

impl PhysicalStoreAbortOutcome {
    pub(in crate::physical_runtime) fn execute(parts: PhysicalStoreInstanceParts) -> Self {
        let progress = PhysicalStoreCloseProgressOwner::new();
        Self {
            shutdown: parts.abort(progress),
            phases: CLOSE_PHASES,
        }
    }

    pub const fn phases(&self) -> &[PhysicalStoreClosePhase; 7] {
        &self.phases
    }

    pub const fn shutdown(&self) -> &ServingShutdownOutcome<AbortedRuntime> {
        &self.shutdown
    }

    pub fn requires_inspection(&self) -> bool {
        shutdown_requires_inspection(&self.shutdown)
    }

    pub fn into_shutdown(self) -> ServingShutdownOutcome<AbortedRuntime> {
        self.shutdown
    }
}

fn shutdown_requires_inspection<Terminal>(shutdown: &ServingShutdownOutcome<Terminal>) -> bool {
    shutdown.records().posture() == RecordServingTerminalPosture::InspectionRequired
        || shutdown.checkpoint().requires_inspection()
        || shutdown.residency().requires_inspection()
        || shutdown.work().drain().requires_inspection()
        || shutdown.signal() != PhysicalSignalShutdownOutcome::Disposed
        || shutdown.signal_cancellation_failures() != 0
        || shutdown
            .signal_summary()
            .is_none_or(|summary| summary.active_in_flight_node_count() != 0)
        || shutdown.media().release() != OwnershipReleaseOutcome::Released
}
