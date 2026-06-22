use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePrecisionAgreementReceipt;

use super::error::PlanarBooleanCommonPlanePrecisionAgreementError;
use super::identity::agreed_request_identity;
use crate::workload_composition::{
    PlanarBooleanCommonPlanePostureAgreedRequest, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlanePrecisionAgreedRequest {
    posture_agreed_request: PlanarBooleanCommonPlanePostureAgreedRequest,
    precision_receipt: PlanarBooleanCommonPlanePrecisionAgreementReceipt,
    precision_agreement_identity: String,
}

impl PlanarBooleanCommonPlanePrecisionAgreedRequest {
    pub fn from_posture_agreed_request(
        posture_agreed_request: PlanarBooleanCommonPlanePostureAgreedRequest,
    ) -> Result<Self, PlanarBooleanCommonPlanePrecisionAgreementError> {
        let declaration = posture_agreed_request
            .plane_agreed_request()
            .admitted_request()
            .reduction_request()
            .declaration_receipt()
            .ok_or(
                PlanarBooleanCommonPlanePrecisionAgreementError::MissingBooleanDeclarationBoundary,
            )?;
        let precision_receipt =
            PlanarBooleanCommonPlanePrecisionAgreementReceipt::from_m7_readiness(
                declaration
                    .basis()
                    .readiness_receipt()
                    .m7_readiness_receipt(),
            );
        Self::from_parts(posture_agreed_request, precision_receipt)
    }

    pub fn from_parts(
        posture_agreed_request: PlanarBooleanCommonPlanePostureAgreedRequest,
        precision_receipt: PlanarBooleanCommonPlanePrecisionAgreementReceipt,
    ) -> Result<Self, PlanarBooleanCommonPlanePrecisionAgreementError> {
        let declaration = posture_agreed_request
            .plane_agreed_request()
            .admitted_request()
            .reduction_request()
            .declaration_receipt()
            .ok_or(
                PlanarBooleanCommonPlanePrecisionAgreementError::MissingBooleanDeclarationBoundary,
            )?;
        let readiness = declaration
            .basis()
            .readiness_receipt()
            .m7_readiness_receipt();

        if readiness.precision_fact_digest() != precision_receipt.precision_fact_digest() {
            return Err(
                PlanarBooleanCommonPlanePrecisionAgreementError::PrecisionFactDigestMismatch {
                    expected_precision_fact_digest: readiness.precision_fact_digest().to_string(),
                    actual_precision_fact_digest: precision_receipt
                        .precision_fact_digest()
                        .to_string(),
                },
            );
        }
        if readiness.local_frame_fact_digest() != precision_receipt.local_frame_fact_digest() {
            return Err(
                PlanarBooleanCommonPlanePrecisionAgreementError::LocalFrameFactDigestMismatch {
                    expected_local_frame_fact_digest: readiness
                        .local_frame_fact_digest()
                        .to_string(),
                    actual_local_frame_fact_digest: precision_receipt
                        .local_frame_fact_digest()
                        .to_string(),
                },
            );
        }
        if readiness.topology_basis_identity() != precision_receipt.topology_basis_identity() {
            return Err(
                PlanarBooleanCommonPlanePrecisionAgreementError::TopologyBasisIdentityMismatch {
                    expected_topology_basis_identity: readiness
                        .topology_basis_identity()
                        .to_string(),
                    actual_topology_basis_identity: precision_receipt
                        .topology_basis_identity()
                        .to_string(),
                },
            );
        }
        if readiness.movement_rotation_posture_identity()
            != precision_receipt.movement_rotation_posture_identity()
        {
            return Err(
                PlanarBooleanCommonPlanePrecisionAgreementError::MovementRotationPostureIdentityMismatch {
                    expected_movement_rotation_posture_identity: readiness
                        .movement_rotation_posture_identity()
                        .to_string(),
                    actual_movement_rotation_posture_identity: precision_receipt
                        .movement_rotation_posture_identity()
                        .to_string(),
                },
            );
        }

        let precision_agreement_identity =
            agreed_request_identity(&posture_agreed_request, &precision_receipt);
        Ok(Self {
            posture_agreed_request,
            precision_receipt,
            precision_agreement_identity,
        })
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.posture_agreed_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.posture_agreed_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.posture_agreed_request.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        self.posture_agreed_request.request_identity()
    }

    pub fn scope_admission_identity(&self) -> &str {
        self.posture_agreed_request.scope_admission_identity()
    }

    pub fn plane_agreement_identity(&self) -> &str {
        self.posture_agreed_request.plane_agreement_identity()
    }

    pub fn posture_agreement_identity(&self) -> &str {
        self.posture_agreed_request.posture_agreement_identity()
    }

    pub fn precision_agreement_identity(&self) -> &str {
        &self.precision_agreement_identity
    }

    pub fn precision_fact_digest(&self) -> &str {
        self.precision_receipt.precision_fact_digest()
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        self.precision_receipt.local_frame_fact_digest()
    }

    pub fn topology_basis_identity(&self) -> &str {
        self.precision_receipt.topology_basis_identity()
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        self.precision_receipt.movement_rotation_posture_identity()
    }

    pub fn precision_receipt(&self) -> &PlanarBooleanCommonPlanePrecisionAgreementReceipt {
        &self.precision_receipt
    }

    pub fn posture_agreed_request(&self) -> &PlanarBooleanCommonPlanePostureAgreedRequest {
        &self.posture_agreed_request
    }
}
