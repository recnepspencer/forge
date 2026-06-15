use crate::workload_platform::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    PlanarBooleanCommonPlaneOperandSide,
};

use super::denial::{
    PlanarBooleanCommonPlaneReducedOperandPairDenial,
    PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
};

pub(crate) fn validate_reduced_operand_pair(
    left: &PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    right: &PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
) -> Result<(), PlanarBooleanCommonPlaneReducedOperandPairDenial> {
    if left.operand_side() == right.operand_side() {
        return Err(PlanarBooleanCommonPlaneReducedOperandPairDenial::new(
            PlanarBooleanCommonPlaneReducedOperandPairDenialKind::DuplicateOperandSide,
            "reduced operand-pair assembly requires one left projection receipt and one right projection receipt",
        ));
    }
    require_side(
        left.operand_side(),
        PlanarBooleanCommonPlaneOperandSide::Left,
        PlanarBooleanCommonPlaneReducedOperandPairDenialKind::MissingLeftOperand,
        "reduced operand-pair assembly requires the left reduced slot to be backed by the left operand projection receipt",
    )?;
    require_side(
        right.operand_side(),
        PlanarBooleanCommonPlaneOperandSide::Right,
        PlanarBooleanCommonPlaneReducedOperandPairDenialKind::MissingRightOperand,
        "reduced operand-pair assembly requires the right reduced slot to be backed by the right operand projection receipt",
    )?;
    require_matching_identity(
        left.shared_plane_receipt_identity(),
        right.shared_plane_receipt_identity(),
        PlanarBooleanCommonPlaneReducedOperandPairDenialKind::SharedPlaneReceiptIdentityMismatch,
        "reduced operand-pair assembly requires both operand projections to come from the same shared-plane receipt",
    )?;
    require_matching_identity(
        left.shared_plane_identity(),
        right.shared_plane_identity(),
        PlanarBooleanCommonPlaneReducedOperandPairDenialKind::SharedPlaneIdentityMismatch,
        "reduced operand-pair assembly requires both operand projections to preserve the same shared-plane identity",
    )?;
    require_matching_identity(
        left.plane_agreement_identity(),
        right.plane_agreement_identity(),
        PlanarBooleanCommonPlaneReducedOperandPairDenialKind::PlaneAgreementIdentityMismatch,
        "reduced operand-pair assembly requires both operand projections to preserve the same plane-agreement identity",
    )?;
    require_matching_identity(
        left.local_frame_selection_identity(),
        right.local_frame_selection_identity(),
        PlanarBooleanCommonPlaneReducedOperandPairDenialKind::LocalFrameSelectionIdentityMismatch,
        "reduced operand-pair assembly requires both operand projections to come from the same local-frame selection",
    )?;
    require_matching_identity(
        left.projection_local_basis_identity(),
        right.projection_local_basis_identity(),
        PlanarBooleanCommonPlaneReducedOperandPairDenialKind::ProjectionLocalBasisIdentityMismatch,
        "reduced operand-pair assembly requires both operand projections to preserve the same projected local-basis identity",
    )?;
    Ok(())
}

fn require_side(
    actual: PlanarBooleanCommonPlaneOperandSide,
    expected: PlanarBooleanCommonPlaneOperandSide,
    kind: PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanCommonPlaneReducedOperandPairDenial> {
    if actual != expected {
        return Err(PlanarBooleanCommonPlaneReducedOperandPairDenial::new(
            kind,
            human_reason,
        ));
    }
    Ok(())
}

fn require_matching_identity(
    left: &str,
    right: &str,
    kind: PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanCommonPlaneReducedOperandPairDenial> {
    if left != right {
        return Err(PlanarBooleanCommonPlaneReducedOperandPairDenial::new(
            kind,
            human_reason,
        ));
    }
    Ok(())
}
