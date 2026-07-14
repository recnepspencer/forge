use worth_foundational::{
    FoundationalDiagnosticCodeId, FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRow,
    FoundationalDiagnosticSubject,
};

use super::super::materialization::WorthQueryDomainCapabilityProfileProgression;
use super::super::payloads::WorthQueryDomainCapabilityPayload;
use super::super::targets::WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryDomainCapabilityTraceArtifact<P, T>
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
    required_rows: Vec<FoundationalDiagnosticRow>,
    standard_rows: Vec<FoundationalDiagnosticRow>,
    forensic_rows: Vec<FoundationalDiagnosticRow>,
}

impl<P, T> WorthQueryDomainCapabilityTraceArtifact<P, T>
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
        required_rows: Vec<FoundationalDiagnosticRow>,
        standard_rows: Vec<FoundationalDiagnosticRow>,
        forensic_rows: Vec<FoundationalDiagnosticRow>,
    ) -> Self {
        Self {
            contribution,
            profile_progression,
            provenance,
            subject,
            primary_code,
            outcome_kind,
            required_rows,
            standard_rows,
            forensic_rows,
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

    pub fn required_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.required_rows
    }

    pub fn standard_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.standard_rows
    }

    pub fn forensic_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.forensic_rows
    }
}
