use forge_foundational::{
    FoundationalDiagnosticMaterializationDenial, FoundationalProfileIdentityDenial,
    FoundationalProfileProgressionDenial,
};

use super::payloads::ForgeQueryDomainCapabilityCategory;

pub use super::summary::*;
pub use super::support::*;
pub use super::trace::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainCapabilityDescriptiveArtifactKind {
    Summary,
    SupportReport,
    TraceArtifact,
    ExplanationBundle,
}

impl ForgeQueryDomainCapabilityDescriptiveArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::SupportReport => "support-report",
            Self::TraceArtifact => "trace-artifact",
            Self::ExplanationBundle => "explanation-bundle",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQueryDomainCapabilityDescriptiveMaterializationDenial {
    ProfileAdmission {
        category: ForgeQueryDomainCapabilityCategory,
        artifact_kind: ForgeQueryDomainCapabilityDescriptiveArtifactKind,
        denial: FoundationalProfileProgressionDenial,
    },
    ProfileMaterialization {
        category: ForgeQueryDomainCapabilityCategory,
        artifact_kind: ForgeQueryDomainCapabilityDescriptiveArtifactKind,
        denial: FoundationalProfileProgressionDenial,
    },
    ProfileIdentity {
        category: ForgeQueryDomainCapabilityCategory,
        artifact_kind: ForgeQueryDomainCapabilityDescriptiveArtifactKind,
        denial: FoundationalProfileIdentityDenial,
    },
    SupportReport {
        category: ForgeQueryDomainCapabilityCategory,
        denial: FoundationalDiagnosticMaterializationDenial,
    },
    ExplanationBundle {
        category: ForgeQueryDomainCapabilityCategory,
        denial: FoundationalDiagnosticMaterializationDenial,
    },
}

pub struct ForgeQueryDomainCapabilityProfileProgression {
    requested: forge_foundational::RequestedFoundationalProfileArtifact,
    admitted: forge_foundational::AdmittedFoundationalProfileArtifact,
    materialized: forge_foundational::MaterializedFoundationalProfileArtifact,
}

impl ForgeQueryDomainCapabilityProfileProgression {
    pub fn new(
        requested: forge_foundational::RequestedFoundationalProfileArtifact,
        admitted: forge_foundational::AdmittedFoundationalProfileArtifact,
        materialized: forge_foundational::MaterializedFoundationalProfileArtifact,
    ) -> Self {
        Self {
            requested,
            admitted,
            materialized,
        }
    }

    pub fn requested(&self) -> &forge_foundational::RequestedFoundationalProfileArtifact {
        &self.requested
    }

    pub fn admitted(&self) -> &forge_foundational::AdmittedFoundationalProfileArtifact {
        &self.admitted
    }

    pub fn materialized(&self) -> &forge_foundational::MaterializedFoundationalProfileArtifact {
        &self.materialized
    }
}
