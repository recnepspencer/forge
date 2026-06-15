use worth_math::{FinitePoint3, FiniteVector3};

use super::{
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DDenial,
    ProjectPointToCertifiedPlane2DDenialKind,
};

pub(crate) fn validate_project_point_to_certified_plane_2d_basis(
    basis: &ProjectPointToCertifiedPlane2DBasis,
) -> Result<(), ProjectPointToCertifiedPlane2DDenial> {
    require_identity(
        basis.source_point_identity(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingSourcePointIdentity,
        "source point identity is required",
    )?;
    require_identity(
        basis.source_point_basis_digest(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingSourcePointBasisDigest,
        "source point basis digest is required",
    )?;
    require_identity(
        basis.local_frame_fact_digest(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingLocalFrameReceipt,
        "local-frame receipt digest is required",
    )?;
    require_identity(
        basis.movement_rotation_posture_identity(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingMovementRotationPostureIdentity,
        "movement/rotation posture identity is required",
    )?;
    require_identity(
        basis.tolerance_policy_identity(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingTolerancePolicyIdentity,
        "tolerance policy identity is required",
    )?;
    require_source_point_is_finite(basis.source_point())?;
    require_local_delta_is_finite(basis.local_delta_from_frame_origin())?;
    require_frame_basis_alignment(basis)?;
    require_admitted_movement_rotation_posture(basis)?;
    Ok(())
}

fn require_identity(
    identity: &str,
    kind: ProjectPointToCertifiedPlane2DDenialKind,
    reason: &'static str,
) -> Result<(), ProjectPointToCertifiedPlane2DDenial> {
    if identity.trim().is_empty() {
        return Err(ProjectPointToCertifiedPlane2DDenial::new(kind, reason));
    }
    Ok(())
}

fn require_source_point_is_finite(
    point: [f64; 3],
) -> Result<(), ProjectPointToCertifiedPlane2DDenial> {
    FinitePoint3::try_new(point).map(|_| ()).map_err(|_| {
        ProjectPointToCertifiedPlane2DDenial::new(
            ProjectPointToCertifiedPlane2DDenialKind::NonFiniteSourcePoint,
            "source point must be finite",
        )
    })
}

fn require_local_delta_is_finite(
    delta: [f64; 3],
) -> Result<(), ProjectPointToCertifiedPlane2DDenial> {
    FiniteVector3::try_new(delta).map(|_| ()).map_err(|_| {
        ProjectPointToCertifiedPlane2DDenial::new(
            ProjectPointToCertifiedPlane2DDenialKind::NonFiniteLocalDelta,
            "local delta from frame origin must be finite",
        )
    })
}

fn require_frame_basis_alignment(
    basis: &ProjectPointToCertifiedPlane2DBasis,
) -> Result<(), ProjectPointToCertifiedPlane2DDenial> {
    if basis.frame_identity() != basis.local_frame_snapshot().frame_identity
        || basis.transform_chain_digest() != basis.local_frame_snapshot().transform_chain_digest
        || basis.movement_rotation_posture_identity()
            != basis
                .local_frame_snapshot()
                .movement_rotation_posture_identity
        || basis.tolerance_policy_identity()
            != basis.local_frame_snapshot().tolerance_policy_identity
    {
        return Err(ProjectPointToCertifiedPlane2DDenial::new(
            ProjectPointToCertifiedPlane2DDenialKind::FrameBasisMismatch,
            "projection basis must match the consumed local-frame receipt",
        ));
    }
    Ok(())
}

fn require_admitted_movement_rotation_posture(
    basis: &ProjectPointToCertifiedPlane2DBasis,
) -> Result<(), ProjectPointToCertifiedPlane2DDenial> {
    if basis
        .movement_rotation_posture_identity()
        .contains("invalidated")
    {
        return Err(ProjectPointToCertifiedPlane2DDenial::new(
            ProjectPointToCertifiedPlane2DDenialKind::SemanticRotationInvalidatedPlanarClass,
            "movement/rotation posture invalidated the admitted planar projection class",
        ));
    }
    Ok(())
}
