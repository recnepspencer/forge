use crate::planar_contracts::predicate_authority::PlanarPredicateKind;

use super::basis::{CertifiedSegmentSegment2DBasis, ProjectedEndpointSnapshot};
use super::{CertifiedSegmentSegment2DDenial, CertifiedSegmentSegment2DDenialKind};

pub(crate) fn validate_certified_segment_segment_2d_basis(
    basis: &CertifiedSegmentSegment2DBasis,
) -> Result<(), CertifiedSegmentSegment2DDenial> {
    if basis.first_segment_identity().is_empty() {
        return Err(denial(
            CertifiedSegmentSegment2DDenialKind::MissingFirstSegmentIdentity,
            "certified segment classification requires a first segment identity",
        ));
    }
    if basis.second_segment_identity().is_empty() {
        return Err(denial(
            CertifiedSegmentSegment2DDenialKind::MissingSecondSegmentIdentity,
            "certified segment classification requires a second segment identity",
        ));
    }
    if basis.topology_basis_identity().is_empty() {
        return Err(denial(
            CertifiedSegmentSegment2DDenialKind::MissingTopologyBasisIdentity,
            "certified segment classification requires an explicit topology basis",
        ));
    }
    if basis.contact_policy_identity().is_empty() {
        return Err(denial(
            CertifiedSegmentSegment2DDenialKind::MissingContactPolicyIdentity,
            "certified segment classification requires an explicit contact policy",
        ));
    }
    for endpoint in basis.endpoints() {
        validate_endpoint_present(endpoint)?;
    }
    validate_shared_projection_basis(basis)?;
    validate_segment_non_degenerate(
        basis.first_start_point_2d(),
        basis.first_end_point_2d(),
        CertifiedSegmentSegment2DDenialKind::DegenerateFirstSegment,
    )?;
    validate_segment_non_degenerate(
        basis.second_start_point_2d(),
        basis.second_end_point_2d(),
        CertifiedSegmentSegment2DDenialKind::DegenerateSecondSegment,
    )?;
    Ok(())
}

fn validate_endpoint_present(
    endpoint: &ProjectedEndpointSnapshot,
) -> Result<(), CertifiedSegmentSegment2DDenial> {
    if endpoint.projection_fact_digest.is_empty() {
        return Err(denial(
            CertifiedSegmentSegment2DDenialKind::MissingProjectionReceipt,
            "certified segment classification requires projected endpoint receipts",
        ));
    }
    Ok(())
}

fn validate_shared_projection_basis(
    basis: &CertifiedSegmentSegment2DBasis,
) -> Result<(), CertifiedSegmentSegment2DDenial> {
    let first = basis.endpoints()[0];
    for endpoint in basis.endpoints().iter().skip(1) {
        if endpoint.local_frame_fact_digest != first.local_frame_fact_digest
            || endpoint.local_frame_declaration_digest != first.local_frame_declaration_digest
            || endpoint.local_frame_envelope_digest != first.local_frame_envelope_digest
            || endpoint.frame_identity != first.frame_identity
            || endpoint.transform_chain_digest != first.transform_chain_digest
        {
            return Err(denial(
                CertifiedSegmentSegment2DDenialKind::FrameBasisMismatch,
                "all projected segment endpoints must share one certified local-frame basis",
            ));
        }
        if endpoint.movement_rotation_posture_identity != first.movement_rotation_posture_identity {
            return Err(denial(
                CertifiedSegmentSegment2DDenialKind::MovementRotationMismatch,
                "all projected segment endpoints must share movement and rotation posture",
            ));
        }
        if endpoint.tolerance_policy_identity != first.tolerance_policy_identity {
            return Err(denial(
                CertifiedSegmentSegment2DDenialKind::TolerancePolicyMismatch,
                "all projected segment endpoints must share tolerance policy",
            ));
        }
    }
    Ok(())
}

fn validate_segment_non_degenerate(
    start: [f64; 2],
    end: [f64; 2],
    kind: CertifiedSegmentSegment2DDenialKind,
) -> Result<(), CertifiedSegmentSegment2DDenial> {
    if start == end {
        return Err(denial(
            kind,
            "zero-length segments require a separate point-segment contract",
        ));
    }
    Ok(())
}

pub(crate) fn validate_orientation_receipts(
    basis: &CertifiedSegmentSegment2DBasis,
) -> Result<(), CertifiedSegmentSegment2DDenial> {
    if basis.orientations().len() != 4 {
        return Err(denial(
            CertifiedSegmentSegment2DDenialKind::MissingOrientationReceipt,
            "certified segment classification requires four orient2d receipts",
        ));
    }
    let expected = basis.expected_orientation_points();
    for (index, orientation) in basis.orientations().iter().enumerate() {
        if orientation.fact_digest.is_empty() {
            return Err(denial(
                CertifiedSegmentSegment2DDenialKind::MissingOrientationReceipt,
                "certified segment classification requires four orient2d receipts",
            ));
        }
        if orientation.predicate_kind != PlanarPredicateKind::Orient2d {
            return Err(denial(
                CertifiedSegmentSegment2DDenialKind::PredicateKindMismatch,
                "certified segment classification only consumes orient2d receipts",
            ));
        }
        if orientation.local_frame_identity != basis.frame_identity()
            || orientation.topology_basis_identity != basis.topology_basis_identity()
            || orientation.movement_rotation_posture_identity
                != basis.movement_rotation_posture_identity()
            || orientation.tolerance_policy_identity != basis.tolerance_policy_identity()
            || orientation.projected_points != expected[index]
        {
            return Err(denial(
                CertifiedSegmentSegment2DDenialKind::PredicateBasisMismatch,
                "orient2d receipt basis must match the projected segment endpoints exactly",
            ));
        }
    }
    Ok(())
}

fn denial(
    kind: CertifiedSegmentSegment2DDenialKind,
    reason: &'static str,
) -> CertifiedSegmentSegment2DDenial {
    CertifiedSegmentSegment2DDenial::new(kind, reason)
}
