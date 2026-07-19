use worth_foundational::{
    FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticSupportReport,
};

use super::super::materialization::WorthQueryDomainCapabilityProfileProgression;
use super::super::payloads::WorthQueryDomainCapabilityPayload;
use super::super::targets::WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryDomainCapabilitySupportReport<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    contribution: super::super::WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    profile_progression: WorthQueryDomainCapabilityProfileProgression,
    provenance: worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
    report: FoundationalDiagnosticSupportReport,
}

impl<P, T> WorthQueryDomainCapabilitySupportReport<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    pub fn new(
        contribution: super::super::WorthQueryMaterializationReadyDomainCapabilityContribution<
            P,
            T,
        >,
        profile_progression: WorthQueryDomainCapabilityProfileProgression,
        provenance: worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
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
    ) -> &super::super::WorthQueryMaterializationReadyDomainCapabilityContribution<P, T> {
        &self.contribution
    }

    pub fn profile_progression(&self) -> &WorthQueryDomainCapabilityProfileProgression {
        &self.profile_progression
    }

    pub fn provenance(
        &self,
    ) -> &worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn report(&self) -> &FoundationalDiagnosticSupportReport {
        &self.report
    }
}

pub struct WorthQueryDomainCapabilityExplanationBundle<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    contribution: super::super::WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    profile_progression: WorthQueryDomainCapabilityProfileProgression,
    provenance: worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
    bundle: FoundationalDiagnosticExplanationBundle,
}

impl<P, T> WorthQueryDomainCapabilityExplanationBundle<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    pub fn new(
        contribution: super::super::WorthQueryMaterializationReadyDomainCapabilityContribution<
            P,
            T,
        >,
        profile_progression: WorthQueryDomainCapabilityProfileProgression,
        provenance: worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
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
    ) -> &super::super::WorthQueryMaterializationReadyDomainCapabilityContribution<P, T> {
        &self.contribution
    }

    pub fn profile_progression(&self) -> &WorthQueryDomainCapabilityProfileProgression {
        &self.profile_progression
    }

    pub fn provenance(
        &self,
    ) -> &worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn bundle(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.bundle
    }
}
