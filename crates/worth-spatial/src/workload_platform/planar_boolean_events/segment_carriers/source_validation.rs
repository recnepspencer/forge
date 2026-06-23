use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;

use super::carrier_set::PlanarBooleanSegmentCarrierOperandSource;
use super::denial::{
    PlanarBooleanSegmentCarrierSetDenial, PlanarBooleanSegmentCarrierSetDenialKind,
};

pub(super) fn validate_operand_source_slot(
    source: &PlanarBooleanSegmentCarrierOperandSource<'_>,
    expected_side: PlanarBooleanCommonPlaneOperandSide,
) -> Result<(), PlanarBooleanSegmentCarrierSetDenial> {
    if source.operand_side == expected_side {
        Ok(())
    } else {
        Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::OperandSlotSideMismatch,
            "segment carrier extraction requires left and right sources to occupy their semantic operand slots",
        ))
    }
}

pub(super) fn validate_precision_basis_identity(
    source: &PlanarBooleanSegmentCarrierOperandSource<'_>,
) -> Result<(), PlanarBooleanSegmentCarrierSetDenial> {
    if source.precision_basis_identity.trim().is_empty() {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::MissingPrecisionBasisIdentity,
            "segment carriers require the reduced-pair precision basis identity",
        ));
    }
    Ok(())
}

pub(super) fn validate_cross_operand_context(
    left: &PlanarBooleanSegmentCarrierOperandSource<'_>,
    right: &PlanarBooleanSegmentCarrierOperandSource<'_>,
) -> Result<(), PlanarBooleanSegmentCarrierSetDenial> {
    if left.precision_basis_identity != right.precision_basis_identity {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::PrecisionBasisIdentityMismatch,
            "segment carrier extraction requires both operands to use one reduced-pair precision basis",
        ));
    }

    if !operand_receipts_share_common_plane_context(left, right) {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::OperandSourceContextMismatch,
            "segment carrier extraction requires both operand sources to come from the same reduced-pair common-plane context",
        ));
    }
    Ok(())
}

pub(super) fn validate_operand_projection_source(
    source: &PlanarBooleanSegmentCarrierOperandSource<'_>,
) -> Result<(), PlanarBooleanSegmentCarrierSetDenial> {
    if source.projection_receipt.operand_side() != source.operand_side {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::ProjectionOperandSideMismatch,
            "segment carriers require the declared operand side to match the operand projection receipt",
        ));
    }
    if projected_workload_stage_identity(source)
        != source.projection_receipt.projection_stage_identity()
    {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::ProjectionStageIdentityMismatch,
            "segment carriers require the projected workload to match the operand projection receipt stage",
        ));
    }
    if projected_workload_local_basis_identity(source)
        != source.projection_receipt.projection_local_basis_identity()
    {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::ProjectionLocalBasisIdentityMismatch,
            "segment carriers require the projected workload to use the selected common-plane local basis",
        ));
    }
    Ok(())
}

fn operand_receipts_share_common_plane_context(
    left: &PlanarBooleanSegmentCarrierOperandSource<'_>,
    right: &PlanarBooleanSegmentCarrierOperandSource<'_>,
) -> bool {
    let left_receipt = left.projection_receipt;
    let right_receipt = right.projection_receipt;

    left_receipt.local_frame_selection_identity() == right_receipt.local_frame_selection_identity()
        && left_receipt.shared_plane_receipt_identity()
            == right_receipt.shared_plane_receipt_identity()
        && left_receipt.shared_plane_identity() == right_receipt.shared_plane_identity()
        && left_receipt.plane_agreement_identity() == right_receipt.plane_agreement_identity()
        && left_receipt.projection_local_basis_identity()
            == right_receipt.projection_local_basis_identity()
}

fn projected_workload_stage_identity(
    source: &PlanarBooleanSegmentCarrierOperandSource<'_>,
) -> String {
    source
        .projected_workload
        .receipts()
        .stage_identity()
        .receipt_identity()
}

fn projected_workload_local_basis_identity<'a>(
    source: &PlanarBooleanSegmentCarrierOperandSource<'a>,
) -> &'a str {
    source
        .projected_workload
        .receipts()
        .local_frame_receipt()
        .local_basis_identity()
}
