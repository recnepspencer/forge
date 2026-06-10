use super::{
    PlanarMotionCancellation, PlanarMotionPostureBasis, PlanarMotionPostureDenial,
    PlanarMotionPostureDenialKind, PlanarMotionStep,
};

pub(crate) fn validate_planar_motion_posture_basis(
    basis: &PlanarMotionPostureBasis,
) -> Result<(), PlanarMotionPostureDenial> {
    reject_coordinate_only_motion_basis(basis)?;
    require_motion_steps(basis)?;
    require_rotation_basis_for_exact_cancellation(basis)?;
    reject_orientation_flip(basis)
}

fn reject_coordinate_only_motion_basis(
    basis: &PlanarMotionPostureBasis,
) -> Result<(), PlanarMotionPostureDenial> {
    if basis.final_coordinate_digest().is_some() {
        Err(denial(
            PlanarMotionPostureDenialKind::CoordinateOnlyMotionBasis,
            "final coordinates may be inspected but cannot reconstruct planar motion posture",
        ))
    } else {
        Ok(())
    }
}

fn require_motion_steps(basis: &PlanarMotionPostureBasis) -> Result<(), PlanarMotionPostureDenial> {
    if basis.steps().is_empty() {
        Err(denial(
            PlanarMotionPostureDenialKind::MissingMotionStep,
            "planar motion posture requires at least one typed motion, rotation, reorientation, or cancellation step",
        ))
    } else {
        Ok(())
    }
}

fn require_rotation_basis_for_exact_cancellation(
    basis: &PlanarMotionPostureBasis,
) -> Result<(), PlanarMotionPostureDenial> {
    if basis.cancellation() == PlanarMotionCancellation::ExactBasisReplay
        && !basis
            .steps()
            .iter()
            .any(|step| matches!(step, PlanarMotionStep::ExactRotation { .. }))
    {
        Err(denial(
            PlanarMotionPostureDenialKind::ExactCancellationMissingRotation,
            "exact cancellation replay requires at least one exact rotation step",
        ))
    } else {
        Ok(())
    }
}

fn reject_orientation_flip(
    basis: &PlanarMotionPostureBasis,
) -> Result<(), PlanarMotionPostureDenial> {
    if basis
        .reorientation_steps()
        .any(|posture| posture.invalidates_planar_basis())
    {
        Err(denial(
            PlanarMotionPostureDenialKind::OrientationFlipInvalidatesPlanarBasis,
            "orientation reversal invalidates the retained planar basis before projection consumption",
        ))
    } else {
        Ok(())
    }
}

fn denial(kind: PlanarMotionPostureDenialKind, reason: &'static str) -> PlanarMotionPostureDenial {
    PlanarMotionPostureDenial::new(kind, reason)
}
