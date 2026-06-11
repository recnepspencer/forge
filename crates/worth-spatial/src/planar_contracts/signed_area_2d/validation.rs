use super::{
    CertifiedSignedArea2DBasis, CertifiedSignedArea2DDenial, CertifiedSignedArea2DDenialKind,
};

pub(crate) fn validate_signed_area_basis(
    basis: &CertifiedSignedArea2DBasis,
) -> Result<(), CertifiedSignedArea2DDenial> {
    let precision = basis.precision_receipt().basis();
    if basis.frame_identity() != precision.local_frame_identity() {
        return Err(CertifiedSignedArea2DDenial::new(
            CertifiedSignedArea2DDenialKind::WindingPrecisionBasisMismatch,
            "signed area requires winding and precision receipts from the same local frame",
        ));
    }
    if basis.movement_rotation_posture_identity() != precision.movement_rotation_posture_identity()
    {
        return Err(CertifiedSignedArea2DDenial::new(
            CertifiedSignedArea2DDenialKind::MovementRotationMismatch,
            "signed area requires winding and precision receipts with matching movement and rotation posture",
        ));
    }
    if basis.tolerance_policy_identity() != precision.tolerance_policy_identity() {
        return Err(CertifiedSignedArea2DDenial::new(
            CertifiedSignedArea2DDenialKind::TolerancePolicyMismatch,
            "signed area requires winding and precision receipts with matching tolerance policy",
        ));
    }
    for loop_summary in basis.loops() {
        for vertex in loop_summary.vertices() {
            if !vertex.point_2d[0].is_finite() || !vertex.point_2d[1].is_finite() {
                return Err(CertifiedSignedArea2DDenial::new(
                    CertifiedSignedArea2DDenialKind::NonFiniteProjectedCoordinate,
                    "signed area only consumes finite projected local coordinates",
                ));
            }
        }
    }
    Ok(())
}
