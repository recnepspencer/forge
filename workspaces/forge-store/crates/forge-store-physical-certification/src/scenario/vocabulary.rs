#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalSimulationScenarioFamily {
    S4RecoveryDogfood,
    S5ReadinessShapeProbe,
    ShortcutRejectionDogfood,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioIntent {
    RecoveryReplayDogfood,
    ProtectBeforeObserveShape,
    ForbiddenShortcutRejectionShape,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioActor {
    id: String,
    role: PhysicalScenarioActorRole,
}

impl PhysicalScenarioActor {
    pub fn foreground_reader(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::ForegroundReader)
    }

    pub fn foreground_writer(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::ForegroundWriter)
    }

    pub fn checkpoint_driver(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::CheckpointDriver)
    }

    pub fn compaction_driver(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::CompactionDriver)
    }

    pub fn maintenance_reclaimer(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::MaintenanceReclaimer)
    }

    pub fn recovery_driver(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::RecoveryDriver)
    }

    pub fn scrub_driver(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::ScrubDriver)
    }

    pub fn offline_verifier(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::OfflineVerifier)
    }

    pub fn shortcut_rejection_probe(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::ShortcutRejectionProbe)
    }

    pub fn future_extension_slot(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::FutureExtensionSlot)
    }

    fn new(id: impl Into<String>, role: PhysicalScenarioActorRole) -> Self {
        Self {
            id: id.into(),
            role,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub const fn role(&self) -> PhysicalScenarioActorRole {
        self.role
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioActorRole {
    ForegroundReader,
    ForegroundWriter,
    CheckpointDriver,
    CompactionDriver,
    MaintenanceReclaimer,
    RecoveryDriver,
    ScrubDriver,
    OfflineVerifier,
    ShortcutRejectionProbe,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioSchedule {
    production_boundary_yieldpoint: String,
}

impl PhysicalScenarioSchedule {
    pub fn named_boundary_yieldpoint(production_boundary_yieldpoint: impl Into<String>) -> Self {
        Self {
            production_boundary_yieldpoint: production_boundary_yieldpoint.into(),
        }
    }

    pub fn production_boundary_yieldpoint(&self) -> &str {
        &self.production_boundary_yieldpoint
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioFault {
    kind: PhysicalScenarioFaultKind,
}

impl PhysicalScenarioFault {
    pub const fn no_fault() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::NoFault,
        }
    }

    pub const fn future_extension_slot() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::FutureExtensionSlot,
        }
    }

    pub const fn kind(&self) -> PhysicalScenarioFaultKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioFaultKind {
    NoFault,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioExpectation {
    kind: PhysicalScenarioExpectationKind,
    non_claims: Vec<PhysicalScenarioNonClaim>,
}

impl PhysicalScenarioExpectation {
    pub fn s4_recovery_dogfood() -> Self {
        Self {
            kind: PhysicalScenarioExpectationKind::S4RecoveryDogfood,
            non_claims: Vec::new(),
        }
    }

    pub fn shortcut_rejection_dogfood() -> Self {
        Self {
            kind: PhysicalScenarioExpectationKind::ShortcutRejectionDogfood,
            non_claims: Vec::new(),
        }
    }

    pub fn non_claiming_s5_readiness_shape() -> Self {
        Self {
            kind: PhysicalScenarioExpectationKind::S5ReadinessShapeProbe,
            non_claims: vec![PhysicalScenarioNonClaim::NoS5PhysicalIsolationCorrectnessClaim],
        }
    }

    pub fn non_claiming_s5_readiness_with_shortcut_rejection() -> Self {
        Self {
            kind: PhysicalScenarioExpectationKind::S5ReadinessWithShortcutRejectionProbe,
            non_claims: vec![PhysicalScenarioNonClaim::NoS5PhysicalIsolationCorrectnessClaim],
        }
    }

    pub fn with_future_extension_non_claim(mut self) -> Self {
        let non_claim = PhysicalScenarioNonClaim::FutureExtensionSlotDoesNotImplementFutureBehavior;
        if !self.non_claims.contains(&non_claim) {
            self.non_claims.push(non_claim);
        }
        self
    }

    pub const fn kind(&self) -> PhysicalScenarioExpectationKind {
        self.kind
    }

    pub fn non_claims(&self) -> &[PhysicalScenarioNonClaim] {
        &self.non_claims
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioExpectationKind {
    S4RecoveryDogfood,
    S5ReadinessShapeProbe,
    S5ReadinessWithShortcutRejectionProbe,
    ShortcutRejectionDogfood,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioNonClaim {
    NoS5PhysicalIsolationCorrectnessClaim,
    FutureExtensionSlotDoesNotImplementFutureBehavior,
}
