use super::{bypass::LegacyAccessPathBypass, disposition::LegacySurfaceDisposition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySurfaceOwner {
    LegacyRootCrate,
    CertificationLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySurfaceStage {
    DeclarationFacade,
    DeclarationArtifact,
    AdmissionFacade,
    AdmissionArtifact,
    SelectionFacade,
    SelectionArtifact,
    ReadinessFacade,
    ReadinessArtifact,
    ExecutionArtifact,
    ExecutionFacade,
    InputOnlyArtifact,
    CertificationArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacySurfaceInventoryRow {
    surface: &'static str,
    owner: LegacySurfaceOwner,
    stage: LegacySurfaceStage,
    disposition: LegacySurfaceDisposition,
    bypass: LegacyAccessPathBypass,
}

impl LegacySurfaceInventoryRow {
    pub const fn new(
        surface: &'static str,
        owner: LegacySurfaceOwner,
        stage: LegacySurfaceStage,
        disposition: LegacySurfaceDisposition,
        bypass: LegacyAccessPathBypass,
    ) -> Self {
        Self {
            surface,
            owner,
            stage,
            disposition,
            bypass,
        }
    }

    pub const fn surface(self) -> &'static str {
        self.surface
    }

    pub const fn owner(self) -> LegacySurfaceOwner {
        self.owner
    }

    pub const fn stage(self) -> LegacySurfaceStage {
        self.stage
    }

    pub const fn disposition(self) -> LegacySurfaceDisposition {
        self.disposition
    }

    pub const fn bypass(self) -> LegacyAccessPathBypass {
        self.bypass
    }
}
