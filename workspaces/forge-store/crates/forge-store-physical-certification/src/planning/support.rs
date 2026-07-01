use super::requirements::{ObserverKind, OracleFamilyKind, PhysicalDriverKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedPhysicalDriverSet {
    drivers: Vec<PhysicalDriverKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedObserverSet {
    observers: Vec<ObserverKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedOracleFamilySet {
    oracle_families: Vec<OracleFamilyKind>,
}

impl SupportedPhysicalDriverSet {
    pub fn empty() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }

    pub fn all_for_developer_smoke() -> Self {
        Self::from_drivers([
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            PhysicalDriverKind::FreshRuntimeRecovery,
            PhysicalDriverKind::MemoryPressureBoundary,
            PhysicalDriverKind::IoPressureBoundary,
            PhysicalDriverKind::OfflineVerifierBoundary,
            PhysicalDriverKind::ShortcutRejectionBoundary,
        ])
    }

    pub fn without(mut self, driver: PhysicalDriverKind) -> Self {
        self.drivers.retain(|candidate| *candidate != driver);
        self
    }

    pub fn contains(&self, driver: PhysicalDriverKind) -> bool {
        self.drivers.contains(&driver)
    }

    fn from_drivers(drivers: impl IntoIterator<Item = PhysicalDriverKind>) -> Self {
        Self {
            drivers: sorted_unique(drivers),
        }
    }
}

impl SupportedObserverSet {
    pub fn empty() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn all_for_developer_smoke() -> Self {
        Self::from_observers([
            ObserverKind::IndependentPhysicalTrace,
            ObserverKind::RecoveryOutcomeObserver,
            ObserverKind::ShortcutRejectionObserver,
        ])
    }

    pub fn without(mut self, observer: ObserverKind) -> Self {
        self.observers.retain(|candidate| *candidate != observer);
        self
    }

    pub fn contains(&self, observer: ObserverKind) -> bool {
        self.observers.contains(&observer)
    }

    fn from_observers(observers: impl IntoIterator<Item = ObserverKind>) -> Self {
        Self {
            observers: sorted_unique(observers),
        }
    }
}

impl SupportedOracleFamilySet {
    pub fn empty() -> Self {
        Self {
            oracle_families: Vec::new(),
        }
    }

    pub fn all_for_developer_smoke() -> Self {
        Self::from_oracles([
            OracleFamilyKind::TranscriptReplayEvidence,
            OracleFamilyKind::S5ReadinessShape,
            OracleFamilyKind::S4RecoveryDogfood,
            OracleFamilyKind::ForbiddenShortcutRejection,
        ])
    }

    pub fn without(mut self, oracle_family: OracleFamilyKind) -> Self {
        self.oracle_families
            .retain(|candidate| *candidate != oracle_family);
        self
    }

    pub fn contains(&self, oracle_family: OracleFamilyKind) -> bool {
        self.oracle_families.contains(&oracle_family)
    }

    fn from_oracles(oracles: impl IntoIterator<Item = OracleFamilyKind>) -> Self {
        Self {
            oracle_families: sorted_unique(oracles),
        }
    }
}

fn sorted_unique<T: Ord>(values: impl IntoIterator<Item = T>) -> Vec<T> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}
