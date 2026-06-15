use forge_query::facade::{
    CausalInspectionExplanationFamily, CausalInspectionMaterializationPolicy,
    CausalInspectionRichness,
};

use super::{
    PlanarDiagnosticBundleBasis, PlanarDiagnosticDenial, PlanarDiagnosticDenialKind,
    PlanarDiagnosticEvidenceKind, PlanarDiagnosticTriggerLocality,
};

pub(crate) fn validate_planar_diagnostic_bundle_basis(
    basis: &PlanarDiagnosticBundleBasis,
) -> Result<(), PlanarDiagnosticDenial> {
    if basis.subject().source_digest().trim().is_empty() {
        return Err(PlanarDiagnosticDenial::new(
            PlanarDiagnosticDenialKind::MissingDiagnosticSource,
            "planar diagnostics require a typed source receipt digest",
        ));
    }
    if basis.materialized_causal_archive_requested() {
        return Err(PlanarDiagnosticDenial::new(
            PlanarDiagnosticDenialKind::MaterializedCausalArchiveNotSupported,
            "phase 18 only admits reference-rich causal inspection, not materialized causal archives",
        ));
    }
    match basis.subject().trigger_locality() {
        PlanarDiagnosticTriggerLocality::TopologyContract => {
            require_topology_declared_surface(basis)?
        }
        PlanarDiagnosticTriggerLocality::ProjectionBasis => {
            require_projection_consumption_receipt(basis)?
        }
        PlanarDiagnosticTriggerLocality::RetainedTransformStep
        | PlanarDiagnosticTriggerLocality::MotionOrRotationPosture => {
            require_retained_transform_evidence(basis)?
        }
        PlanarDiagnosticTriggerLocality::PredicateAuthority
        | PlanarDiagnosticTriggerLocality::BindingOrRebinding
        | PlanarDiagnosticTriggerLocality::PolicyBoundary
        | PlanarDiagnosticTriggerLocality::UnsupportedPlanarClass => {}
    }
    if needs_causal_reference(basis) {
        require_reference_rich_query_causal_inspection(basis)?;
    }
    Ok(())
}

fn require_topology_declared_surface(
    basis: &PlanarDiagnosticBundleBasis,
) -> Result<(), PlanarDiagnosticDenial> {
    if basis.topology_evidence().is_some()
        || has_evidence_kind(basis, PlanarDiagnosticEvidenceKind::TopologyDeclaredSurface)
    {
        return Ok(());
    }
    Err(PlanarDiagnosticDenial::new(
        PlanarDiagnosticDenialKind::MissingTopologyDeclaredSurface,
        "topology-local diagnostics require topology declared Query surface evidence",
    ))
}

fn require_projection_consumption_receipt(
    basis: &PlanarDiagnosticBundleBasis,
) -> Result<(), PlanarDiagnosticDenial> {
    if has_evidence_kind(
        basis,
        PlanarDiagnosticEvidenceKind::ProjectionConsumptionReceipt,
    ) {
        return Ok(());
    }
    Err(PlanarDiagnosticDenial::new(
        PlanarDiagnosticDenialKind::MissingProjectionConsumptionReceipt,
        "projection-local diagnostics require projection-consumed planar fact evidence",
    ))
}

fn require_retained_transform_evidence(
    basis: &PlanarDiagnosticBundleBasis,
) -> Result<(), PlanarDiagnosticDenial> {
    if has_evidence_kind(basis, PlanarDiagnosticEvidenceKind::BasisLifecycleReceipt) {
        return Ok(());
    }
    Err(PlanarDiagnosticDenial::new(
        PlanarDiagnosticDenialKind::MissingRetainedTransformEvidence,
        "retained transform diagnostics require retained or motion basis evidence",
    ))
}

fn require_reference_rich_query_causal_inspection(
    basis: &PlanarDiagnosticBundleBasis,
) -> Result<(), PlanarDiagnosticDenial> {
    let Some(causal_evidence) = basis.causal_evidence() else {
        return Err(PlanarDiagnosticDenial::new(
            PlanarDiagnosticDenialKind::MissingCausalInspectionReference,
            "cross-runtime planar diagnostics require a Query causal inspection reference",
        ));
    };
    if causal_evidence.anchor_digest().trim().is_empty()
        || causal_evidence.reference_set_digest().trim().is_empty()
        || causal_evidence.request_digest().trim().is_empty()
        || causal_evidence.admission_digest().trim().is_empty()
    {
        return Err(PlanarDiagnosticDenial::new(
            PlanarDiagnosticDenialKind::MissingCausalInspectionReference,
            "cross-runtime planar diagnostics require Query causal anchor, reference-set, request, and admission digests",
        ));
    }
    if causal_evidence.richness() != CausalInspectionRichness::ReferenceOnly
        || causal_evidence.explanation_family()
            != CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation
        || causal_evidence.materialization_policy()
            != CausalInspectionMaterializationPolicy::DigestReferenceOnly
    {
        return Err(PlanarDiagnosticDenial::new(
            PlanarDiagnosticDenialKind::MissingCausalInspectionReference,
            "phase 18 admits cross-runtime causal inspection only at reference richness",
        ));
    }
    Ok(())
}

fn needs_causal_reference(basis: &PlanarDiagnosticBundleBasis) -> bool {
    matches!(
        basis.subject().trigger_locality(),
        PlanarDiagnosticTriggerLocality::TopologyContract
            | PlanarDiagnosticTriggerLocality::ProjectionBasis
            | PlanarDiagnosticTriggerLocality::RetainedTransformStep
            | PlanarDiagnosticTriggerLocality::MotionOrRotationPosture
    )
}

fn has_evidence_kind(
    basis: &PlanarDiagnosticBundleBasis,
    kind: PlanarDiagnosticEvidenceKind,
) -> bool {
    basis
        .subject()
        .evidence()
        .iter()
        .any(|evidence| evidence.kind() == kind)
}
