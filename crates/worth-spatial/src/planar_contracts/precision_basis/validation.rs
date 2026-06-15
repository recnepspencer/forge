use super::basis::{PlanarPrecisionBasis, PredicateBasisSnapshot};
use super::{PlanarPrecisionBasisDenial, PlanarPrecisionBasisDenialKind};

const NORMALIZATION_SCALE_RELATIVE_TOLERANCE: f64 = 1.0e-12;

pub(crate) fn validate_planar_precision_basis(
    basis: &PlanarPrecisionBasis,
    predicate_basis: Option<PredicateBasisSnapshot>,
) -> Result<(), PlanarPrecisionBasisDenial> {
    require_identity(
        basis.local_frame_identity(),
        PlanarPrecisionBasisDenialKind::MissingLocalFrameIdentity,
        "local frame identity is required",
    )?;
    require_identity(
        basis.topology_basis_identity(),
        PlanarPrecisionBasisDenialKind::MissingTopologyBasisIdentity,
        "topology basis identity is required",
    )?;
    require_identity(
        basis.movement_rotation_posture_identity(),
        PlanarPrecisionBasisDenialKind::MissingMovementRotationPostureIdentity,
        "movement/rotation posture identity is required",
    )?;
    require_identity(
        basis.tolerance_policy_identity(),
        PlanarPrecisionBasisDenialKind::MissingTolerancePolicyIdentity,
        "tolerance policy identity is required",
    )?;
    require_identity(
        basis.predicate_fact_digest(),
        PlanarPrecisionBasisDenialKind::MissingPredicateReceipt,
        "predicate receipt digest is required",
    )?;
    require_finite_scale_inputs(basis)?;
    require_predicate_basis_alignment(basis, predicate_basis)?;
    Ok(())
}

fn require_identity(
    identity: &str,
    kind: PlanarPrecisionBasisDenialKind,
    reason: &'static str,
) -> Result<(), PlanarPrecisionBasisDenial> {
    if identity.trim().is_empty() {
        return Err(PlanarPrecisionBasisDenial::new(kind, reason));
    }
    Ok(())
}

fn require_finite_scale_inputs(
    basis: &PlanarPrecisionBasis,
) -> Result<(), PlanarPrecisionBasisDenial> {
    if !(-308..=308).contains(&basis.local_feature_scale_order()) {
        return Err(PlanarPrecisionBasisDenial::new(
            PlanarPrecisionBasisDenialKind::InvalidLocalFeatureScaleOrder,
            "local feature scale order must fit finite f64 decimal orders",
        ));
    }
    if !(-308..=308).contains(&basis.world_magnitude_order()) {
        return Err(PlanarPrecisionBasisDenial::new(
            PlanarPrecisionBasisDenialKind::InvalidWorldMagnitudeOrder,
            "world magnitude order must fit finite f64 decimal orders",
        ));
    }
    if !basis.normalization_scale().is_finite() || basis.normalization_scale() <= 0.0 {
        return Err(PlanarPrecisionBasisDenial::new(
            PlanarPrecisionBasisDenialKind::InvalidNormalizationScale,
            "normalization scale must be finite and positive",
        ));
    }
    require_normalization_matches_local_feature_scale(basis)?;
    if basis.scale_separation_orders() < 0 {
        return Err(PlanarPrecisionBasisDenial::new(
            PlanarPrecisionBasisDenialKind::ContradictoryScaleSeparation,
            "world magnitude order must not be smaller than local feature scale order",
        ));
    }
    Ok(())
}

fn require_normalization_matches_local_feature_scale(
    basis: &PlanarPrecisionBasis,
) -> Result<(), PlanarPrecisionBasisDenial> {
    let expected_scale = 10.0_f64.powi(basis.local_feature_scale_order());
    let ratio = basis.normalization_scale() / expected_scale;
    if (ratio - 1.0).abs() > NORMALIZATION_SCALE_RELATIVE_TOLERANCE {
        return Err(PlanarPrecisionBasisDenial::new(
            PlanarPrecisionBasisDenialKind::NormalizationScaleLocalFeatureMismatch,
            "normalization scale must match the local feature scale order",
        ));
    }
    Ok(())
}

fn require_predicate_basis_alignment(
    basis: &PlanarPrecisionBasis,
    predicate_basis: Option<PredicateBasisSnapshot>,
) -> Result<(), PlanarPrecisionBasisDenial> {
    let Some(predicate_basis) = predicate_basis else {
        return Err(PlanarPrecisionBasisDenial::new(
            PlanarPrecisionBasisDenialKind::MissingPredicateReceipt,
            "precision basis must consume a predicate receipt",
        ));
    };
    if basis.local_frame_identity() != predicate_basis.local_frame_identity
        || basis.topology_basis_identity() != predicate_basis.topology_basis_identity
        || basis.movement_rotation_posture_identity()
            != predicate_basis.movement_rotation_posture_identity
        || basis.tolerance_policy_identity() != predicate_basis.tolerance_policy_identity
    {
        return Err(PlanarPrecisionBasisDenial::new(
            PlanarPrecisionBasisDenialKind::PredicateBasisMismatch,
            "precision basis must match the consumed predicate receipt basis",
        ));
    }
    Ok(())
}
