use super::{
    admission_receipt_is_boolean_readiness, PlanarContractBundleDenial,
    PlanarContractBundleDenialKind, PlanarContractBundleFamily,
    PlanarContractBundleValidationBasis,
};

use super::predicate_consumption_closure::validate_predicate_consumption_closure;
use super::projection_consumption::validate_projection_consumption_closure;

pub(crate) fn validate_planar_contract_bundle_basis(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    validate_bundle_scope(basis)?;
    validate_required_receipt_vectors(basis)?;
    validate_admission_receipt(basis)?;
    validate_retained_fact_digests(basis)?;
    validate_topology_basis(basis)?;
    validate_movement_rotation_posture(basis)?;
    validate_consumed_certificate_identity_links(basis)?;
    validate_predicate_consumption_closure(basis)?;
    validate_projection_consumption_closure(basis)?;
    validate_boolean_readiness_boundary(basis)
}

fn validate_bundle_scope(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    if basis.topology_basis_identity().is_empty() {
        return Err(denial(
            PlanarContractBundleDenialKind::MissingTopologyBasis,
            None,
            "boolean-readiness validation requires an explicit topology basis",
        ));
    }
    if basis.movement_rotation_posture_identity().is_empty() {
        return Err(denial(
            PlanarContractBundleDenialKind::MissingMovementRotationPosture,
            None,
            "boolean-readiness validation requires movement/rotation posture",
        ));
    }
    if basis.diagnostic_scope_identity().is_empty() {
        return Err(denial(
            PlanarContractBundleDenialKind::MissingDiagnosticScope,
            None,
            "boolean-readiness validation requires a diagnostic scope",
        ));
    }
    Ok(())
}

fn validate_required_receipt_vectors(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    if basis.projection_receipts().is_empty() {
        return Err(denial(
            PlanarContractBundleDenialKind::MissingProjectionConsumption,
            Some(PlanarContractBundleFamily::ProjectionConsumption),
            "at least one projection-consumed point receipt is required",
        ));
    }
    if basis.predicate_receipts().is_empty() {
        return Err(denial(
            PlanarContractBundleDenialKind::MissingCertificateFamily,
            Some(PlanarContractBundleFamily::PredicateAuthority),
            "at least one exact predicate authority receipt is required",
        ));
    }
    if basis.segment_receipts().is_empty() {
        return Err(denial(
            PlanarContractBundleDenialKind::MissingCertificateFamily,
            Some(PlanarContractBundleFamily::SegmentContact),
            "at least one segment-contact receipt is required",
        ));
    }
    Ok(())
}

fn validate_admission_receipt(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    if admission_receipt_is_boolean_readiness(basis.admission_receipt()) {
        Ok(())
    } else {
        Err(denial(
            PlanarContractBundleDenialKind::MissingCertificateFamily,
            Some(PlanarContractBundleFamily::Admission),
            "admission receipt must admit PlanarContractBundle/BooleanReadinessBundle",
        ))
    }
}

fn validate_retained_fact_digests(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let required = [
        (
            PlanarContractBundleFamily::TopologyContractCompleteness,
            basis.topology_contract_receipt().fact_digest(),
        ),
        (
            PlanarContractBundleFamily::Precision,
            basis.precision_receipt().fact_digest(),
        ),
        (
            PlanarContractBundleFamily::LocalFrame,
            basis.local_frame_receipt().fact_digest(),
        ),
        (
            PlanarContractBundleFamily::PolygonWinding,
            basis.winding_receipt().fact_digest(),
        ),
        (
            PlanarContractBundleFamily::SignedArea,
            basis.signed_area_receipt().fact_digest(),
        ),
        (
            PlanarContractBundleFamily::CoplanarOverlap,
            basis.overlap_receipt().fact_digest(),
        ),
        (
            PlanarContractBundleFamily::PredicateCertificateConsumption,
            basis.predicate_consumption_receipt().fact_digest(),
        ),
    ];
    for (family, digest) in required {
        if digest.is_empty() {
            return Err(denial(
                PlanarContractBundleDenialKind::MissingRetainedFactDigest,
                Some(family),
                "required retained fact digest is empty",
            ));
        }
    }
    Ok(())
}

fn validate_topology_basis(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let expected = basis.topology_basis_identity();
    if basis
        .topology_contract_receipt()
        .basis()
        .topology_basis_identity()
        != expected
    {
        return Err(denial(
            PlanarContractBundleDenialKind::TopologyBasisMismatch,
            Some(PlanarContractBundleFamily::TopologyContractCompleteness),
            "bundle topology basis must match the topology completeness receipt",
        ));
    }
    let predicate_match = basis
        .predicate_receipts()
        .iter()
        .all(|receipt| receipt.input_basis().topology_basis_identity() == expected);
    let segment_match = basis
        .segment_receipts()
        .iter()
        .all(|receipt| receipt.basis().topology_basis_identity() == expected);
    if predicate_match
        && segment_match
        && basis.precision_receipt().basis().topology_basis_identity() == expected
    {
        Ok(())
    } else {
        Err(denial(
            PlanarContractBundleDenialKind::TopologyBasisMismatch,
            None,
            "bundle topology basis must match consumed predicate, precision, and segment receipts",
        ))
    }
}

fn validate_movement_rotation_posture(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let expected = basis.movement_rotation_posture_identity();
    let all_match = basis
        .precision_receipt()
        .basis()
        .movement_rotation_posture_identity()
        == expected
        && basis
            .local_frame_receipt()
            .basis()
            .movement_rotation_posture_identity()
            == expected
        && basis
            .projection_receipts()
            .iter()
            .all(|receipt| receipt.basis().movement_rotation_posture_identity() == expected)
        && basis
            .predicate_receipts()
            .iter()
            .all(|receipt| receipt.input_basis().movement_rotation_posture_identity() == expected)
        && basis
            .segment_receipts()
            .iter()
            .all(|receipt| receipt.basis().movement_rotation_posture_identity() == expected)
        && basis
            .winding_receipt()
            .basis()
            .movement_rotation_posture_identity()
            == expected
        && basis
            .signed_area_receipt()
            .basis()
            .movement_rotation_posture_identity()
            == expected
        && basis
            .overlap_receipt()
            .basis()
            .first_face()
            .signed_area_receipt()
            .basis()
            .movement_rotation_posture_identity()
            == expected
        && basis
            .overlap_receipt()
            .basis()
            .second_face()
            .signed_area_receipt()
            .basis()
            .movement_rotation_posture_identity()
            == expected;
    if all_match {
        Ok(())
    } else {
        Err(denial(
            PlanarContractBundleDenialKind::MismatchedMovementRotationPosture,
            None,
            "every consumed planar receipt must share the declared movement/rotation posture",
        ))
    }
}

fn validate_boolean_readiness_boundary(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    if basis.overlap_receipt().boolean_result().is_none()
        && basis.overlap_receipt().imprint_action().is_none()
    {
        Ok(())
    } else {
        Err(denial(
            PlanarContractBundleDenialKind::BooleanExecutionAlreadyPresent,
            Some(PlanarContractBundleFamily::CoplanarOverlap),
            "M6 bundle validation must stop before boolean result or imprint action",
        ))
    }
}

fn validate_consumed_certificate_identity_links(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    validate_frame_consumes_precision(basis)?;
    validate_signed_area_consumes_winding_and_precision(basis)?;
    validate_overlap_consumes_supplied_signed_area(basis)
}
fn validate_frame_consumes_precision(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    if basis.local_frame_receipt().precision_fact_digest()
        == basis.precision_receipt().fact_digest()
    {
        Ok(())
    } else {
        Err(mismatched_family(
            PlanarContractBundleFamily::LocalFrame,
            "local-frame receipt must consume the supplied precision receipt",
        ))
    }
}

fn validate_signed_area_consumes_winding_and_precision(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    if basis
        .signed_area_receipt()
        .basis()
        .winding_receipt()
        .fact_digest()
        != basis.winding_receipt().fact_digest()
    {
        return Err(mismatched_family(
            PlanarContractBundleFamily::SignedArea,
            "signed-area receipt must consume the supplied winding receipt",
        ));
    }
    if basis
        .signed_area_receipt()
        .basis()
        .precision_receipt()
        .fact_digest()
        != basis.precision_receipt().fact_digest()
    {
        return Err(mismatched_family(
            PlanarContractBundleFamily::SignedArea,
            "signed-area receipt must consume the supplied precision receipt",
        ));
    }
    Ok(())
}

fn validate_overlap_consumes_supplied_signed_area(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let supplied = basis.signed_area_receipt().fact_digest();
    let first = basis
        .overlap_receipt()
        .basis()
        .first_face()
        .signed_area_receipt()
        .fact_digest();
    let second = basis
        .overlap_receipt()
        .basis()
        .second_face()
        .signed_area_receipt()
        .fact_digest();
    if supplied == first || supplied == second {
        Ok(())
    } else {
        Err(mismatched_family(
            PlanarContractBundleFamily::CoplanarOverlap,
            "overlap receipt must consume the supplied signed-area receipt",
        ))
    }
}

fn mismatched_family(
    family: PlanarContractBundleFamily,
    reason: &'static str,
) -> PlanarContractBundleDenial {
    denial(
        PlanarContractBundleDenialKind::MismatchedCertificateFamily,
        Some(family),
        reason,
    )
}

fn denial(
    kind: PlanarContractBundleDenialKind,
    family: Option<PlanarContractBundleFamily>,
    reason: &'static str,
) -> PlanarContractBundleDenial {
    PlanarContractBundleDenial::new(kind, family, reason)
}
