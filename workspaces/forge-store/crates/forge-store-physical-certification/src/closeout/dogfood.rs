use super::PhysicalSimulationHarnessCloseoutDenial;
use crate::{
    CertifiedPhysicalScenario, PhysicalScenarioActorRole, PhysicalScenarioExpectationKind,
    PhysicalScenarioIntent, PhysicalScenarioNonClaim, PhysicalSimulationScenarioFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4RecoveryDogfoodScenario {
    scenario: CertifiedPhysicalScenario,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutRejectionDogfoodScenario {
    scenario: CertifiedPhysicalScenario,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationReadinessShapeProbeScenario {
    scenario: CertifiedPhysicalScenario,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationHarnessDogfoodReport {
    recovery: S4RecoveryDogfoodScenario,
    shortcut_rejection: ShortcutRejectionDogfoodScenario,
    physical_isolation_readiness_shape_probe: PhysicalIsolationReadinessShapeProbeScenario,
}

impl S4RecoveryDogfoodScenario {
    pub fn from_public_authoring(
        scenario: CertifiedPhysicalScenario,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_scenario_shape(
            &scenario,
            PhysicalSimulationScenarioFamily::S4RecoveryDogfood,
            PhysicalScenarioIntent::RecoveryReplayDogfood,
            PhysicalScenarioExpectationKind::S4RecoveryDogfood,
        )?;
        require_actor(&scenario, PhysicalScenarioActorRole::RecoveryDriver)?;
        Ok(Self { scenario })
    }

    pub const fn scenario(&self) -> &CertifiedPhysicalScenario {
        &self.scenario
    }

    pub const fn used_public_authoring_api(&self) -> bool {
        true
    }
}

impl ShortcutRejectionDogfoodScenario {
    pub fn from_public_authoring(
        scenario: CertifiedPhysicalScenario,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_scenario_shape(
            &scenario,
            PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood,
            PhysicalScenarioIntent::ForbiddenShortcutRejectionShape,
            PhysicalScenarioExpectationKind::ShortcutRejectionDogfood,
        )?;
        require_actor(&scenario, PhysicalScenarioActorRole::ShortcutRejectionProbe)?;
        Ok(Self { scenario })
    }

    pub const fn scenario(&self) -> &CertifiedPhysicalScenario {
        &self.scenario
    }

    pub const fn used_public_authoring_api(&self) -> bool {
        true
    }
}

impl PhysicalIsolationReadinessShapeProbeScenario {
    pub fn from_public_authoring(
        scenario: CertifiedPhysicalScenario,
    ) -> Result<Self, PhysicalSimulationHarnessCloseoutDenial> {
        require_scenario_shape(
            &scenario,
            PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe,
            PhysicalScenarioIntent::ProtectBeforeObserveShape,
            PhysicalScenarioExpectationKind::PhysicalIsolationReadinessWithShortcutRejectionProbe,
        )?;
        require_actor(&scenario, PhysicalScenarioActorRole::MaintenanceReclaimer)?;
        require_actor(&scenario, PhysicalScenarioActorRole::ForegroundReader)?;
        require_non_claim(
            &scenario,
            PhysicalScenarioNonClaim::NoPhysicalIsolationCorrectnessClaim,
        )?;
        Ok(Self { scenario })
    }

    pub const fn scenario(&self) -> &CertifiedPhysicalScenario {
        &self.scenario
    }

    pub const fn used_public_authoring_api(&self) -> bool {
        true
    }
}

impl SimulationHarnessDogfoodReport {
    pub const fn new(
        recovery: S4RecoveryDogfoodScenario,
        shortcut_rejection: ShortcutRejectionDogfoodScenario,
        physical_isolation_readiness_shape_probe: PhysicalIsolationReadinessShapeProbeScenario,
    ) -> Self {
        Self {
            recovery,
            shortcut_rejection,
            physical_isolation_readiness_shape_probe,
        }
    }

    pub const fn recovery_slice(&self) -> &S4RecoveryDogfoodScenario {
        &self.recovery
    }

    pub const fn shortcut_rejection_slice(&self) -> &ShortcutRejectionDogfoodScenario {
        &self.shortcut_rejection
    }

    pub const fn physical_isolation_readiness_shape_probe(
        &self,
    ) -> &PhysicalIsolationReadinessShapeProbeScenario {
        &self.physical_isolation_readiness_shape_probe
    }
}

fn require_scenario_shape(
    scenario: &CertifiedPhysicalScenario,
    expected_family: PhysicalSimulationScenarioFamily,
    expected_intent: PhysicalScenarioIntent,
    expected_expectation: PhysicalScenarioExpectationKind,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    let definition = scenario.definition();
    if definition.family() != expected_family {
        return Err(
            PhysicalSimulationHarnessCloseoutDenial::WrongDogfoodScenarioFamily {
                expected: expected_family,
                actual: definition.family(),
            },
        );
    }
    if definition.intent() != expected_intent {
        return Err(
            PhysicalSimulationHarnessCloseoutDenial::WrongDogfoodScenarioIntent {
                expected: expected_intent,
                actual: definition.intent(),
            },
        );
    }
    if definition.expectation().kind() != expected_expectation {
        return Err(
            PhysicalSimulationHarnessCloseoutDenial::WrongDogfoodScenarioExpectation {
                expected: expected_expectation,
                actual: definition.expectation().kind(),
            },
        );
    }
    Ok(())
}

fn require_actor(
    scenario: &CertifiedPhysicalScenario,
    role: PhysicalScenarioActorRole,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    if scenario
        .definition()
        .actors()
        .iter()
        .any(|actor| actor.role() == role)
    {
        Ok(())
    } else {
        Err(PhysicalSimulationHarnessCloseoutDenial::MissingDogfoodActor { role })
    }
}

fn require_non_claim(
    scenario: &CertifiedPhysicalScenario,
    non_claim: PhysicalScenarioNonClaim,
) -> Result<(), PhysicalSimulationHarnessCloseoutDenial> {
    if scenario
        .definition()
        .expectation()
        .non_claims()
        .contains(&non_claim)
    {
        Ok(())
    } else {
        Err(PhysicalSimulationHarnessCloseoutDenial::MissingScenarioNonClaim { non_claim })
    }
}
