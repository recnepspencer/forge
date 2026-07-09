use super::denial::S45HarnessBoundaryDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S45HarnessSurfaceClassification {
    ReusableMechanics,
    MilestoneLocalMechanics,
    CertificationMeaning,
    ObsoleteSemanticContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S45RegisteredHarnessSurface {
    TestSupportS4RecoveryPhysics,
    TestSupportNativeAspectFixtures,
    TestSupportTerminalProjectionJsonFixtures,
    TestSupportHostileReadmissionJsonFixtures,
    CertificationS4RecoveryHarness,
    ObsoleteSemanticHarness,
}

impl S45RegisteredHarnessSurface {
    pub const fn path(self) -> &'static str {
        match self {
            Self::TestSupportS4RecoveryPhysics => "worth-store-test-support::s4_recovery_physics",
            Self::TestSupportNativeAspectFixtures => {
                "worth-store-test-support::native_aspect_fixtures"
            }
            Self::TestSupportTerminalProjectionJsonFixtures => {
                "worth-store-test-support::terminal_projection_json_fixtures"
            }
            Self::TestSupportHostileReadmissionJsonFixtures => {
                "worth-store-test-support::hostile_readmission_json_fixtures"
            }
            Self::CertificationS4RecoveryHarness => {
                "worth-store-certification::s4_recovery_harness"
            }
            Self::ObsoleteSemanticHarness => "crates/worth-store/src/tests/harness",
        }
    }

    pub const fn classification(self) -> S45HarnessSurfaceClassification {
        match self {
            Self::TestSupportS4RecoveryPhysics | Self::TestSupportNativeAspectFixtures => {
                S45HarnessSurfaceClassification::ReusableMechanics
            }
            Self::TestSupportTerminalProjectionJsonFixtures
            | Self::TestSupportHostileReadmissionJsonFixtures => {
                S45HarnessSurfaceClassification::MilestoneLocalMechanics
            }
            Self::CertificationS4RecoveryHarness => {
                S45HarnessSurfaceClassification::CertificationMeaning
            }
            Self::ObsoleteSemanticHarness => {
                S45HarnessSurfaceClassification::ObsoleteSemanticContext
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45ExistingHarnessSurface {
    path: String,
    classification: S45HarnessSurfaceClassification,
}

impl S45ExistingHarnessSurface {
    pub(crate) fn new(
        path: impl Into<String>,
        classification: S45HarnessSurfaceClassification,
    ) -> Self {
        Self {
            path: path.into(),
            classification,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn classification(&self) -> S45HarnessSurfaceClassification {
        self.classification
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S45ExistingHarnessInventory {
    surfaces: Vec<S45ExistingHarnessSurface>,
}

impl S45ExistingHarnessInventory {
    pub fn dedicated_workspace_baseline() -> Self {
        Self::from_registered_surfaces(required_registered_surfaces().to_vec())
    }

    pub fn from_registered_surfaces(surfaces: Vec<S45RegisteredHarnessSurface>) -> Self {
        let mut surfaces = surfaces;
        surfaces.sort();
        surfaces.dedup();
        Self {
            surfaces: surfaces.into_iter().map(registered_surface).collect(),
        }
    }

    pub fn surfaces(&self) -> &[S45ExistingHarnessSurface] {
        &self.surfaces
    }

    pub fn contains_reusable_mechanics(&self, path: &str) -> bool {
        self.contains_classification(path, S45HarnessSurfaceClassification::ReusableMechanics)
    }

    pub fn contains_milestone_local_mechanics(&self, path: &str) -> bool {
        self.contains_classification(
            path,
            S45HarnessSurfaceClassification::MilestoneLocalMechanics,
        )
    }

    pub fn contains_certification_meaning(&self, path: &str) -> bool {
        self.contains_classification(path, S45HarnessSurfaceClassification::CertificationMeaning)
    }

    pub fn contains_obsolete_semantic_context(&self, path: &str) -> bool {
        self.contains_classification(
            path,
            S45HarnessSurfaceClassification::ObsoleteSemanticContext,
        )
    }

    pub(crate) fn validate_for_s45_entry(&self) -> Result<(), S45HarnessBoundaryDenial> {
        self.require_registered_baseline()?;
        self.require_classification(S45HarnessSurfaceClassification::ReusableMechanics)?;
        self.require_classification(S45HarnessSurfaceClassification::CertificationMeaning)?;
        self.require_classification(S45HarnessSurfaceClassification::ObsoleteSemanticContext)?;
        self.require_test_support_stays_mechanics()?;
        self.require_old_semantic_harness_stays_obsolete()
    }

    fn contains_classification(
        &self,
        path: &str,
        classification: S45HarnessSurfaceClassification,
    ) -> bool {
        self.surfaces
            .iter()
            .any(|surface| surface.path() == path && surface.classification() == classification)
    }

    fn require_classification(
        &self,
        classification: S45HarnessSurfaceClassification,
    ) -> Result<(), S45HarnessBoundaryDenial> {
        if self
            .surfaces
            .iter()
            .any(|surface| surface.classification() == classification)
        {
            return Ok(());
        }
        Err(missing_classification_denial(classification))
    }

    fn require_test_support_stays_mechanics(&self) -> Result<(), S45HarnessBoundaryDenial> {
        if self.surfaces.iter().any(|surface| {
            surface.path().starts_with("worth-store-test-support")
                && surface.classification() == S45HarnessSurfaceClassification::CertificationMeaning
        }) {
            return Err(
                S45HarnessBoundaryDenial::TestSupportMechanicsCannotOwnCertificationMeaning,
            );
        }
        Ok(())
    }

    fn require_old_semantic_harness_stays_obsolete(&self) -> Result<(), S45HarnessBoundaryDenial> {
        if self.surfaces.iter().any(|surface| {
            old_semantic_harness_path(surface.path())
                && surface.classification()
                    != S45HarnessSurfaceClassification::ObsoleteSemanticContext
        }) {
            return Err(S45HarnessBoundaryDenial::OldSemanticHarnessContextCannotAdmitEntry);
        }
        Ok(())
    }

    fn require_registered_baseline(&self) -> Result<(), S45HarnessBoundaryDenial> {
        for surface in required_registered_surfaces() {
            if !self.contains_classification(surface.path(), surface.classification()) {
                return Err(missing_classification_denial(surface.classification()));
            }
        }
        Ok(())
    }
}

fn old_semantic_harness_path(path: &str) -> bool {
    path.contains("crates/worth-store/src/tests/harness") || path.contains("legacy::tests::harness")
}

fn registered_surface(surface: S45RegisteredHarnessSurface) -> S45ExistingHarnessSurface {
    S45ExistingHarnessSurface::new(surface.path(), surface.classification())
}

fn required_registered_surfaces() -> &'static [S45RegisteredHarnessSurface] {
    &[
        S45RegisteredHarnessSurface::TestSupportS4RecoveryPhysics,
        S45RegisteredHarnessSurface::TestSupportNativeAspectFixtures,
        S45RegisteredHarnessSurface::TestSupportTerminalProjectionJsonFixtures,
        S45RegisteredHarnessSurface::TestSupportHostileReadmissionJsonFixtures,
        S45RegisteredHarnessSurface::CertificationS4RecoveryHarness,
        S45RegisteredHarnessSurface::ObsoleteSemanticHarness,
    ]
}

fn missing_classification_denial(
    classification: S45HarnessSurfaceClassification,
) -> S45HarnessBoundaryDenial {
    match classification {
        S45HarnessSurfaceClassification::ReusableMechanics => {
            S45HarnessBoundaryDenial::MissingReusableMechanicsInventory
        }
        S45HarnessSurfaceClassification::CertificationMeaning => {
            S45HarnessBoundaryDenial::MissingCertificationMeaningInventory
        }
        S45HarnessSurfaceClassification::ObsoleteSemanticContext => {
            S45HarnessBoundaryDenial::MissingObsoleteSemanticContextInventory
        }
        S45HarnessSurfaceClassification::MilestoneLocalMechanics => {
            S45HarnessBoundaryDenial::MissingMilestoneLocalMechanicsInventory
        }
    }
}
