#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedRetentionClassBudget {
    frame_limit: usize,
    structural_byte_limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedFrameRetentionBudget {
    current: UiMountedRetentionClassBudget,
    in_flight: UiMountedRetentionClassBudget,
    predecessor_inspection: UiMountedRetentionClassBudget,
    expired_identity_limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedRetentionClass {
    Current,
    InFlight,
    PredecessorInspection,
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
    pub const fn new(
        current: UiMountedRetentionClassBudget,
        in_flight: UiMountedRetentionClassBudget,
        predecessor_inspection: UiMountedRetentionClassBudget,
        expired_identity_limit: usize,
    ) -> Self {
        Self {
            current,
            in_flight,
            predecessor_inspection,
            expired_identity_limit,
        }
    }

    pub const fn current(self) -> UiMountedRetentionClassBudget {
        self.current
    }

    pub const fn in_flight(self) -> UiMountedRetentionClassBudget {
        self.in_flight
    }

    pub const fn predecessor_inspection(self) -> UiMountedRetentionClassBudget {
        self.predecessor_inspection
    }

    pub const fn expired_identity_limit(self) -> usize {
        self.expired_identity_limit
    }
}

impl Default for UiMountedFrameRetentionBudget {
    fn default() -> Self {
        const MIB: usize = 1024 * 1024;
        Self::new(
            UiMountedRetentionClassBudget::new(1, 64 * MIB),
            UiMountedRetentionClassBudget::new(1, 64 * MIB),
            UiMountedRetentionClassBudget::new(8, 256 * MIB),
            64,
        )
    }
}

impl UiMountedFrameRetentionDenial {
    pub fn class(self) -> UiMountedRetentionClass {
        match self {
            Self::CapacityExceeded { class, .. } | Self::AccountingOverflow { class } => class,
        }
    }
}
