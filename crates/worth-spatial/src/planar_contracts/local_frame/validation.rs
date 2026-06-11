use worth_math::{FinitePoint3, UnitVector3};

use super::basis::{PlanarLocalFrameBasis, PrecisionBasisSnapshot};
use super::{PlanarLocalFrameDenial, PlanarLocalFrameDenialKind};

const NORMALIZATION_SCALE_RELATIVE_TOLERANCE: f64 = 1.0e-12;

pub(crate) fn validate_planar_local_frame_basis(
    basis: &PlanarLocalFrameBasis,
    precision_basis: Option<PrecisionBasisSnapshot>,
) -> Result<(), PlanarLocalFrameDenial> {
    require_identity(
        basis.frame_identity(),
        PlanarLocalFrameDenialKind::MissingFrameIdentity,
        "frame identity is required",
    )?;
    require_identity(
        basis.movement_rotation_posture_identity(),
        PlanarLocalFrameDenialKind::MissingMovementRotationPostureIdentity,
        "movement/rotation posture identity is required",
    )?;
    require_identity(
        basis.tolerance_policy_identity(),
        PlanarLocalFrameDenialKind::MissingTolerancePolicyIdentity,
        "tolerance policy identity is required",
    )?;
    require_identity(
        basis.transform_chain_digest(),
        PlanarLocalFrameDenialKind::MissingTransformChainDigest,
        "transform-chain digest is required",
    )?;
    require_identity(
        basis.precision_fact_digest(),
        PlanarLocalFrameDenialKind::MissingPrecisionReceipt,
        "precision receipt digest is required",
    )?;
    require_origin_is_finite(basis.origin())?;
    require_normal_is_finite_and_nonzero(basis.normal())?;
    require_scale_basis(basis)?;
    require_precision_basis_alignment(basis, precision_basis)?;
    require_admitted_movement_rotation_posture(basis)?;
    Ok(())
}

fn require_normal_is_finite_and_nonzero(normal: [f64; 3]) -> Result<(), PlanarLocalFrameDenial> {
    UnitVector3::try_new(normal).map(|_| ()).map_err(|_| {
        PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::InvalidNormal,
            "local frame normal must be finite and non-zero",
        )
    })
}

fn require_identity(
    identity: &str,
    kind: PlanarLocalFrameDenialKind,
    reason: &'static str,
) -> Result<(), PlanarLocalFrameDenial> {
    if identity.trim().is_empty() {
        return Err(PlanarLocalFrameDenial::new(kind, reason));
    }
    Ok(())
}

fn require_origin_is_finite(origin: [f64; 3]) -> Result<(), PlanarLocalFrameDenial> {
    FinitePoint3::try_new(origin).map(|_| ()).map_err(|_| {
        PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::NonFiniteOrigin,
            "local frame origin must be finite",
        )
    })
}

fn require_scale_basis(basis: &PlanarLocalFrameBasis) -> Result<(), PlanarLocalFrameDenial> {
    if !(-308..=308).contains(&basis.local_feature_scale_order()) {
        return Err(PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::InvalidLocalFeatureScaleOrder,
            "local feature scale order must fit finite f64 decimal orders",
        ));
    }
    if !(-308..=308).contains(&basis.world_magnitude_order()) {
        return Err(PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::InvalidWorldMagnitudeOrder,
            "world magnitude order must fit finite f64 decimal orders",
        ));
    }
    if !basis.normalization_scale().is_finite() || basis.normalization_scale() <= 0.0 {
        return Err(PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::InvalidNormalizationScale,
            "normalization scale must be finite and positive",
        ));
    }
    let expected_scale = 10.0_f64.powi(basis.local_feature_scale_order());
    let ratio = basis.normalization_scale() / expected_scale;
    if (ratio - 1.0).abs() > NORMALIZATION_SCALE_RELATIVE_TOLERANCE {
        return Err(PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::NormalizationScaleLocalFeatureMismatch,
            "normalization scale must match the local feature scale order",
        ));
    }
    if basis.scale_separation_orders() < 0 {
        return Err(PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::ContradictoryScaleSeparation,
            "world magnitude order must not be smaller than local feature scale order",
        ));
    }
    Ok(())
}

fn require_precision_basis_alignment(
    basis: &PlanarLocalFrameBasis,
    precision_basis: Option<PrecisionBasisSnapshot>,
) -> Result<(), PlanarLocalFrameDenial> {
    let Some(precision_basis) = precision_basis else {
        return Err(PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::MissingPrecisionReceipt,
            "local frame certificate must consume a precision receipt",
        ));
    };
    if basis.frame_identity() != precision_basis.local_frame_identity
        || basis.movement_rotation_posture_identity()
            != precision_basis.movement_rotation_posture_identity
        || basis.tolerance_policy_identity() != precision_basis.tolerance_policy_identity
        || basis.local_feature_scale_order() != precision_basis.local_feature_scale_order
        || basis.world_magnitude_order() != precision_basis.world_magnitude_order
        || basis.normalization_scale().to_bits() != precision_basis.normalization_scale.to_bits()
    {
        return Err(PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::PrecisionBasisMismatch,
            "local frame basis must match the consumed precision receipt basis",
        ));
    }
    Ok(())
}

fn require_admitted_movement_rotation_posture(
    basis: &PlanarLocalFrameBasis,
) -> Result<(), PlanarLocalFrameDenial> {
    if basis
        .movement_rotation_posture_identity()
        .contains("invalidated")
    {
        return Err(PlanarLocalFrameDenial::new(
            PlanarLocalFrameDenialKind::SemanticRotationInvalidatedPlanarClass,
            "movement/rotation posture invalidated the admitted planar class",
        ));
    }
    Ok(())
}
