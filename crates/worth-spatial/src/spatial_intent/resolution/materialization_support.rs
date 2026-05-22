use forge_foundational::facade::{
    boundary_evidence, FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceProvenanceArtifact, FoundationalBoundaryEvidenceSourceBasis,
    FoundationalBoundaryEvidenceSupportContextAttachment, FoundationalDiagnosticDecisionRow,
    FoundationalDiagnosticGapClass, FoundationalDiagnosticGapClosurePosture,
    FoundationalDiagnosticGapTarget, FoundationalDiagnosticLocalityClaim,
    FoundationalDiagnosticNamedGap, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticProvenanceReadyRow,
    FoundationalDiagnosticRow, FoundationalDiagnosticSeverity,
    FoundationalDiagnosticSupportEvidencePosture, FoundationalDiagnosticSupportRow,
    FoundationalDiagnosticSurfaceAvailability, FoundationalDiagnosticWidenedFalloutPosture,
};
use forge_proof::TransitionOutcome;

use crate::spatial_intent::refs::SpatialCatalogParameterAdmission;
use crate::spatial_intent::resolution::{
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

use super::materialization::SpatialWitnessMaterializationDenial;
use super::materialization_vocab::{
    carrier_boundary_artifact, code, code_for_denial, code_for_provenance_ready, code_for_success,
    code_for_support, denial_class, denial_outcome, denial_severity, evidence_posture,
    fallback_boundary_artifact, frame_boundary_artifact, parameter_trimmed_polygonal,
    request_boundary_artifact, resolved_locator, semantic_labels, widened_posture, witness_scope,
    witness_subject, WitnessKind,
};

pub(crate) fn base_rows(
    kind: WitnessKind,
    evidence_origin_locator: forge_foundational::facade::FoundationalDiagnosticLocator,
    requested: &impl core::fmt::Debug,
    resolution_class: Result<SpatialWitnessResolutionClass, &SpatialWitnessFailureClass>,
    denial: Option<SpatialWitnessFailureClass>,
    parameter_admission: Option<&SpatialCatalogParameterAdmission>,
) -> Result<Vec<FoundationalDiagnosticRow>, SpatialWitnessMaterializationDenial> {
    let subject = witness_subject(kind);
    let locator = resolved_locator(kind);
    let mut rows = Vec::new();
    match resolution_class {
        Ok(class) => {
            rows.push(FoundationalDiagnosticRow::Decision(
                FoundationalDiagnosticDecisionRow::new(
                    code_for_success(kind, class)?,
                    witness_scope()?,
                    FoundationalDiagnosticSeverity::Info,
                    subject.clone(),
                    locator.clone(),
                    FoundationalDiagnosticOutcomeKind::Accepted,
                    semantic_labels(kind, requested, parameter_admission)?,
                    None,
                    FoundationalDiagnosticLocalityClaim::ExactSubject,
                    widened_posture(class),
                ),
            ));
            rows.push(FoundationalDiagnosticRow::ProvenanceReady(
                FoundationalDiagnosticProvenanceReadyRow::new(
                    code_for_provenance_ready(kind)?,
                    witness_scope()?,
                    FoundationalDiagnosticSeverity::Info,
                    subject.clone(),
                    locator.clone(),
                    FoundationalDiagnosticOutcomeKind::Accepted,
                    semantic_labels(kind, requested, parameter_admission)?,
                    evidence_origin_locator,
                    evidence_posture(class),
                ),
            ));
            if matches!(
                class,
                SpatialWitnessResolutionClass::CarrierDerived
                    | SpatialWitnessResolutionClass::FallbackDerived
            ) || parameter_admission.is_some()
            {
                rows.push(FoundationalDiagnosticRow::Support(
                    FoundationalDiagnosticSupportRow::new(
                        code_for_support(kind)?,
                        witness_scope()?,
                        FoundationalDiagnosticSeverity::Advisory,
                        subject,
                        locator,
                        FoundationalDiagnosticOutcomeKind::Accepted,
                        semantic_labels(kind, requested, parameter_admission)?,
                        FoundationalDiagnosticSupportEvidencePosture::Present(evidence_posture(
                            class,
                        )),
                        FoundationalDiagnosticLocalityClaim::ExactSubject,
                        widened_posture(class),
                    ),
                ));
            }
        }
        Err(denial_ref) => {
            let denial = denial.or(Some(*denial_ref)).expect("denial available");
            rows.push(FoundationalDiagnosticRow::Decision(
                FoundationalDiagnosticDecisionRow::new(
                    code_for_denial(kind, denial)?,
                    witness_scope()?,
                    denial_severity(denial),
                    subject,
                    locator,
                    denial_outcome(denial),
                    semantic_labels(kind, requested, parameter_admission)?,
                    denial_class(denial),
                    FoundationalDiagnosticLocalityClaim::ExactSubject,
                    FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
                ),
            ));
        }
    }
    Ok(rows)
}

pub(crate) fn provenance_from_outcome(
    kind: WitnessKind,
    resolution_class: Result<SpatialWitnessResolutionClass, &SpatialWitnessFailureClass>,
    parameter_admission: Option<&SpatialCatalogParameterAdmission>,
) -> Result<FoundationalBoundaryEvidenceProvenanceArtifact, SpatialWitnessMaterializationDenial> {
    let class = resolution_class.unwrap_or(SpatialWitnessResolutionClass::Exhausted);
    let source_basis = FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(match class {
        SpatialWitnessResolutionClass::DirectWorld => request_boundary_artifact(kind),
        SpatialWitnessResolutionClass::FrameDerived => frame_boundary_artifact(kind),
        SpatialWitnessResolutionClass::CarrierDerived => carrier_boundary_artifact(kind),
        SpatialWitnessResolutionClass::FallbackDerived => fallback_boundary_artifact(kind),
        SpatialWitnessResolutionClass::Exhausted => request_boundary_artifact(kind),
    });
    let freshness = match class {
        SpatialWitnessResolutionClass::FallbackDerived
        | SpatialWitnessResolutionClass::Exhausted => {
            FoundationalBoundaryEvidenceFreshnessPosture::ReducedRetained
        }
        _ => FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained,
    };
    let mut step = boundary_evidence().provenance().current(source_basis);
    for attachment in provenance_support_context(kind, parameter_admission)? {
        step = step.attach_support_context(attachment);
    }
    match step.with_freshness(freshness) {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial) => Err(
            SpatialWitnessMaterializationDenial::ProvenanceConstruction(denial),
        ),
        _ => unreachable!("provenance construction here is denial-only"),
    }
}

pub(crate) fn availability_from_outcome<T>(
    _outcome: &Result<T, SpatialWitnessFailureClass>,
) -> FoundationalDiagnosticSurfaceAvailability {
    FoundationalDiagnosticSurfaceAvailability::retained_hot()
}

pub(crate) fn partiality_from_outcome<T>(
    kind: WitnessKind,
    outcome: &Result<T, SpatialWitnessFailureClass>,
) -> FoundationalDiagnosticPartiality {
    if matches!(outcome, Err(SpatialWitnessFailureClass::Exhausted)) {
        FoundationalDiagnosticPartiality::PartialWithNamedGaps(vec![
            FoundationalDiagnosticNamedGap::new(
                FoundationalDiagnosticGapClass::SupportBreadthUnavailable,
                FoundationalDiagnosticGapTarget::Locator(resolved_locator(kind)),
                FoundationalDiagnosticGapClosurePosture::Denied,
            ),
        ])
    } else {
        FoundationalDiagnosticPartiality::Complete
    }
}

pub(crate) fn outcome_kind(
    class: Result<SpatialWitnessResolutionClass, &SpatialWitnessFailureClass>,
    denial: Option<SpatialWitnessFailureClass>,
) -> FoundationalDiagnosticOutcomeKind {
    match (class, denial) {
        (Ok(_), _) => FoundationalDiagnosticOutcomeKind::Accepted,
        (Err(SpatialWitnessFailureClass::Unsupported), _) => {
            FoundationalDiagnosticOutcomeKind::Unsupported
        }
        _ => FoundationalDiagnosticOutcomeKind::Denied,
    }
}

fn provenance_support_context(
    kind: WitnessKind,
    parameter_admission: Option<&SpatialCatalogParameterAdmission>,
) -> Result<
    Vec<FoundationalBoundaryEvidenceSupportContextAttachment>,
    SpatialWitnessMaterializationDenial,
> {
    let mut attachments = vec![
        FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_code(code(match kind {
            WitnessKind::Point => "worth.spatial.witness.point",
            WitnessKind::Direction => "worth.spatial.witness.direction",
        })?),
        FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_scope(witness_scope()?),
    ];
    if parameter_admission.is_some() {
        attachments.push(
            FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_code(code(
                "worth.spatial.witness.parameter.requested",
            )?),
        );
        attachments.push(
            FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_code(code(
                "worth.spatial.witness.parameter.domain_admitted",
            )?),
        );
        attachments.push(
            FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_code(code(
                "worth.spatial.witness.parameter.canonicalized",
            )?),
        );
    }
    if parameter_trimmed_polygonal(parameter_admission) {
        attachments.push(
            FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_code(code(
                "worth.spatial.witness.parameter.trimmed_polygonal",
            )?),
        );
    }
    Ok(attachments)
}
