use super::denial::{
    PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial,
    PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind,
};
use super::receipt::PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt;

pub(crate) fn validate_operand_projection_consumption(
    receipt: &PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
) -> Result<(), PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial> {
    require_identity(
        receipt.local_frame_selection_identity(),
        PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingLocalFrameSelectionIdentity,
        "operand projection consumption requires a real local-frame selection identity",
    )?;
    require_identity(
        receipt.shared_plane_receipt_identity(),
        PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingSharedPlaneReceiptIdentity,
        "operand projection consumption requires a real shared-plane receipt identity",
    )?;
    require_identity(
        receipt.shared_plane_identity(),
        PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingSharedPlaneIdentity,
        "operand projection consumption requires a real shared-plane identity",
    )?;
    require_identity(
        receipt.plane_agreement_identity(),
        PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingPlaneAgreementIdentity,
        "operand projection consumption requires a real plane-agreement identity",
    )?;
    require_identity(
        receipt.projection_stage_identity(),
        PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingProjectionStageIdentity,
        "operand projection consumption requires a real projection stage identity",
    )?;
    require_identity(
        receipt.upstream_surface_support_identity(),
        PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingUpstreamSurfaceSupportIdentity,
        "operand projection consumption requires a real upstream surface-support identity",
    )?;
    require_identity(
        receipt.certified_plane_support_identity(),
        PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingCertifiedPlaneSupportIdentity,
        "operand projection consumption requires a real certified plane-support identity",
    )?;
    require_identity(
        receipt.projection_local_basis_identity(),
        PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingProjectionLocalBasisIdentity,
        "operand projection consumption requires a real projected local-basis identity",
    )?;
    if receipt.projected_entity_count() == 0 {
        return Err(PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial::new(
            PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind::MissingProjectedEntityCount,
            "operand projection consumption requires at least one projected topology entity",
        ));
    }
    Ok(())
}

fn require_identity(
    identity: &str,
    kind: PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial> {
    if identity.trim().is_empty() {
        return Err(
            PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial::new(kind, human_reason),
        );
    }
    Ok(())
}
