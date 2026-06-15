use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt;

use super::error::PlanarBooleanCommonPlaneSharedPlaneIdentityError;
use super::identity::identified_request_identity;
use crate::workload_composition::{
    PlanarBooleanCommonPlanePrecisionAgreedRequest, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest {
    precision_agreed_request: PlanarBooleanCommonPlanePrecisionAgreedRequest,
    identity_receipt: PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
    shared_plane_identified_request_identity: String,
}

impl PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest {
    pub fn from_precision_agreed_request(
        precision_agreed_request: PlanarBooleanCommonPlanePrecisionAgreedRequest,
    ) -> Result<Self, PlanarBooleanCommonPlaneSharedPlaneIdentityError> {
        let identity_receipt =
            PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt::from_plane_agreement(
                precision_agreed_request
                    .posture_agreed_request()
                    .plane_agreed_request()
                    .agreement_receipt(),
            );
        Self::from_parts(precision_agreed_request, identity_receipt)
    }

    pub fn from_parts(
        precision_agreed_request: PlanarBooleanCommonPlanePrecisionAgreedRequest,
        identity_receipt: PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
    ) -> Result<Self, PlanarBooleanCommonPlaneSharedPlaneIdentityError> {
        if precision_agreed_request
            .posture_agreed_request()
            .plane_agreed_request()
            .agreement_receipt()
            .agreement_identity()
            != identity_receipt.plane_agreement_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneSharedPlaneIdentityError::PlaneAgreementIdentityMismatch {
                    expected_plane_agreement_identity: precision_agreed_request
                        .posture_agreed_request()
                        .plane_agreed_request()
                        .agreement_receipt()
                        .agreement_identity()
                        .to_string(),
                    actual_plane_agreement_identity: identity_receipt
                        .plane_agreement_identity()
                        .to_string(),
                },
            );
        }
        if precision_agreed_request
            .posture_agreed_request()
            .shared_plane_identity()
            != identity_receipt.shared_plane_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneSharedPlaneIdentityError::SharedPlaneIdentityMismatch {
                    expected_shared_plane_identity: precision_agreed_request
                        .posture_agreed_request()
                        .shared_plane_identity()
                        .to_string(),
                    actual_shared_plane_identity: identity_receipt
                        .shared_plane_identity()
                        .to_string(),
                },
            );
        }

        let shared_plane_identified_request_identity =
            identified_request_identity(&precision_agreed_request, &identity_receipt);
        Ok(Self {
            precision_agreed_request,
            identity_receipt,
            shared_plane_identified_request_identity,
        })
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.precision_agreed_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.precision_agreed_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.precision_agreed_request.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        self.precision_agreed_request.request_identity()
    }

    pub fn scope_admission_identity(&self) -> &str {
        self.precision_agreed_request.scope_admission_identity()
    }

    pub fn plane_agreement_identity(&self) -> &str {
        self.precision_agreed_request.plane_agreement_identity()
    }

    pub fn posture_agreement_identity(&self) -> &str {
        self.precision_agreed_request.posture_agreement_identity()
    }

    pub fn precision_agreement_identity(&self) -> &str {
        self.precision_agreed_request.precision_agreement_identity()
    }

    pub fn shared_plane_identity(&self) -> &str {
        self.identity_receipt.shared_plane_identity()
    }

    pub fn shared_plane_receipt_identity(&self) -> &str {
        self.identity_receipt.shared_plane_receipt_identity()
    }

    pub fn shared_plane_identified_request_identity(&self) -> &str {
        &self.shared_plane_identified_request_identity
    }

    pub fn identity_receipt(&self) -> &PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt {
        &self.identity_receipt
    }

    pub fn precision_agreed_request(&self) -> &PlanarBooleanCommonPlanePrecisionAgreedRequest {
        &self.precision_agreed_request
    }
}
