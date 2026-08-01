pub(crate) const DEFAULT_OBSERVATION_FRAME_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedRetentionClassBudget {
    frame_limit: usize,
    structural_byte_limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedFrameRetentionBudget {
    current: UiMountedRetentionClassBudget,
    in_flight: UiMountedRetentionClassBudget,
    observation_basis: UiMountedRetentionClassBudget,
    predecessor_inspection: UiMountedRetentionClassBudget,
    diagnostic: UiMountedRetentionClassBudget,
    visual_snapshot: UiMountedRetentionClassBudget,
    visual_overlay: UiMountedRetentionClassBudget,
    expired_identity_limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedFrameRetentionBudgetInput {
    pub current: UiMountedRetentionClassBudget,
    pub in_flight: UiMountedRetentionClassBudget,
    pub observation_basis: UiMountedRetentionClassBudget,
    pub predecessor_inspection: UiMountedRetentionClassBudget,
    pub diagnostic: UiMountedRetentionClassBudget,
    pub visual_snapshot: UiMountedRetentionClassBudget,
    pub visual_overlay: UiMountedRetentionClassBudget,
    pub expired_identity_limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedRetentionClass {
    Current,
    InFlight,
    ObservationBasis,
    PredecessorInspection,
    Diagnostic,
    Quarantine,
    VisualSnapshot,
    VisualOverlay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedFrameRetentionDenial {
    CapacityExceeded {
        class: UiMountedRetentionClass,
        required_frames: usize,
        required_structural_bytes: usize,
        budget: UiMountedRetentionClassBudget,
    },
    AccountingOverflow {
        class: UiMountedRetentionClass,
    },
}

impl UiMountedRetentionClassBudget {
    pub const fn new(frame_limit: usize, structural_byte_limit: usize) -> Self {
        Self {
            frame_limit,
            structural_byte_limit,
        }
    }

    pub const fn frame_limit(self) -> usize {
        self.frame_limit
    }

    pub const fn structural_byte_limit(self) -> usize {
        self.structural_byte_limit
    }

    pub(crate) fn admits(self, frames: usize, structural_bytes: usize) -> bool {
        frames <= self.frame_limit && structural_bytes <= self.structural_byte_limit
    }
}

impl UiMountedFrameRetentionBudget {
    pub const fn new(input: UiMountedFrameRetentionBudgetInput) -> Self {
        Self {
            current: input.current,
            in_flight: input.in_flight,
            observation_basis: input.observation_basis,
            predecessor_inspection: input.predecessor_inspection,
            diagnostic: input.diagnostic,
            visual_snapshot: input.visual_snapshot,
            visual_overlay: input.visual_overlay,
            expired_identity_limit: input.expired_identity_limit,
        }
    }

    pub const fn current(self) -> UiMountedRetentionClassBudget {
        self.current
    }

    pub const fn in_flight(self) -> UiMountedRetentionClassBudget {
        self.in_flight
    }

    pub const fn observation_basis(self) -> UiMountedRetentionClassBudget {
        self.observation_basis
    }

    pub const fn predecessor_inspection(self) -> UiMountedRetentionClassBudget {
        self.predecessor_inspection
    }

    pub const fn diagnostic(self) -> UiMountedRetentionClassBudget {
        self.diagnostic
    }

    pub const fn visual_snapshot(self) -> UiMountedRetentionClassBudget {
        self.visual_snapshot
    }

    pub const fn visual_overlay(self) -> UiMountedRetentionClassBudget {
        self.visual_overlay
    }

    pub const fn expired_identity_limit(self) -> usize {
        self.expired_identity_limit
    }
}

impl Default for UiMountedFrameRetentionBudget {
    fn default() -> Self {
        const MIB: usize = 1024 * 1024;
        Self::new(UiMountedFrameRetentionBudgetInput {
            current: UiMountedRetentionClassBudget::new(1, 64 * MIB),
            in_flight: UiMountedRetentionClassBudget::new(1, 64 * MIB),
            observation_basis: UiMountedRetentionClassBudget::new(
                DEFAULT_OBSERVATION_FRAME_CAPACITY,
                256 * MIB,
            ),
            predecessor_inspection: UiMountedRetentionClassBudget::new(
                DEFAULT_OBSERVATION_FRAME_CAPACITY,
                256 * MIB,
            ),
            diagnostic: UiMountedRetentionClassBudget::new(8, 16 * MIB),
            visual_snapshot: UiMountedRetentionClassBudget::new(4, 128 * MIB),
            visual_overlay: UiMountedRetentionClassBudget::new(4, 32 * MIB),
            expired_identity_limit: 64,
        })
    }
}

impl UiMountedFrameRetentionDenial {
    pub fn class(self) -> UiMountedRetentionClass {
        match self {
            Self::CapacityExceeded { class, .. } | Self::AccountingOverflow { class } => class,
        }
    }
}
