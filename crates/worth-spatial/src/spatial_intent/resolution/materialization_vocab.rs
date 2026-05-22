use forge_foundational::facade::{
    foundational_diagnostic_boundary_artifact_subject, foundational_diagnostic_code,
    foundational_diagnostic_locator_boundary_artifact, foundational_diagnostic_scope,
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalDiagnosticCodeId, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticEvidencePosture, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticScopeId, FoundationalDiagnosticSemanticLabelSet,
    FoundationalDiagnosticSeverity, FoundationalDiagnosticWidenedFalloutPosture,
};

use crate::spatial_intent::refs::{
    SpatialCatalogParameterAdmission, SpatialCatalogTrimmedAdmissionPosture,
};
use crate::spatial_intent::resolution::{
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

use super::materialization::SpatialWitnessMaterializationDenial;

#[derive(Clone, Copy)]
pub(crate) enum WitnessKind {
    Point,
    Direction,
}

pub(crate) fn requested_locator(
    kind: WitnessKind,
) -> forge_foundational::facade::FoundationalDiagnosticLocator {
    foundational_diagnostic_locator_boundary_artifact(request_boundary_artifact(kind))
}

pub(crate) fn witness_subject(
    kind: WitnessKind,
) -> forge_foundational::facade::FoundationalDiagnosticSubject {
    foundational_diagnostic_boundary_artifact_subject(
        match kind {
            WitnessKind::Point => BoundaryArtifactId::new(1101),
            WitnessKind::Direction => BoundaryArtifactId::new(1201),
        },
        BoundaryArtifactField::Payload,
    )
}

pub(crate) fn resolved_locator(
    kind: WitnessKind,
) -> forge_foundational::facade::FoundationalDiagnosticLocator {
    foundational_diagnostic_locator_boundary_artifact(match kind {
        WitnessKind::Point => BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(1102),
            BoundaryArtifactField::Payload,
        ),
        WitnessKind::Direction => BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(1202),
            BoundaryArtifactField::Payload,
        ),
    })
}

pub(crate) fn request_boundary_artifact(kind: WitnessKind) -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        match kind {
            WitnessKind::Point => BoundaryArtifactId::new(1100),
            WitnessKind::Direction => BoundaryArtifactId::new(1200),
        },
        BoundaryArtifactField::Payload,
    )
}

pub(crate) fn frame_boundary_artifact(kind: WitnessKind) -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        match kind {
            WitnessKind::Point => BoundaryArtifactId::new(1103),
            WitnessKind::Direction => BoundaryArtifactId::new(1203),
        },
        BoundaryArtifactField::Basis,
    )
}

pub(crate) fn carrier_boundary_artifact(kind: WitnessKind) -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        match kind {
            WitnessKind::Point => BoundaryArtifactId::new(1104),
            WitnessKind::Direction => BoundaryArtifactId::new(1204),
        },
        BoundaryArtifactField::Payload,
    )
}

pub(crate) fn fallback_boundary_artifact(kind: WitnessKind) -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        match kind {
            WitnessKind::Point => BoundaryArtifactId::new(1105),
            WitnessKind::Direction => BoundaryArtifactId::new(1205),
        },
        BoundaryArtifactField::Payload,
    )
}

pub(crate) fn semantic_labels(
    kind: WitnessKind,
    requested: &impl core::fmt::Debug,
    parameter_admission: Option<&SpatialCatalogParameterAdmission>,
) -> Result<FoundationalDiagnosticSemanticLabelSet, SpatialWitnessMaterializationDenial> {
    let mut labels = vec![code(match kind {
        WitnessKind::Point => "worth.spatial.witness.point",
        WitnessKind::Direction => "worth.spatial.witness.direction",
    })?];
    let debug = format!("{requested:?}");
    if debug.contains("Frame") {
        labels.push(code("worth.spatial.witness.frame")?);
    }
    if debug.contains("Carrier") || debug.contains("FeatureOwned") {
        labels.push(code("worth.spatial.witness.carrier")?);
    }
    if parameter_admission.is_some() {
        labels.push(code("worth.spatial.witness.parameter_space")?);
    }
    Ok(FoundationalDiagnosticSemanticLabelSet::new(labels))
}

pub(crate) fn code(
    value: &'static str,
) -> Result<FoundationalDiagnosticCodeId, SpatialWitnessMaterializationDenial> {
    foundational_diagnostic_code(value)
        .map_err(SpatialWitnessMaterializationDenial::DiagnosticPrimitive)
}

pub(crate) fn witness_scope(
) -> Result<FoundationalDiagnosticScopeId, SpatialWitnessMaterializationDenial> {
    foundational_diagnostic_scope("worth.spatial.witness_resolution")
        .map_err(SpatialWitnessMaterializationDenial::DiagnosticPrimitive)
}

pub(crate) fn code_for_success(
    kind: WitnessKind,
    class: SpatialWitnessResolutionClass,
) -> Result<FoundationalDiagnosticCodeId, SpatialWitnessMaterializationDenial> {
    code(match (kind, class) {
        (WitnessKind::Point, SpatialWitnessResolutionClass::DirectWorld) => {
            "worth.spatial.witness.point.accepted.direct_world"
        }
        (WitnessKind::Point, SpatialWitnessResolutionClass::FrameDerived) => {
            "worth.spatial.witness.point.accepted.frame_derived"
        }
        (WitnessKind::Point, SpatialWitnessResolutionClass::CarrierDerived) => {
            "worth.spatial.witness.point.accepted.carrier_derived"
        }
        (WitnessKind::Point, SpatialWitnessResolutionClass::FallbackDerived) => {
            "worth.spatial.witness.point.accepted.fallback_derived"
        }
        (WitnessKind::Point, SpatialWitnessResolutionClass::Exhausted) => {
            "worth.spatial.witness.point.denied.exhausted"
        }
        (WitnessKind::Direction, SpatialWitnessResolutionClass::DirectWorld) => {
            "worth.spatial.witness.direction.accepted.direct_world"
        }
        (WitnessKind::Direction, SpatialWitnessResolutionClass::FrameDerived) => {
            "worth.spatial.witness.direction.accepted.frame_derived"
        }
        (WitnessKind::Direction, SpatialWitnessResolutionClass::CarrierDerived) => {
            "worth.spatial.witness.direction.accepted.carrier_derived"
        }
        (WitnessKind::Direction, SpatialWitnessResolutionClass::FallbackDerived) => {
            "worth.spatial.witness.direction.accepted.fallback_derived"
        }
        (WitnessKind::Direction, SpatialWitnessResolutionClass::Exhausted) => {
            "worth.spatial.witness.direction.denied.exhausted"
        }
    })
}

pub(crate) fn code_for_support(
    kind: WitnessKind,
) -> Result<FoundationalDiagnosticCodeId, SpatialWitnessMaterializationDenial> {
    code(match kind {
        WitnessKind::Point => "worth.spatial.witness.point.support",
        WitnessKind::Direction => "worth.spatial.witness.direction.support",
    })
}

pub(crate) fn code_for_provenance_ready(
    kind: WitnessKind,
) -> Result<FoundationalDiagnosticCodeId, SpatialWitnessMaterializationDenial> {
    code(match kind {
        WitnessKind::Point => "worth.spatial.witness.point.provenance_ready",
        WitnessKind::Direction => "worth.spatial.witness.direction.provenance_ready",
    })
}

pub(crate) fn code_for_denial(
    kind: WitnessKind,
    denial: SpatialWitnessFailureClass,
) -> Result<FoundationalDiagnosticCodeId, SpatialWitnessMaterializationDenial> {
    code(match (kind, denial) {
        (WitnessKind::Point, SpatialWitnessFailureClass::Ambiguous) => {
            "worth.spatial.witness.point.denied.ambiguous"
        }
        (WitnessKind::Point, SpatialWitnessFailureClass::Unsupported) => {
            "worth.spatial.witness.point.denied.unsupported"
        }
        (WitnessKind::Point, SpatialWitnessFailureClass::Undefined) => {
            "worth.spatial.witness.point.denied.undefined"
        }
        (WitnessKind::Point, SpatialWitnessFailureClass::Exhausted) => {
            "worth.spatial.witness.point.denied.exhausted"
        }
        (WitnessKind::Point, SpatialWitnessFailureClass::NonFinite) => {
            "worth.spatial.witness.point.failure.non_finite"
        }
        (WitnessKind::Point, SpatialWitnessFailureClass::Degenerate) => {
            "worth.spatial.witness.point.denied.degenerate"
        }
        (WitnessKind::Point, SpatialWitnessFailureClass::Coincident) => {
            "worth.spatial.witness.point.denied.coincident"
        }
        (WitnessKind::Direction, SpatialWitnessFailureClass::Ambiguous) => {
            "worth.spatial.witness.direction.denied.ambiguous"
        }
        (WitnessKind::Direction, SpatialWitnessFailureClass::Unsupported) => {
            "worth.spatial.witness.direction.denied.unsupported"
        }
        (WitnessKind::Direction, SpatialWitnessFailureClass::Undefined) => {
            "worth.spatial.witness.direction.denied.undefined"
        }
        (WitnessKind::Direction, SpatialWitnessFailureClass::Exhausted) => {
            "worth.spatial.witness.direction.denied.exhausted"
        }
        (WitnessKind::Direction, SpatialWitnessFailureClass::NonFinite) => {
            "worth.spatial.witness.direction.failure.non_finite"
        }
        (WitnessKind::Direction, SpatialWitnessFailureClass::Degenerate) => {
            "worth.spatial.witness.direction.denied.degenerate"
        }
        (WitnessKind::Direction, SpatialWitnessFailureClass::Coincident) => {
            "worth.spatial.witness.direction.denied.coincident"
        }
    })
}

pub(crate) fn denial_severity(
    denial: SpatialWitnessFailureClass,
) -> FoundationalDiagnosticSeverity {
    if matches!(denial, SpatialWitnessFailureClass::NonFinite) {
        FoundationalDiagnosticSeverity::Failure
    } else {
        FoundationalDiagnosticSeverity::Denial
    }
}

pub(crate) fn denial_outcome(
    denial: SpatialWitnessFailureClass,
) -> FoundationalDiagnosticOutcomeKind {
    if matches!(denial, SpatialWitnessFailureClass::Unsupported) {
        FoundationalDiagnosticOutcomeKind::Unsupported
    } else if matches!(denial, SpatialWitnessFailureClass::NonFinite) {
        FoundationalDiagnosticOutcomeKind::Violation
    } else {
        FoundationalDiagnosticOutcomeKind::Denied
    }
}

pub(crate) fn denial_class(
    denial: SpatialWitnessFailureClass,
) -> Option<FoundationalDiagnosticDenialClass> {
    match denial {
        SpatialWitnessFailureClass::Unsupported => {
            Some(FoundationalDiagnosticDenialClass::UnsupportedDenied)
        }
        SpatialWitnessFailureClass::Ambiguous
        | SpatialWitnessFailureClass::Undefined
        | SpatialWitnessFailureClass::Degenerate
        | SpatialWitnessFailureClass::Coincident
        | SpatialWitnessFailureClass::Exhausted => {
            Some(FoundationalDiagnosticDenialClass::DomainDenied)
        }
        SpatialWitnessFailureClass::NonFinite => None,
    }
}

pub(crate) fn widened_posture(
    class: SpatialWitnessResolutionClass,
) -> FoundationalDiagnosticWidenedFalloutPosture {
    if matches!(class, SpatialWitnessResolutionClass::FallbackDerived) {
        FoundationalDiagnosticWidenedFalloutPosture::WidenedExpected
    } else {
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened
    }
}

pub(crate) fn evidence_posture(
    class: SpatialWitnessResolutionClass,
) -> FoundationalDiagnosticEvidencePosture {
    match class {
        SpatialWitnessResolutionClass::DirectWorld
        | SpatialWitnessResolutionClass::FrameDerived => {
            FoundationalDiagnosticEvidencePosture::RetainedDirect
        }
        SpatialWitnessResolutionClass::CarrierDerived => {
            FoundationalDiagnosticEvidencePosture::Summarized
        }
        SpatialWitnessResolutionClass::FallbackDerived => {
            FoundationalDiagnosticEvidencePosture::Reconstructed
        }
        SpatialWitnessResolutionClass::Exhausted => {
            FoundationalDiagnosticEvidencePosture::AbsentExpected
        }
    }
}

pub(crate) fn parameter_trimmed_polygonal(
    parameter_admission: Option<&SpatialCatalogParameterAdmission>,
) -> bool {
    matches!(
        parameter_admission.and_then(SpatialCatalogParameterAdmission::trimmed_posture),
        Some(SpatialCatalogTrimmedAdmissionPosture::PolygonalRegion)
    )
}
