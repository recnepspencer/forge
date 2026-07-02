use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalSimulationCapability {
    ProductionBoundaryDriver,
    IndependentObserver,
    CertificationOracleFamily,
    CounterContracts,
    FixtureClassAdmission,
    EvidencePolicy,
    ForbiddenShortcutDenial,
    ProfileSupport,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationCapabilitySet {
    capabilities: BTreeSet<PhysicalSimulationCapability>,
}

impl PhysicalSimulationCapabilitySet {
    pub fn empty() -> Self {
        Self {
            capabilities: BTreeSet::new(),
        }
    }

    pub fn all_for_developer_smoke() -> Self {
        Self::from_capabilities(BASELINE_CAPABILITIES)
    }

    pub fn s4_recovery_dogfood() -> Self {
        Self::all_for_developer_smoke()
    }

    pub fn s5_readiness_shape_probe() -> Self {
        Self::all_for_developer_smoke()
    }

    pub fn s5_ci_certification() -> Self {
        Self::all_for_developer_smoke()
    }

    pub fn shortcut_rejection_dogfood() -> Self {
        Self::all_for_developer_smoke()
    }

    pub fn without(mut self, capability: PhysicalSimulationCapability) -> Self {
        self.capabilities.remove(&capability);
        self
    }

    pub fn contains(&self, capability: PhysicalSimulationCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn iter(&self) -> impl Iterator<Item = PhysicalSimulationCapability> + '_ {
        self.capabilities.iter().copied()
    }

    pub(crate) fn from_capabilities(
        capabilities: impl IntoIterator<Item = PhysicalSimulationCapability>,
    ) -> Self {
        Self {
            capabilities: capabilities.into_iter().collect(),
        }
    }
}

const BASELINE_CAPABILITIES: [PhysicalSimulationCapability; 7] = [
    PhysicalSimulationCapability::ProductionBoundaryDriver,
    PhysicalSimulationCapability::IndependentObserver,
    PhysicalSimulationCapability::CertificationOracleFamily,
    PhysicalSimulationCapability::CounterContracts,
    PhysicalSimulationCapability::FixtureClassAdmission,
    PhysicalSimulationCapability::EvidencePolicy,
    PhysicalSimulationCapability::ForbiddenShortcutDenial,
];

pub(crate) fn capability_token(capability: PhysicalSimulationCapability) -> &'static str {
    match capability {
        PhysicalSimulationCapability::ProductionBoundaryDriver => "production-boundary-driver",
        PhysicalSimulationCapability::IndependentObserver => "independent-observer",
        PhysicalSimulationCapability::CertificationOracleFamily => "certification-oracle-family",
        PhysicalSimulationCapability::CounterContracts => "counter-contracts",
        PhysicalSimulationCapability::FixtureClassAdmission => "fixture-class-admission",
        PhysicalSimulationCapability::EvidencePolicy => "evidence-policy",
        PhysicalSimulationCapability::ForbiddenShortcutDenial => "forbidden-shortcut-denial",
        PhysicalSimulationCapability::ProfileSupport => "profile-support",
        PhysicalSimulationCapability::FutureExtensionSlot => "future-extension-slot",
    }
}
