use crate::PhysicalReadStabilityAuthority;

use super::{PhysicalLatchKey, PhysicalLatchMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LatchUpgradeAuthority {
    _private: (),
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum LatchAcquisitionStep {
    Acquire {
        key: PhysicalLatchKey,
        mode: PhysicalLatchMode,
    },
    Upgrade {
        key: PhysicalLatchKey,
        authority: LatchUpgradeAuthority,
    },
}

impl LatchUpgradeAuthority {
    pub const fn from_physical_read_stability_authority(
        _: &PhysicalReadStabilityAuthority,
    ) -> Self {
        Self { _private: () }
    }
}

impl LatchAcquisitionStep {
    pub const fn shared(key: PhysicalLatchKey) -> Self {
        Self::Acquire {
            key,
            mode: PhysicalLatchMode::Shared,
        }
    }

    pub const fn exclusive(key: PhysicalLatchKey) -> Self {
        Self::Acquire {
            key,
            mode: PhysicalLatchMode::Exclusive,
        }
    }

    pub const fn upgrade(key: PhysicalLatchKey, authority: LatchUpgradeAuthority) -> Self {
        Self::Upgrade { key, authority }
    }

    pub const fn key(self) -> PhysicalLatchKey {
        match self {
            Self::Acquire { key, .. } | Self::Upgrade { key, .. } => key,
        }
    }

    pub const fn mode(self) -> PhysicalLatchMode {
        match self {
            Self::Acquire { mode, .. } => mode,
            Self::Upgrade { .. } => PhysicalLatchMode::Exclusive,
        }
    }

    pub const fn is_upgrade(self) -> bool {
        matches!(self, Self::Upgrade { .. })
    }
}
