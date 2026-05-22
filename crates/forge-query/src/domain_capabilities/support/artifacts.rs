use forge_foundational::{
    FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticSupportReport,
};

use super::super::materialization::ForgeQueryDomainCapabilityProfileProgression;
use super::super::payloads::ForgeQueryDomainCapabilityPayload;
use super::super::targets::ForgeQueryDomainCapabilityTargetBinding;

pub struct ForgeQueryDomainCapabilitySupportReport<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    contribution: super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    profile_progression: ForgeQueryDomainCapabilityProfileProgression,
    provenance: forge_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
    report: FoundationalDiagnosticSupportReport,
}

impl<P, T> ForgeQueryDomainCapabilitySupportReport<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn new(
        contribution: super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<
            P,
            T,
        >,
        profile_progression: ForgeQueryDomainCapabilityProfileProgression,
        provenance: forge_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
        report: FoundationalDiagnosticSupportReport,
    ) -> Self {
        Self {
            contribution,
            profile_progression,
            provenance,
            report,
        }
    }

    pub fn contribution(
        &self,
    ) -> &super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T> {
        &self.contribution
    }

    pub fn profile_progression(&self) -> &ForgeQueryDomainCapabilityProfileProgression {
        &self.profile_progression
    }

    pub fn provenance(
        &self,
    ) -> &forge_foundational::FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn report(&self) -> &FoundationalDiagnosticSupportReport {
        &self.report
    }
}

pub struct ForgeQueryDomainCapabilityExplanationBundle<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    contribution: super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    profile_progression: ForgeQueryDomainCapabilityProfileProgression,
    provenance: forge_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
    bundle: FoundationalDiagnosticExplanationBundle,
}

impl<P, T> ForgeQueryDomainCapabilityExplanationBundle<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn new(
        contribution: super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<
            P,
            T,
        >,
        profile_progression: ForgeQueryDomainCapabilityProfileProgression,
        provenance: forge_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
        bundle: FoundationalDiagnosticExplanationBundle,
    ) -> Self {
        Self {
            contribution,
            profile_progression,
            provenance,
            bundle,
        }
    }

    pub fn contribution(
        &self,
    ) -> &super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T> {
        &self.contribution
    }

    pub fn profile_progression(&self) -> &ForgeQueryDomainCapabilityProfileProgression {
        &self.profile_progression
    }

    pub fn provenance(
        &self,
    ) -> &forge_foundational::FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn bundle(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.bundle
    }
}
