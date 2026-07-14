use worth_foundational::{
    foundational_diagnostic_code, foundational_diagnostic_locator_boundary_artifact,
    foundational_diagnostic_scope, materialize_diagnostic_support_report,
    AdmissionReadinessProfile, BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    CertificationPostureProfile, CompatibilityPostureProfile, DiagnosticRichnessProfile,
    FoundationalDiagnosticAbsenceCause, FoundationalDiagnosticCertifiedCoverageClass,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticGapClass, FoundationalDiagnosticGapClosurePosture,
    FoundationalDiagnosticGapTarget, FoundationalDiagnosticLocalityClaim,
    FoundationalDiagnosticNamedGap, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSeverity,
    FoundationalDiagnosticSubject, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSupportEvidencePosture, FoundationalDiagnosticSupportInput,
    FoundationalDiagnosticSupportReport, FoundationalDiagnosticSupportRow,
    FoundationalDiagnosticSurfaceAvailability, FoundationalDiagnosticWidenedFalloutPosture,
    FoundationalProfileSet, FoundationalProfileSetInput, RetentionDeliveryProfile,
    SupportPostureProfile,
};

use super::{CoverageGapDenial, CoverageSurfaceKind, HarnessCoverageStage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalHarnessReadinessReport {
    sequence: HarnessCoverageStage,
    coverage_class: FoundationalDiagnosticCertifiedCoverageClass,
    support_report: FoundationalDiagnosticSupportReport,
}

impl PhysicalHarnessReadinessReport {
    pub fn from_coverage_gap(sequence: HarnessCoverageStage, denial: &CoverageGapDenial) -> Self {
        let gap = named_gap_for_coverage_denial(denial);
        let support_report = support_report_for_named_gap(&gap);
        Self {
            sequence,
            coverage_class: FoundationalDiagnosticCertifiedCoverageClass::PartialWithNamedGaps,
            support_report,
        }
    }

    pub const fn sequence(&self) -> HarnessCoverageStage {
        self.sequence
    }

    pub const fn coverage_class(&self) -> FoundationalDiagnosticCertifiedCoverageClass {
        self.coverage_class
    }

    pub const fn support_report(&self) -> &FoundationalDiagnosticSupportReport {
        &self.support_report
    }

    pub fn named_gaps(&self) -> &[FoundationalDiagnosticNamedGap] {
        self.support_report.named_gaps()
    }
}

fn named_gap_for_coverage_denial(denial: &CoverageGapDenial) -> FoundationalDiagnosticNamedGap {
    FoundationalDiagnosticNamedGap::new(
        FoundationalDiagnosticGapClass::CoverageOmission,
        FoundationalDiagnosticGapTarget::Locator(gap_locator(denial)),
        FoundationalDiagnosticGapClosurePosture::Denied,
    )
}

fn support_report_for_named_gap(
    gap: &FoundationalDiagnosticNamedGap,
) -> FoundationalDiagnosticSupportReport {
    materialize_diagnostic_support_report(
        FoundationalDiagnosticSupportInput::new(
            diagnostic_subject(),
            FoundationalDiagnosticOutcomeKind::Denied,
            vec![FoundationalDiagnosticRow::Support(support_row(gap))],
            vec![],
            vec![],
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
            FoundationalDiagnosticPartiality::PartialWithNamedGaps(vec![gap.clone()]),
            FoundationalDiagnosticCounterSnapshot::new(0, 0, 0, 0, 0, 0),
            vec![],
        ),
        diagnostic_profile(),
        FoundationalDiagnosticDeliveryClass::CanDefer,
    )
    .expect("coverage diagnostic support report uses a valid partial named gap")
}

fn support_row(gap: &FoundationalDiagnosticNamedGap) -> FoundationalDiagnosticSupportRow {
    FoundationalDiagnosticSupportRow::new(
        foundational_diagnostic_code("store.s45.coverage-gap").unwrap(),
        foundational_diagnostic_scope("store.s45.coverage").unwrap(),
        FoundationalDiagnosticSeverity::Denial,
        diagnostic_subject(),
        gap_locator_from_target(gap.target()),
        FoundationalDiagnosticOutcomeKind::Denied,
        FoundationalDiagnosticSemanticLabelSet::new([foundational_diagnostic_code(
            "store.s45.coverage",
        )
        .unwrap()]),
        FoundationalDiagnosticSupportEvidencePosture::Absent(
            FoundationalDiagnosticAbsenceCause::MissingEvidence,
        ),
        FoundationalDiagnosticLocalityClaim::ExactSubject,
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
    )
}

fn gap_locator_from_target(
    target: &FoundationalDiagnosticGapTarget,
) -> worth_foundational::FoundationalDiagnosticLocator {
    match target {
        FoundationalDiagnosticGapTarget::Subject(_) => gap_locator_for_surface(0),
        FoundationalDiagnosticGapTarget::Locator(locator) => locator.clone(),
    }
}

fn gap_locator(denial: &CoverageGapDenial) -> worth_foundational::FoundationalDiagnosticLocator {
    match denial {
        CoverageGapDenial::MissingRegistrationEvidence { surface }
        | CoverageGapDenial::MissingPlanBeforeDependentSurface { surface } => {
            gap_locator_for_surface(surface_token(*surface))
        }
        _ => gap_locator_for_surface(0),
    }
}

fn diagnostic_subject() -> FoundationalDiagnosticSubject {
    FoundationalDiagnosticSubject::BoundaryArtifact {
        artifact_locator: BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(11),
            BoundaryArtifactField::Proofs,
        ),
    }
}

fn gap_locator_for_surface(
    surface_token: u64,
) -> worth_foundational::FoundationalDiagnosticLocator {
    foundational_diagnostic_locator_boundary_artifact(BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(1100 + surface_token),
        BoundaryArtifactField::Proofs,
    ))
}

fn diagnostic_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .expect("coverage diagnostic profile is support-ready and evidence-backed")
}

const fn surface_token(surface: CoverageSurfaceKind) -> u64 {
    match surface {
        CoverageSurfaceKind::Scenario => 1,
        CoverageSurfaceKind::Plan => 2,
        CoverageSurfaceKind::YieldpointSchedule => 3,
        CoverageSurfaceKind::Actor => 4,
        CoverageSurfaceKind::Driver => 5,
        CoverageSurfaceKind::Oracle => 6,
        CoverageSurfaceKind::Counter => 7,
        CoverageSurfaceKind::Transcript => 8,
        CoverageSurfaceKind::MutationResult => 9,
    }
}
