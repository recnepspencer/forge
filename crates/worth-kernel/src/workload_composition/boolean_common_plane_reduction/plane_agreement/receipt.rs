use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneAgreementReceipt;

use super::certify::certify_plane_agreement;
use super::denial::PlanarBooleanCommonPlanePlaneAgreementError;
use super::identity::agreed_request_identity;
use crate::workload_composition::{
    PlanarBooleanCommonPlaneScopeAdmittedRequest, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlanePlaneAgreedRequest {
    admitted_request: PlanarBooleanCommonPlaneScopeAdmittedRequest,
    agreement_receipt: PlanarBooleanCommonPlaneAgreementReceipt,
    plane_agreement_identity: String,
}

impl PlanarBooleanCommonPlanePlaneAgreedRequest {
    pub fn from_scope_admitted_request(
        admitted_request: PlanarBooleanCommonPlaneScopeAdmittedRequest,
    ) -> Result<Self, PlanarBooleanCommonPlanePlaneAgreementError> {
        let agreement_receipt = certify_plane_agreement(&admitted_request)?;
        let plane_agreement_identity =
            agreed_request_identity(&admitted_request, &agreement_receipt);
        Ok(Self {
            admitted_request,
            agreement_receipt,
            plane_agreement_identity,
        })
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.admitted_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.admitted_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.admitted_request.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        self.admitted_request.request_identity()
    }

    pub fn scope_admission_identity(&self) -> &str {
        self.admitted_request.scope_admission_identity()
    }

    pub fn plane_agreement_identity(&self) -> &str {
        &self.plane_agreement_identity
    }

    pub fn shared_plane_identity(&self) -> &str {
        self.agreement_receipt.shared_plane_identity()
    }

    pub fn agreement_receipt(&self) -> &PlanarBooleanCommonPlaneAgreementReceipt {
        &self.agreement_receipt
    }

    pub fn admitted_request(&self) -> &PlanarBooleanCommonPlaneScopeAdmittedRequest {
        &self.admitted_request
    }
}
