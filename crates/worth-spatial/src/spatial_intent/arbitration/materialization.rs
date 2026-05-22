use forge_foundational::facade::{
    admit_requested_foundational_profile, boundary_evidence,
    materialize_admitted_foundational_profile, materialize_diagnostic_explanation_bundle,
    materialize_diagnostic_support_report, request_foundational_profile_set,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalBoundaryEvidenceSourceBasis, FoundationalBoundaryEvidenceSupportContextAttachment,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDecisionRow,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticExplanationInput, FoundationalDiagnosticMaterializationDenial,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticSupportClaimStrength, FoundationalDiagnosticSupportEvidencePosture,
    FoundationalDiagnosticSupportInput, FoundationalDiagnosticSupportReport,
    FoundationalDiagnosticSupportRow, FoundationalDiagnosticSurfaceAvailability,
    FoundationalProfileNarrowingRecord, FoundationalProfileProgressionDenial,
    FoundationalProfileSet, MaterializedFoundationalProfileArtifact,
};
use forge_proof::TransitionOutcome;

use super::capabilities::SpatialIntentCandidateAvailability;
use super::declared_analysis::{SpatialIntentArbitrationDeclaration, SpatialIntentEscalation};
use super::materialization_vocab::{
    arbitration_locator, arbitration_scope, arbitration_subject, blocked_capability_code,
    candidate_support_code, decision_code, decision_severity, denial_class, evidence_posture,
    request_boundary_artifact, semantic_labels, support_boundary_artifact, widened_posture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialArbitrationMaterializationProfilePlan {
    pub requested: FoundationalProfileSet,
    pub admitted: FoundationalProfileSet,
    pub materialized: FoundationalProfileSet,
    pub requested_to_admitted_narrowing: Option<FoundationalProfileNarrowingRecord>,
    pub admitted_to_materialized_narrowing: Option<FoundationalProfileNarrowingRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpatialArbitrationMaterializationDenial {
    ProfileProgression(FoundationalProfileProgressionDenial),
    DiagnosticMaterialization(FoundationalDiagnosticMaterializationDenial),
    ProvenanceConstruction(FoundationalBoundaryEvidenceProvenanceConstructionDenial),
    Primitive(forge_foundational::facade::FoundationalDiagnosticPrimitiveConstructionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialArbitrationSupportMaterialization {
    support_report: FoundationalDiagnosticSupportReport,
    explanation_bundle: FoundationalDiagnosticExplanationBundle,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    profile: MaterializedFoundationalProfileArtifact,
}

impl SpatialArbitrationSupportMaterialization {
    pub fn support_report(&self) -> &FoundationalDiagnosticSupportReport {
        &self.support_report
    }

    pub fn explanation_bundle(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.explanation_bundle
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn profile(&self) -> &MaterializedFoundationalProfileArtifact {
        &self.profile
    }
}

pub fn materialize_spatial_arbitration_support_report(
    declaration: SpatialIntentArbitrationDeclaration,
    plan: SpatialArbitrationMaterializationProfilePlan,
) -> Result<SpatialArbitrationSupportMaterialization, SpatialArbitrationMaterializationDenial> {
    let profile = materialize_profile(plan)?;
    let rows = rows_for_declaration(&declaration)?;
    let outcome_kind = outcome_kind(declaration.escalation());
    let support_report = materialize_diagnostic_support_report(
        FoundationalDiagnosticSupportInput::new(
            arbitration_subject(),
            outcome_kind,
            rows.clone(),
            Vec::new(),
            Vec::new(),
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
            FoundationalDiagnosticPartiality::Complete,
            FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
            Vec::new(),
        ),
        *profile.payload().materialized(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .map_err(SpatialArbitrationMaterializationDenial::DiagnosticMaterialization)?;
    let explanation_bundle = materialize_diagnostic_explanation_bundle(
        FoundationalDiagnosticExplanationInput::new(
            arbitration_subject(),
            outcome_kind,
            rows,
            Vec::new(),
            Vec::new(),
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticPartiality::Complete,
            FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
            Vec::new(),
        ),
        *profile.payload().materialized(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .map_err(SpatialArbitrationMaterializationDenial::DiagnosticMaterialization)?;
    Ok(SpatialArbitrationSupportMaterialization {
        support_report,
        explanation_bundle,
        provenance: provenance_for_declaration(&declaration)?,
        profile,
    })
}

fn materialize_profile(
    plan: SpatialArbitrationMaterializationProfilePlan,
) -> Result<MaterializedFoundationalProfileArtifact, SpatialArbitrationMaterializationDenial> {
    let requested = request_foundational_profile_set(plan.requested);
    let admitted = match admit_requested_foundational_profile(
        requested,
        plan.admitted,
        plan.requested_to_admitted_narrowing,
        forge_foundational::facade::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => {
            return Err(SpatialArbitrationMaterializationDenial::ProfileProgression(
                denial,
            ))
        }
        _ => unreachable!(),
    };
    match materialize_admitted_foundational_profile(
        admitted,
        plan.materialized,
        plan.admitted_to_materialized_narrowing,
        forge_foundational::facade::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial) => Err(
            SpatialArbitrationMaterializationDenial::ProfileProgression(denial),
        ),
        _ => unreachable!(),
    }
}

fn rows_for_declaration(
    declaration: &SpatialIntentArbitrationDeclaration,
) -> Result<Vec<FoundationalDiagnosticRow>, SpatialArbitrationMaterializationDenial> {
    let mut rows = vec![FoundationalDiagnosticRow::Decision(
        FoundationalDiagnosticDecisionRow::new(
            decision_code(declaration.escalation())?,
            arbitration_scope()?,
            decision_severity(declaration.escalation()),
            arbitration_subject(),
            arbitration_locator(),
            outcome_kind(declaration.escalation()),
            semantic_labels(
                declaration.conflict_class(),
                declaration.escalation(),
                declaration.policy_profile_name(),
            )?,
            denial_class(declaration.escalation()),
            forge_foundational::facade::FoundationalDiagnosticLocalityClaim::ExactSubject,
            forge_foundational::facade::FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
        ),
    )];

    for candidate in declaration.candidates() {
        let mut labels = semantic_labels(
            declaration.conflict_class(),
            declaration.escalation(),
            declaration.policy_profile_name(),
        )?
        .labels()
        .to_vec();
        labels.push(candidate_support_code(candidate.explanation())?);
        if candidate.is_baseline() {
            labels.push(
                forge_foundational::facade::foundational_diagnostic_code(
                    "worth.spatial.arbitration.candidate.baseline",
                )
                .map_err(SpatialArbitrationMaterializationDenial::Primitive)?,
            );
        }
        if candidate.is_policy_preferred() {
            labels.push(
                forge_foundational::facade::foundational_diagnostic_code(
                    "worth.spatial.arbitration.candidate.policy_preferred",
                )
                .map_err(SpatialArbitrationMaterializationDenial::Primitive)?,
            );
        }
        if let Some(capability) = candidate.blocked_capability() {
            labels.push(blocked_capability_code(capability)?);
        }
        rows.push(FoundationalDiagnosticRow::Support(
            FoundationalDiagnosticSupportRow::new(
                candidate_support_code(candidate.explanation())?,
                arbitration_scope()?,
                forge_foundational::facade::FoundationalDiagnosticSeverity::Advisory,
                arbitration_subject(),
                arbitration_locator(),
                match candidate.availability() {
                    SpatialIntentCandidateAvailability::Available => {
                        FoundationalDiagnosticOutcomeKind::Accepted
                    }
                    SpatialIntentCandidateAvailability::Blocked(_) => {
                        FoundationalDiagnosticOutcomeKind::Unsupported
                    }
                },
                forge_foundational::facade::FoundationalDiagnosticSemanticLabelSet::new(labels),
                FoundationalDiagnosticSupportEvidencePosture::Present(evidence_posture(
                    candidate.explanation(),
                )),
                forge_foundational::facade::FoundationalDiagnosticLocalityClaim::ExactSubject,
                widened_posture(candidate.explanation()),
            ),
        ));
    }
    Ok(rows)
}

fn provenance_for_declaration(
    declaration: &SpatialIntentArbitrationDeclaration,
) -> Result<FoundationalBoundaryEvidenceProvenanceArtifact, SpatialArbitrationMaterializationDenial>
{
    let source_basis = FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(
        match declaration.escalation() {
            SpatialIntentEscalation::AutoResolve(_) => request_boundary_artifact(),
            _ => support_boundary_artifact(),
        },
    );
    let freshness = match declaration.escalation() {
        SpatialIntentEscalation::AutoResolve(_) => {
            FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
        }
        _ => FoundationalBoundaryEvidenceFreshnessPosture::ReducedRetained,
    };
    let mut step = boundary_evidence().provenance().current(source_basis);
    for label in semantic_labels(
        declaration.conflict_class(),
        declaration.escalation(),
        declaration.policy_profile_name(),
    )?
    .labels()
    .iter()
    .cloned()
    {
        step = step.attach_support_context(
            FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_code(label),
        );
    }
    match step.with_freshness(freshness) {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial) => {
            Err(SpatialArbitrationMaterializationDenial::ProvenanceConstruction(denial))
        }
        _ => unreachable!(),
    }
}

fn outcome_kind(escalation: SpatialIntentEscalation) -> FoundationalDiagnosticOutcomeKind {
    match escalation {
        SpatialIntentEscalation::AutoResolve(_) => FoundationalDiagnosticOutcomeKind::Accepted,
        SpatialIntentEscalation::BlockedByMissingCapability(_) => {
            FoundationalDiagnosticOutcomeKind::Unsupported
        }
        SpatialIntentEscalation::PreserveCandidates
        | SpatialIntentEscalation::AskForClarification => FoundationalDiagnosticOutcomeKind::Denied,
    }
}

#[cfg(test)]
#[path = "materialization_tests.rs"]
mod materialization_tests;
