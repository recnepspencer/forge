use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePostureAgreementReceipt;

use super::certify::certify_posture_agreement;
use super::denial::PlanarBooleanCommonPlanePostureAgreementError;
use super::identity::agreed_request_identity;
use crate::workload_composition::{
    PlanarBooleanCommonPlanePlaneAgreedRequest, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlanePostureAgreedRequest {
    plane_agreed_request: PlanarBooleanCommonPlanePlaneAgreedRequest,
    agreement_receipt: PlanarBooleanCommonPlanePostureAgreementReceipt,
    posture_agreement_identity: String,
}

impl PlanarBooleanCommonPlanePostureAgreedRequest {
    pub fn from_plane_agreed_request(
        plane_agreed_request: PlanarBooleanCommonPlanePlaneAgreedRequest,
    ) -> Result<Self, PlanarBooleanCommonPlanePostureAgreementError> {
        let agreement_receipt = certify_posture_agreement(&plane_agreed_request)?;
        let posture_agreement_identity =
            agreed_request_identity(&plane_agreed_request, &agreement_receipt);
        Ok(Self {
            plane_agreed_request,
            agreement_receipt,
            posture_agreement_identity,
        })
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.plane_agreed_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.plane_agreed_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.plane_agreed_request.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        self.plane_agreed_request.request_identity()
    }

    pub fn scope_admission_identity(&self) -> &str {
        self.plane_agreed_request.scope_admission_identity()
    }

    pub fn plane_agreement_identity(&self) -> &str {
        self.plane_agreed_request.plane_agreement_identity()
    }

    pub fn posture_agreement_identity(&self) -> &str {
        &self.posture_agreement_identity
    }

    pub fn shared_plane_identity(&self) -> &str {
        self.plane_agreed_request.shared_plane_identity()
    }

    pub fn shared_posture_identity(&self) -> &str {
        self.agreement_receipt.shared_posture_identity()
    }

    pub fn agreement_receipt(&self) -> &PlanarBooleanCommonPlanePostureAgreementReceipt {
        &self.agreement_receipt
    }

    pub fn plane_agreed_request(&self) -> &PlanarBooleanCommonPlanePlaneAgreedRequest {
        &self.plane_agreed_request
    }
}
