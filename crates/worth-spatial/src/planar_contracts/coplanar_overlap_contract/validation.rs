use super::{
    CoplanarOverlapContractBasis, CoplanarOverlapDenial, CoplanarOverlapDenialBasisLocus,
    CoplanarOverlapDenialKind,
};

pub(crate) fn validate_coplanar_overlap_basis(
    basis: &CoplanarOverlapContractBasis,
) -> Result<(), CoplanarOverlapDenial> {
    if basis.planar_neighborhood_identity().is_empty() {
        return Err(denial(
            CoplanarOverlapDenialKind::MissingPlanarNeighborhood,
            CoplanarOverlapDenialBasisLocus::PlanarNeighborhood,
            "coplanar overlap extraction requires explicit planar neighborhood scope",
        ));
    }
    let first_area = basis.first_face().signed_area_receipt().basis();
    let second_area = basis.second_face().signed_area_receipt().basis();
    if first_area.planar_neighborhood_identity() != basis.planar_neighborhood_identity()
        || second_area.planar_neighborhood_identity() != basis.planar_neighborhood_identity()
    {
        return Err(denial(
            CoplanarOverlapDenialKind::MismatchedPlanarNeighborhood,
            CoplanarOverlapDenialBasisLocus::PlanarNeighborhood,
            "overlap faces must already belong to the declared planar neighborhood",
        ));
    }
    if first_area.frame_identity() != second_area.frame_identity() {
        return Err(denial(
            CoplanarOverlapDenialKind::MismatchedFrameIdentity,
            CoplanarOverlapDenialBasisLocus::FrameIdentity,
            "overlap extraction does not compare faces from different local frames",
        ));
    }
    if first_area.movement_rotation_posture_identity()
        != second_area.movement_rotation_posture_identity()
    {
        return Err(denial(
            CoplanarOverlapDenialKind::MismatchedMovementRotationPosture,
            CoplanarOverlapDenialBasisLocus::MovementRotation,
            "movement and rotation posture must match before coplanar overlap extraction",
        ));
    }
    if first_area.tolerance_policy_identity() != second_area.tolerance_policy_identity() {
        return Err(denial(
            CoplanarOverlapDenialKind::MismatchedTolerancePolicy,
            CoplanarOverlapDenialBasisLocus::TolerancePolicy,
            "tolerance policy must match before overlap rows can be retained",
        ));
    }
    Ok(())
}

fn denial(
    kind: CoplanarOverlapDenialKind,
    locus: CoplanarOverlapDenialBasisLocus,
    reason: &'static str,
) -> CoplanarOverlapDenial {
    CoplanarOverlapDenial::new(kind, locus, reason)
}
