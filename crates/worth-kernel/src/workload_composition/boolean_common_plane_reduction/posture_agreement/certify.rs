use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePostureAgreementWorkload;

use super::denial::PlanarBooleanCommonPlanePostureAgreementError;
use crate::workload_composition::PlanarBooleanCommonPlanePlaneAgreedRequest;

pub(super) fn certify_posture_agreement(
    plane_agreed_request: &PlanarBooleanCommonPlanePlaneAgreedRequest,
) -> Result<
    worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePostureAgreementReceipt,
    PlanarBooleanCommonPlanePostureAgreementError,
>{
    let left_transform_receipts = plane_agreed_request
        .admitted_request()
        .operand_pair_recipe()
        .left()
        .transform_receipts()
        .clone();
    let right_transform_receipts = plane_agreed_request
        .admitted_request()
        .operand_pair_recipe()
        .right()
        .transform_receipts()
        .clone();

    PlanarBooleanCommonPlanePostureAgreementWorkload::for_transform_receipt_pair(
        left_transform_receipts,
        right_transform_receipts,
    )
    .declared(format!(
        "common-plane posture agreement for {}",
        plane_agreed_request
            .declaration()
            .query_declaration_digest()
    ))
    .certify()
    .map_err(|denial| {
        PlanarBooleanCommonPlanePostureAgreementError::SpatialPostureAgreementDenied {
            request_identity: plane_agreed_request.request_identity().to_string(),
            operand_pair_identity: plane_agreed_request.operand_pair_identity().to_string(),
            scope_admission_identity: plane_agreed_request.scope_admission_identity().to_string(),
            plane_agreement_identity: plane_agreed_request.plane_agreement_identity().to_string(),
            denial,
        }
    })
}
