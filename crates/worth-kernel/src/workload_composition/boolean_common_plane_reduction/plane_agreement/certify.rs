use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneAgreementWorkload;

use super::denial::PlanarBooleanCommonPlanePlaneAgreementError;
use crate::workload_composition::PlanarBooleanCommonPlaneScopeAdmittedRequest;

pub(super) fn certify_plane_agreement(
    admitted_request: &PlanarBooleanCommonPlaneScopeAdmittedRequest,
) -> Result<
    worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneAgreementReceipt,
    PlanarBooleanCommonPlanePlaneAgreementError,
> {
    let left_support = admitted_request
        .operand_pair_recipe()
        .left()
        .surface_support()
        .clone();
    let right_support = admitted_request
        .operand_pair_recipe()
        .right()
        .surface_support()
        .clone();

    PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(left_support, right_support)
        .declared(format!(
            "common-plane plane agreement for {}",
            admitted_request.declaration().query_declaration_digest()
        ))
        .certify()
        .map_err(|denial| {
            PlanarBooleanCommonPlanePlaneAgreementError::SpatialPlaneAgreementDenied {
                request_identity: admitted_request.request_identity().to_string(),
                operand_pair_identity: admitted_request.operand_pair_identity().to_string(),
                scope_admission_identity: admitted_request.scope_admission_identity().to_string(),
                denial,
            }
        })
}
