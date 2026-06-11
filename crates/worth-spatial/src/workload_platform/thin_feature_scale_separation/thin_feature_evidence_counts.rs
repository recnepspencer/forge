use std::collections::BTreeSet;

use super::thin_feature_policy::ThinFeatureTinyRotationPressure;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;

pub(crate) fn tiny_rotation_count(pressure: ThinFeatureTinyRotationPressure) -> usize {
    match pressure {
        ThinFeatureTinyRotationPressure::RequiredAndSupported => 1,
        ThinFeatureTinyRotationPressure::Unsupported => 0,
    }
}

pub(crate) fn precision_witnesses_cover_required_scales_and_basis(
    primary_precision: &PlanarPrecisionCertificateReceipt,
    precision_witnesses: &[&PlanarPrecisionCertificateReceipt],
    required_scales: &BTreeSet<i32>,
) -> bool {
    if precision_witnesses
        .iter()
        .any(|receipt| !shares_precision_basis(primary_precision, receipt))
    {
        return false;
    }
    let witnessed_scales = precision_witnesses
        .iter()
        .map(|receipt| receipt.basis().local_feature_scale_order())
        .collect::<BTreeSet<_>>();
    required_scales
        .iter()
        .all(|required| witnessed_scales.contains(required))
}

pub(crate) fn precision_escalation_count(
    precision_witnesses: &[&PlanarPrecisionCertificateReceipt],
) -> usize {
    let upstream_escalation_breadth = precision_witnesses
        .iter()
        .map(|receipt| receipt.counters().precision_escalation_breadth())
        .sum::<usize>();
    let witnessed_scale_count = precision_witnesses
        .iter()
        .map(|receipt| receipt.basis().local_feature_scale_order())
        .collect::<BTreeSet<_>>()
        .len();
    upstream_escalation_breadth.max(witnessed_scale_count)
}

fn shares_precision_basis(
    primary_precision: &PlanarPrecisionCertificateReceipt,
    witness: &PlanarPrecisionCertificateReceipt,
) -> bool {
    let primary_basis = primary_precision.basis();
    let witness_basis = witness.basis();
    primary_basis.local_frame_identity() == witness_basis.local_frame_identity()
        && primary_basis.topology_basis_identity() == witness_basis.topology_basis_identity()
        && primary_basis.movement_rotation_posture_identity()
            == witness_basis.movement_rotation_posture_identity()
        && primary_basis.tolerance_policy_identity() == witness_basis.tolerance_policy_identity()
        && primary_basis.world_magnitude_order() == witness_basis.world_magnitude_order()
}
