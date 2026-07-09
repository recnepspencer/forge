use worth_foundational::{
    FoundationalDiagnosticCodeId, FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticSubject,
};

use super::super::materialization::WorthQueryDomainCapabilityProfileProgression;
use super::super::payloads::WorthQueryDomainCapabilityPayload;
use super::super::targets::WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryDomainCapabilityDescriptiveSummary<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    contribution: super::super::WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    profile_progression: WorthQueryDomainCapabilityProfileProgression,
    provenance: worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
    subject: FoundationalDiagnosticSubject,
    primary_code: FoundationalDiagnosticCodeId,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    required_row_count: usize,
    standard_row_count: usize,
    forensic_row_count: usize,
}

impl<P, T> WorthQueryDomainCapabilityDescriptiveSummary<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contribution: super::super::WorthQueryMaterializationReadyDomainCapabilityContribution<
            P,
            T,
        >,
        profile_progression: WorthQueryDomainCapabilityProfileProgression,
        provenance: worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact,
        subject: FoundationalDiagnosticSubject,
        primary_code: FoundationalDiagnosticCodeId,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        required_row_count: usize,
        standard_row_count: usize,
        forensic_row_count: usize,
    ) -> Self {
        Self {
            contribution,
            profile_progression,
            provenance,
            subject,
            primary_code,
            outcome_kind,
            required_row_count,
            standard_row_count,
            forensic_row_count,
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

    pub fn subject(&self) -> &FoundationalDiagnosticSubject {
        &self.subject
    }

    pub fn primary_code(&self) -> &FoundationalDiagnosticCodeId {
        &self.primary_code
    }

    pub fn outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.outcome_kind
    }

    pub fn required_row_count(&self) -> usize {
        self.required_row_count
    }

    pub fn standard_row_count(&self) -> usize {
        self.standard_row_count
    }

    pub fn forensic_row_count(&self) -> usize {
        self.forensic_row_count
    }
}
