use forge_foundational::{
    foundational_diagnostic_boundary_artifact_subject, foundational_diagnostic_code,
    foundational_diagnostic_locator_boundary_artifact, foundational_diagnostic_scope,
    materialize_diagnostic_support_report, AdmissionReadinessProfile, BoundaryArtifactField,
    BoundaryArtifactId, BoundaryArtifactLocator, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile, FoundationalDiagnosticCounterSnapshot,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticEvidencePosture,
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticMaterializationDenial,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSeverity,
    FoundationalDiagnosticSubject, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSupportEvidencePosture, FoundationalDiagnosticSupportInput,
    FoundationalDiagnosticSupportReport, FoundationalDiagnosticSupportRow,
    FoundationalDiagnosticSurfaceAvailability, FoundationalDiagnosticWidenedFalloutPosture,
    FoundationalProfileSet, FoundationalProfileSetInput, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use forge_store_physical_isolation::S5IsolationEvidenceRichness;

use super::{S5EvidenceProfileCounterSet, S5ExecutedIsolationFinding, S5ExecutedIsolationOutcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5FoundationalDiagnostics {
    report: FoundationalDiagnosticSupportReport,
    required_counter_fields: S5EvidenceProfileCounterSet,
}

impl S5FoundationalDiagnostics {
    pub(crate) fn from_finding(
        finding: &S5ExecutedIsolationFinding,
    ) -> Result<Self, FoundationalDiagnosticMaterializationDenial> {
        let report = materialize_diagnostic_support_report(
            support_input(finding),
            profile_set(finding),
            FoundationalDiagnosticDeliveryClass::MustBeHot,
        )?;
        Ok(Self {
            report,
            required_counter_fields: finding.counters().profile_counter_set(),
        })
    }

    pub const fn report(&self) -> &FoundationalDiagnosticSupportReport {
        &self.report
    }

    pub const fn required_counter_fields(&self) -> S5EvidenceProfileCounterSet {
        self.required_counter_fields
    }
}

fn support_input(finding: &S5ExecutedIsolationFinding) -> FoundationalDiagnosticSupportInput {
    let subject = diagnostic_subject(finding);
    let locator = foundational_diagnostic_locator_boundary_artifact(BoundaryArtifactLocator::new(
        artifact_id(finding),
        BoundaryArtifactField::Proofs,
    ));
    FoundationalDiagnosticSupportInput::new(
        subject.clone(),
        outcome_kind(finding.outcome()),
        vec![support_row(
            "store.s5.executed-isolation.required",
            subject.clone(),
            locator.clone(),
        )],
        vec![support_row(
            "store.s5.executed-isolation.standard",
            subject.clone(),
            locator.clone(),
        )],
        vec![support_row(
            "store.s5.executed-isolation.forensic",
            subject,
            locator,
        )],
        FoundationalDiagnosticSurfaceAvailability::retained_hot(),
        FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
        FoundationalDiagnosticPartiality::Complete,
        diagnostic_counter_snapshot(finding),
        Vec::new(),
    )
}

fn support_row(
    code: &'static str,
    subject: FoundationalDiagnosticSubject,
    locator: forge_foundational::FoundationalDiagnosticLocator,
) -> FoundationalDiagnosticRow {
    let code = foundational_diagnostic_code(code).expect("static diagnostic code is valid");
    FoundationalDiagnosticRow::Support(FoundationalDiagnosticSupportRow::new(
        code.clone(),
        foundational_diagnostic_scope("store.s5.executed-isolation")
            .expect("static diagnostic scope is valid"),
        FoundationalDiagnosticSeverity::Info,
        subject,
        locator,
        FoundationalDiagnosticOutcomeKind::Accepted,
        FoundationalDiagnosticSemanticLabelSet::new([code]),
        FoundationalDiagnosticSupportEvidencePosture::Present(
            FoundationalDiagnosticEvidencePosture::RetainedDirect,
        ),
        FoundationalDiagnosticLocalityClaim::ExactSubject,
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
    ))
}

fn diagnostic_subject(finding: &S5ExecutedIsolationFinding) -> FoundationalDiagnosticSubject {
    foundational_diagnostic_boundary_artifact_subject(
        artifact_id(finding),
        BoundaryArtifactField::Proofs,
    )
}

fn artifact_id(finding: &S5ExecutedIsolationFinding) -> BoundaryArtifactId {
    let mut bytes = [0_u8; 8];
    bytes.copy_from_slice(&finding.basis().plan_digest()[..8]);
    BoundaryArtifactId::new(u64::from_le_bytes(bytes))
}

fn diagnostic_counter_snapshot(
    finding: &S5ExecutedIsolationFinding,
) -> FoundationalDiagnosticCounterSnapshot {
    let retained = if finding.profile().includes_optional_forensics() {
        3
    } else {
        1
    };
    FoundationalDiagnosticCounterSnapshot::new(retained, 0, 0, 0, 0, 0)
}

fn profile_set(finding: &S5ExecutedIsolationFinding) -> FoundationalProfileSet {
    let diagnostic_richness = match finding.profile().richness() {
        S5IsolationEvidenceRichness::MinimalRequired => {
            DiagnosticRichnessProfile::OperationalMinimal
        }
        S5IsolationEvidenceRichness::Forensic => DiagnosticRichnessProfile::Forensic,
    };
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .expect("S5 diagnostic profile is evidence-backed and retained")
}

fn outcome_kind(outcome: S5ExecutedIsolationOutcome) -> FoundationalDiagnosticOutcomeKind {
    match outcome {
        S5ExecutedIsolationOutcome::Satisfied => FoundationalDiagnosticOutcomeKind::Accepted,
        S5ExecutedIsolationOutcome::DeniedMutation => FoundationalDiagnosticOutcomeKind::Denied,
        S5ExecutedIsolationOutcome::NonClaimStabilityOnly => {
            FoundationalDiagnosticOutcomeKind::Advisory
        }
    }
}
