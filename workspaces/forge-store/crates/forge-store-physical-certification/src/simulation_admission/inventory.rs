use super::denial::SimulationHarnessBoundaryDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimulationHarnessSurfaceClassification {
    ReusableMechanics,
    MilestoneLocalMechanics,
    CertificationMeaning,
    ObsoleteSemanticContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegisteredSimulationHarnessSurface {
    TestSupportS4RecoveryPhysics,
    TestSupportNativeAspectFixtures,
    TestSupportTerminalProjectionJsonFixtures,
    TestSupportHostileReadmissionJsonFixtures,
    CertificationS4RecoveryHarness,
    ObsoleteSemanticHarness,
}

impl RegisteredSimulationHarnessSurface {
    pub const fn path(self) -> &'static str {
        match self {
            Self::TestSupportS4RecoveryPhysics => "forge-store-test-support::s4_recovery_physics",
            Self::TestSupportNativeAspectFixtures => {
                "forge-store-test-support::native_aspect_fixtures"
            }
            Self::TestSupportTerminalProjectionJsonFixtures => {
                "forge-store-test-support::terminal_projection_json_fixtures"
            }
            Self::TestSupportHostileReadmissionJsonFixtures => {
                "forge-store-test-support::hostile_readmission_json_fixtures"
            }
            Self::CertificationS4RecoveryHarness => {
                "forge-store-certification::s4_recovery_harness"
            }
            Self::ObsoleteSemanticHarness => "crates/forge-store/src/tests/harness",
        }
    }

    pub const fn classification(self) -> SimulationHarnessSurfaceClassification {
        match self {
            Self::TestSupportS4RecoveryPhysics | Self::TestSupportNativeAspectFixtures => {
                SimulationHarnessSurfaceClassification::ReusableMechanics
            }
            Self::TestSupportTerminalProjectionJsonFixtures
            | Self::TestSupportHostileReadmissionJsonFixtures => {
                SimulationHarnessSurfaceClassification::MilestoneLocalMechanics
            }
            Self::CertificationS4RecoveryHarness => {
                SimulationHarnessSurfaceClassification::CertificationMeaning
            }
            Self::ObsoleteSemanticHarness => {
                SimulationHarnessSurfaceClassification::ObsoleteSemanticContext
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExistingSimulationHarnessSurface {
    path: String,
    classification: SimulationHarnessSurfaceClassification,
}

impl ExistingSimulationHarnessSurface {
    pub(crate) fn new(
        path: impl Into<String>,
        classification: SimulationHarnessSurfaceClassification,
    ) -> Self {
        Self {
            path: path.into(),
            classification,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn classification(&self) -> SimulationHarnessSurfaceClassification {
        self.classification
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExistingSimulationHarnessInventory {
    surfaces: Vec<ExistingSimulationHarnessSurface>,
}

impl ExistingSimulationHarnessInventory {
    pub fn dedicated_workspace_baseline() -> Self {
        Self::from_registered_surfaces(required_registered_surfaces().to_vec())
    }

    pub fn from_registered_surfaces(surfaces: Vec<RegisteredSimulationHarnessSurface>) -> Self {
        let mut surfaces = surfaces;
        surfaces.sort();
        surfaces.dedup();
        Self {
            surfaces: surfaces.into_iter().map(registered_surface).collect(),
        }
    }

    pub fn surfaces(&self) -> &[ExistingSimulationHarnessSurface] {
        &self.surfaces
    }

    pub fn contains_reusable_mechanics(&self, path: &str) -> bool {
        self.contains_classification(
            path,
            SimulationHarnessSurfaceClassification::ReusableMechanics,
        )
    }

    pub fn contains_milestone_local_mechanics(&self, path: &str) -> bool {
        self.contains_classification(
            path,
            SimulationHarnessSurfaceClassification::MilestoneLocalMechanics,
        )
    }

    pub fn contains_certification_meaning(&self, path: &str) -> bool {
        self.contains_classification(
            path,
            SimulationHarnessSurfaceClassification::CertificationMeaning,
        )
    }

    pub fn contains_obsolete_semantic_context(&self, path: &str) -> bool {
        self.contains_classification(
            path,
            SimulationHarnessSurfaceClassification::ObsoleteSemanticContext,
        )
    }

    pub(crate) fn validate_for_simulation_harness_entry(
        &self,
    ) -> Result<(), SimulationHarnessBoundaryDenial> {
        self.require_registered_baseline()?;
        self.require_classification(SimulationHarnessSurfaceClassification::ReusableMechanics)?;
        self.require_classification(SimulationHarnessSurfaceClassification::CertificationMeaning)?;
        self.require_classification(
            SimulationHarnessSurfaceClassification::ObsoleteSemanticContext,
        )?;
        self.require_test_support_stays_mechanics()?;
        self.require_old_semantic_harness_stays_obsolete()
    }

    fn contains_classification(
        &self,
        path: &str,
        classification: SimulationHarnessSurfaceClassification,
    ) -> bool {
        self.surfaces
            .iter()
            .any(|surface| surface.path() == path && surface.classification() == classification)
    }

    fn require_classification(
        &self,
        classification: SimulationHarnessSurfaceClassification,
    ) -> Result<(), SimulationHarnessBoundaryDenial> {
        if self
            .surfaces
            .iter()
            .any(|surface| surface.classification() == classification)
        {
            return Ok(());
        }
        Err(missing_classification_denial(classification))
    }

    fn require_test_support_stays_mechanics(&self) -> Result<(), SimulationHarnessBoundaryDenial> {
        if self.surfaces.iter().any(|surface| {
            surface.path().starts_with("forge-store-test-support")
                && surface.classification()
                    == SimulationHarnessSurfaceClassification::CertificationMeaning
        }) {
            return Err(
                SimulationHarnessBoundaryDenial::TestSupportMechanicsCannotOwnCertificationMeaning,
            );
        }
        Ok(())
    }

    fn require_old_semantic_harness_stays_obsolete(
        &self,
    ) -> Result<(), SimulationHarnessBoundaryDenial> {
        if self.surfaces.iter().any(|surface| {
            old_semantic_harness_path(surface.path())
                && surface.classification()
                    != SimulationHarnessSurfaceClassification::ObsoleteSemanticContext
        }) {
            return Err(SimulationHarnessBoundaryDenial::OldSemanticHarnessContextCannotAdmitEntry);
        }
        Ok(())
    }

    fn require_registered_baseline(&self) -> Result<(), SimulationHarnessBoundaryDenial> {
        for surface in required_registered_surfaces() {
            if !self.contains_classification(surface.path(), surface.classification()) {
                return Err(missing_classification_denial(surface.classification()));
            }
        }
        Ok(())
    }
}

fn old_semantic_harness_path(path: &str) -> bool {
    path.contains("crates/forge-store/src/tests/harness") || path.contains("legacy::tests::harness")
}

fn registered_surface(
    surface: RegisteredSimulationHarnessSurface,
) -> ExistingSimulationHarnessSurface {
    ExistingSimulationHarnessSurface::new(surface.path(), surface.classification())
}

fn required_registered_surfaces() -> &'static [RegisteredSimulationHarnessSurface] {
    &[
        RegisteredSimulationHarnessSurface::TestSupportS4RecoveryPhysics,
        RegisteredSimulationHarnessSurface::TestSupportNativeAspectFixtures,
        RegisteredSimulationHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        RegisteredSimulationHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        RegisteredSimulationHarnessSurface::CertificationS4RecoveryHarness,
        RegisteredSimulationHarnessSurface::ObsoleteSemanticHarness,
    ]
}

fn missing_classification_denial(
    classification: SimulationHarnessSurfaceClassification,
) -> SimulationHarnessBoundaryDenial {
    match classification {
        SimulationHarnessSurfaceClassification::ReusableMechanics => {
            SimulationHarnessBoundaryDenial::MissingReusableMechanicsInventory
        }
        SimulationHarnessSurfaceClassification::CertificationMeaning => {
            SimulationHarnessBoundaryDenial::MissingCertificationMeaningInventory
        }
        SimulationHarnessSurfaceClassification::ObsoleteSemanticContext => {
            SimulationHarnessBoundaryDenial::MissingObsoleteSemanticContextInventory
        }
        SimulationHarnessSurfaceClassification::MilestoneLocalMechanics => {
            SimulationHarnessBoundaryDenial::MissingMilestoneLocalMechanicsInventory
        }
    }
}
