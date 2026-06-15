use crate::planar_contracts::contract_bundle::PlanarM7ReadinessReceipt;

use super::denial::{
    PlanarBooleanCommonPlaneLocalFrameSelectionDenial,
    PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind,
};
use super::receipt::PlanarBooleanCommonPlaneLocalFrameSelectionReceipt;

pub(crate) fn validate_local_frame_selection(
    receipt: &PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
    readiness: &PlanarM7ReadinessReceipt,
) -> Result<(), PlanarBooleanCommonPlaneLocalFrameSelectionDenial> {
    require_identity(
        receipt.shared_plane_receipt_identity(),
        PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::MissingSharedPlaneReceiptIdentity,
        "local-frame selection requires a real shared-plane receipt identity",
    )?;
    require_identity(
        receipt.shared_plane_identity(),
        PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::MissingSharedPlaneIdentity,
        "local-frame selection requires a real shared-plane identity",
    )?;
    require_identity(
        receipt.plane_agreement_identity(),
        PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::MissingPlaneAgreementIdentity,
        "local-frame selection requires a real plane-agreement identity",
    )?;
    require_identity(
        receipt.frame_identity(),
        PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::MissingLocalFrameIdentity,
        "local-frame selection requires a certified local-frame identity",
    )?;
    require_identity(
        receipt.topology_basis_identity(),
        PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::MissingTopologyBasisIdentity,
        "local-frame selection requires a certified topology-basis identity",
    )?;
    require_identity(
        receipt.movement_rotation_posture_identity(),
        PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::MissingMovementRotationPostureIdentity,
        "local-frame selection requires a certified movement and rotation posture identity",
    )?;
    if receipt.frame_identity() != readiness.local_frame_receipt().basis().frame_identity() {
        return Err(PlanarBooleanCommonPlaneLocalFrameSelectionDenial::new(
            PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::FrameIdentityMismatch,
            "local-frame selection must preserve the retained M7 frame identity",
        ));
    }
    if receipt.topology_basis_identity() != readiness.topology_basis_identity() {
        return Err(PlanarBooleanCommonPlaneLocalFrameSelectionDenial::new(
            PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::TopologyBasisIdentityMismatch,
            "local-frame selection must preserve the retained M7 topology-basis identity",
        ));
    }
    if receipt.precision_fact_digest() != readiness.precision_receipt().fact_digest()
        || receipt.local_frame_fact_digest() != readiness.local_frame_receipt().fact_digest()
        || readiness.local_frame_receipt().precision_fact_digest()
            != readiness.precision_receipt().fact_digest()
    {
        return Err(PlanarBooleanCommonPlaneLocalFrameSelectionDenial::new(
            PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::PrecisionFactDigestMismatch,
            "local-frame selection must preserve the retained M7 precision and local-frame fact digests",
        ));
    }
    if receipt.movement_rotation_posture_identity()
        != readiness
            .local_frame_receipt()
            .basis()
            .movement_rotation_posture_identity()
        || receipt.movement_rotation_posture_identity()
            != readiness.movement_rotation_posture_identity()
    {
        return Err(PlanarBooleanCommonPlaneLocalFrameSelectionDenial::new(
            PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::MovementRotationPostureIdentityMismatch,
            "local-frame selection must preserve the retained movement and rotation posture identity",
        ));
    }
    Ok(())
}

fn require_identity(
    identity: &str,
    kind: PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanCommonPlaneLocalFrameSelectionDenial> {
    if identity.trim().is_empty() {
        return Err(PlanarBooleanCommonPlaneLocalFrameSelectionDenial::new(
            kind,
            human_reason,
        ));
    }
    Ok(())
}
