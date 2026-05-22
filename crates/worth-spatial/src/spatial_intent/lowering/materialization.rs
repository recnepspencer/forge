use forge_foundational::facade::{
    admit_requested_foundational_profile, boundary_evidence,
    foundational_diagnostic_boundary_artifact_subject, foundational_diagnostic_code,
    foundational_diagnostic_locator_boundary_artifact, foundational_diagnostic_scope,
    materialize_admitted_foundational_profile, materialize_diagnostic_explanation_bundle,
    materialize_diagnostic_support_report, request_foundational_profile_set, BoundaryArtifactField,
    BoundaryArtifactId, BoundaryArtifactLocator, FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalBoundaryEvidenceSourceBasis, FoundationalBoundaryEvidenceSupportContextAttachment,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDecisionRow,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticExplanationInput, FoundationalDiagnosticMaterializationDenial,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticSeverity, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSupportEvidencePosture, FoundationalDiagnosticSupportInput,
    FoundationalDiagnosticSupportReport, FoundationalDiagnosticSupportRow,
    FoundationalDiagnosticWidenedFalloutPosture, FoundationalProfileNarrowingRecord,
    FoundationalProfileProgressionDenial, FoundationalProfileSet,
    MaterializedFoundationalProfileArtifact,
};
use forge_proof::TransitionOutcome;

use crate::spatial_intent::lowering::{LoweredSpatialIntent, SpatialLoweringDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredSpatialIntentMaterializationProfilePlan {
    pub requested: FoundationalProfileSet,
    pub admitted: FoundationalProfileSet,
    pub materialized: FoundationalProfileSet,
    pub requested_to_admitted_narrowing: Option<FoundationalProfileNarrowingRecord>,
    pub admitted_to_materialized_narrowing: Option<FoundationalProfileNarrowingRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredSpatialIntentMaterializationDenial {
    ProfileProgression(FoundationalProfileProgressionDenial),
    DiagnosticMaterialization(FoundationalDiagnosticMaterializationDenial),
    ProvenanceConstruction(FoundationalBoundaryEvidenceProvenanceConstructionDenial),
    Primitive(forge_foundational::facade::FoundationalDiagnosticPrimitiveConstructionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredSpatialIntentSupportMaterialization {
    support_report: FoundationalDiagnosticSupportReport,
    explanation_bundle: FoundationalDiagnosticExplanationBundle,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    profile: MaterializedFoundationalProfileArtifact,
}

impl LoweredSpatialIntentSupportMaterialization {
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

pub fn materialize_lowered_spatial_intent_support_report(
    family: crate::spatial_intent::lowering::LoweredSpatialIntentFamily,
    outcome: Result<LoweredSpatialIntent, SpatialLoweringDenial>,
    plan: LoweredSpatialIntentMaterializationProfilePlan,
) -> Result<LoweredSpatialIntentSupportMaterialization, LoweredSpatialIntentMaterializationDenial> {
    let profile = materialize_profile(plan)?;
    let rows = rows_for_outcome(family, &outcome)?;
    let subject = foundational_diagnostic_boundary_artifact_subject(
        BoundaryArtifactId::new(2101),
        BoundaryArtifactField::Payload,
    );
    let outcome_kind = if outcome.is_ok() {
        FoundationalDiagnosticOutcomeKind::Accepted
    } else {
        FoundationalDiagnosticOutcomeKind::Denied
    };
    let support_report = materialize_diagnostic_support_report(
        FoundationalDiagnosticSupportInput::new(
            subject.clone(),
            outcome_kind,
            rows.clone(),
            Vec::new(),
            Vec::new(),
            forge_foundational::facade::FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
            FoundationalDiagnosticPartiality::Complete,
            empty_counters(),
            Vec::new(),
        ),
        *profile.payload().materialized(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .map_err(LoweredSpatialIntentMaterializationDenial::DiagnosticMaterialization)?;
    let explanation_bundle = materialize_diagnostic_explanation_bundle(
        FoundationalDiagnosticExplanationInput::new(
            subject,
            outcome_kind,
            rows,
            Vec::new(),
            Vec::new(),
            forge_foundational::facade::FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticPartiality::Complete,
            empty_counters(),
            Vec::new(),
        ),
        *profile.payload().materialized(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .map_err(LoweredSpatialIntentMaterializationDenial::DiagnosticMaterialization)?;
    Ok(LoweredSpatialIntentSupportMaterialization {
        support_report,
        explanation_bundle,
        provenance: provenance_for_outcome(&outcome)?,
        profile,
    })
}

fn materialize_profile(
    plan: LoweredSpatialIntentMaterializationProfilePlan,
) -> Result<MaterializedFoundationalProfileArtifact, LoweredSpatialIntentMaterializationDenial> {
    let requested = request_foundational_profile_set(plan.requested);
    let admitted = match admit_requested_foundational_profile(
        requested,
        plan.admitted,
        plan.requested_to_admitted_narrowing,
        forge_foundational::facade::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => {
            return Err(LoweredSpatialIntentMaterializationDenial::ProfileProgression(denial))
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
        TransitionOutcome::Denied(denial) => {
            Err(LoweredSpatialIntentMaterializationDenial::ProfileProgression(denial))
        }
        _ => unreachable!(),
    }
}

fn rows_for_outcome(
    family: crate::spatial_intent::lowering::LoweredSpatialIntentFamily,
    outcome: &Result<LoweredSpatialIntent, SpatialLoweringDenial>,
) -> Result<Vec<FoundationalDiagnosticRow>, LoweredSpatialIntentMaterializationDenial> {
    let scope = foundational_diagnostic_scope("worth.spatial.lowering")
        .map_err(LoweredSpatialIntentMaterializationDenial::Primitive)?;
    let locator = foundational_diagnostic_locator_boundary_artifact(BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(2102),
        BoundaryArtifactField::Payload,
    ));
    let code = foundational_diagnostic_code(match outcome {
        Ok(_) => "worth.spatial.lowering.accepted",
        Err(_) => "worth.spatial.lowering.denied",
    })
    .map_err(LoweredSpatialIntentMaterializationDenial::Primitive)?;
    let labels = semantic_labels_for_outcome(family, outcome)?;
    let mut rows = vec![FoundationalDiagnosticRow::Decision(
        FoundationalDiagnosticDecisionRow::new(
            code,
            scope.clone(),
            if outcome.is_ok() {
                FoundationalDiagnosticSeverity::Info
            } else {
                FoundationalDiagnosticSeverity::Denial
            },
            foundational_diagnostic_boundary_artifact_subject(
                BoundaryArtifactId::new(2101),
                BoundaryArtifactField::Payload,
            ),
            locator.clone(),
            if outcome.is_ok() {
                FoundationalDiagnosticOutcomeKind::Accepted
            } else {
                denial_outcome(*outcome.as_ref().err().expect("denial"))
            },
            forge_foundational::facade::FoundationalDiagnosticSemanticLabelSet::new(labels.clone()),
            outcome.as_ref().err().map(|_| {
                forge_foundational::facade::FoundationalDiagnosticDenialClass::DomainDenied
            }),
            forge_foundational::facade::FoundationalDiagnosticLocalityClaim::ExactSubject,
            if matches!(outcome, Ok(intent) if intent.runtime_declaration().numeric_posture() == crate::spatial_intent::lowering::LoweredSpatialNumericPosture::FallbackDerived)
            {
                FoundationalDiagnosticWidenedFalloutPosture::WidenedExpected
            } else {
                FoundationalDiagnosticWidenedFalloutPosture::NotWidened
            },
        ),
    )];
    if let Ok(intent) = outcome {
        rows.push(FoundationalDiagnosticRow::Support(FoundationalDiagnosticSupportRow::new(
            foundational_diagnostic_code("worth.spatial.lowering.support").map_err(LoweredSpatialIntentMaterializationDenial::Primitive)?,
            scope,
            FoundationalDiagnosticSeverity::Advisory,
            foundational_diagnostic_boundary_artifact_subject(BoundaryArtifactId::new(2101), BoundaryArtifactField::Payload),
            locator,
            FoundationalDiagnosticOutcomeKind::Accepted,
            forge_foundational::facade::FoundationalDiagnosticSemanticLabelSet::new(labels),
            FoundationalDiagnosticSupportEvidencePosture::Present(if intent.runtime_declaration().numeric_posture() == crate::spatial_intent::lowering::LoweredSpatialNumericPosture::FallbackDerived {
                forge_foundational::facade::FoundationalDiagnosticEvidencePosture::Reconstructed
            } else {
                forge_foundational::facade::FoundationalDiagnosticEvidencePosture::RetainedDirect
            }),
            forge_foundational::facade::FoundationalDiagnosticLocalityClaim::ExactSubject,
            if intent.runtime_declaration().numeric_posture() == crate::spatial_intent::lowering::LoweredSpatialNumericPosture::FallbackDerived { FoundationalDiagnosticWidenedFalloutPosture::WidenedExpected } else { FoundationalDiagnosticWidenedFalloutPosture::NotWidened },
        )));
    }
    Ok(rows)
}

fn provenance_for_outcome(
    outcome: &Result<LoweredSpatialIntent, SpatialLoweringDenial>,
) -> Result<FoundationalBoundaryEvidenceProvenanceArtifact, LoweredSpatialIntentMaterializationDenial>
{
    let source_basis =
        FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(if outcome.is_ok() { 2102 } else { 2100 }),
            BoundaryArtifactField::Payload,
        ));
    let mut step = boundary_evidence().provenance().current(source_basis);
    for code in support_context_codes_for_outcome(outcome)? {
        step = step.attach_support_context(
            FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_code(code),
        );
    }
    match step.with_freshness(match outcome { Ok(intent) if intent.runtime_declaration().numeric_posture() == crate::spatial_intent::lowering::LoweredSpatialNumericPosture::FallbackDerived => FoundationalBoundaryEvidenceFreshnessPosture::ReducedRetained, Ok(_) => FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained, Err(_) => FoundationalBoundaryEvidenceFreshnessPosture::ReducedRetained }) {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial) => Err(LoweredSpatialIntentMaterializationDenial::ProvenanceConstruction(denial)),
        _ => unreachable!(),
    }
}

fn family_code(
    family: crate::spatial_intent::lowering::LoweredSpatialIntentFamily,
) -> &'static str {
    match family {
        crate::spatial_intent::lowering::LoweredSpatialIntentFamily::Move => {
            "worth.spatial.lowering.move"
        }
        crate::spatial_intent::lowering::LoweredSpatialIntentFamily::Offset => {
            "worth.spatial.lowering.offset"
        }
        crate::spatial_intent::lowering::LoweredSpatialIntentFamily::Rotate => {
            "worth.spatial.lowering.rotate"
        }
        crate::spatial_intent::lowering::LoweredSpatialIntentFamily::Reorient => {
            "worth.spatial.lowering.reorient"
        }
        crate::spatial_intent::lowering::LoweredSpatialIntentFamily::LiesOn => {
            "worth.spatial.lowering.lies_on"
        }
        crate::spatial_intent::lowering::LoweredSpatialIntentFamily::PointsToward => {
            "worth.spatial.lowering.points_toward"
        }
        crate::spatial_intent::lowering::LoweredSpatialIntentFamily::AnchorMatch => {
            "worth.spatial.lowering.anchor_match"
        }
    }
}

fn denial_outcome(denial: SpatialLoweringDenial) -> FoundationalDiagnosticOutcomeKind {
    match denial {
        SpatialLoweringDenial::Unsupported => FoundationalDiagnosticOutcomeKind::Unsupported,
        _ => FoundationalDiagnosticOutcomeKind::Denied,
    }
}

fn semantic_labels_for_outcome(
    family: crate::spatial_intent::lowering::LoweredSpatialIntentFamily,
    outcome: &Result<LoweredSpatialIntent, SpatialLoweringDenial>,
) -> Result<
    Vec<forge_foundational::facade::FoundationalDiagnosticCodeId>,
    LoweredSpatialIntentMaterializationDenial,
> {
    let mut labels = vec![code(family_code(family))?];
    match outcome {
        Ok(intent) => {
            labels.push(code(
                intent.runtime_declaration().numeric_posture().as_str(),
            )?);
            labels.push(code(
                intent.runtime_declaration().target_binding().as_str(),
            )?);
            if let Some(anchor) = intent.runtime_declaration().subject_anchor() {
                labels.push(code(anchor.as_str())?);
            }
            if let Some(anchor) = intent.runtime_declaration().target_anchor() {
                labels.push(code(anchor.as_str())?);
            }
            for payload_code in intent.runtime_declaration().payload().support_codes() {
                labels.push(code(payload_code)?);
            }
        }
        Err(denial) => labels.push(code(denial_code(*denial))?),
    }
    Ok(labels)
}

fn support_context_codes_for_outcome(
    outcome: &Result<LoweredSpatialIntent, SpatialLoweringDenial>,
) -> Result<
    Vec<forge_foundational::facade::FoundationalDiagnosticCodeId>,
    LoweredSpatialIntentMaterializationDenial,
> {
    match outcome {
        Ok(intent) => {
            let mut codes = vec![code(
                intent.runtime_declaration().numeric_posture().as_str(),
            )?];
            codes.push(code(
                intent.runtime_declaration().target_binding().as_str(),
            )?);
            for payload_code in intent.runtime_declaration().payload().support_codes() {
                codes.push(code(payload_code)?);
            }
            Ok(codes)
        }
        Err(denial) => Ok(vec![code(denial_code(*denial))?]),
    }
}

fn denial_code(denial: SpatialLoweringDenial) -> &'static str {
    match denial {
        SpatialLoweringDenial::Ambiguous => "worth.spatial.lowering.denial.ambiguous",
        SpatialLoweringDenial::Unsupported => "worth.spatial.lowering.denial.unsupported",
        SpatialLoweringDenial::Undefined => "worth.spatial.lowering.denial.undefined",
        SpatialLoweringDenial::Degenerate => "worth.spatial.lowering.denial.degenerate",
        SpatialLoweringDenial::Coincident => "worth.spatial.lowering.denial.coincident",
        SpatialLoweringDenial::NonPointLike => "worth.spatial.lowering.denial.non_point_like",
        SpatialLoweringDenial::NonDirectionLike => {
            "worth.spatial.lowering.denial.non_direction_like"
        }
        SpatialLoweringDenial::WitnessFailure(_) => "worth.spatial.lowering.denial.witness_failure",
        SpatialLoweringDenial::TagFailure(_) => "worth.spatial.lowering.denial.tag_failure",
        SpatialLoweringDenial::InvalidReferenceFrame(_) => {
            "worth.spatial.lowering.denial.invalid_reference_frame"
        }
        SpatialLoweringDenial::InvalidExistingPlacement => {
            "worth.spatial.lowering.denial.invalid_existing_placement"
        }
    }
}

fn code(
    value: &'static str,
) -> Result<
    forge_foundational::facade::FoundationalDiagnosticCodeId,
    LoweredSpatialIntentMaterializationDenial,
> {
    foundational_diagnostic_code(value)
        .map_err(LoweredSpatialIntentMaterializationDenial::Primitive)
}

fn empty_counters() -> FoundationalDiagnosticCounterSnapshot {
    FoundationalDiagnosticCounterSnapshot::new(0, 0, 0, 0, 0, 0)
}

#[cfg(test)]
#[path = "lowering_materialization_tests.rs"]
mod lowering_materialization_tests;
