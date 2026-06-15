use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneLocalFrameSelectionReceipt;

use super::error::PlanarBooleanCommonPlaneLocalFrameSelectionError;
use super::identity::selected_request_identity;
use crate::workload_composition::{
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, WorkloadCatalogDeclarationReceipt,
    WorkloadCatalogSupportReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlaneLocalFrameSelectedRequest {
    shared_plane_identified_request: PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest,
    selection_receipt: PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
    local_frame_selection_identity: String,
}

impl PlanarBooleanCommonPlaneLocalFrameSelectedRequest {
    pub fn from_shared_plane_identified_request(
        shared_plane_identified_request: PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest,
    ) -> Result<Self, PlanarBooleanCommonPlaneLocalFrameSelectionError> {
        let declaration = shared_plane_identified_request
            .precision_agreed_request()
            .posture_agreed_request()
            .plane_agreed_request()
            .admitted_request()
            .reduction_request()
            .declaration_receipt()
            .ok_or(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::MissingBooleanDeclarationBoundary,
            )?;
        let selection_receipt =
            PlanarBooleanCommonPlaneLocalFrameSelectionReceipt::from_shared_plane_identity_and_m7_readiness(
                shared_plane_identified_request.identity_receipt(),
                declaration
                    .basis()
                    .readiness_receipt()
                    .m7_readiness_receipt(),
            )
            .map_err(|denial| {
                PlanarBooleanCommonPlaneLocalFrameSelectionError::RetainedLocalFrameSelectionDenied {
                    kind: denial.kind(),
                    human_reason: denial.human_reason(),
                }
            })?;
        Self::from_parts(shared_plane_identified_request, selection_receipt)
    }

    pub fn from_parts(
        shared_plane_identified_request: PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest,
        selection_receipt: PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
    ) -> Result<Self, PlanarBooleanCommonPlaneLocalFrameSelectionError> {
        let declaration = shared_plane_identified_request
            .precision_agreed_request()
            .posture_agreed_request()
            .plane_agreed_request()
            .admitted_request()
            .reduction_request()
            .declaration_receipt()
            .ok_or(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::MissingBooleanDeclarationBoundary,
            )?;
        let readiness = declaration
            .basis()
            .readiness_receipt()
            .m7_readiness_receipt();

        if shared_plane_identified_request.shared_plane_receipt_identity()
            != selection_receipt.shared_plane_receipt_identity()
        {
            return Err(PlanarBooleanCommonPlaneLocalFrameSelectionError::SharedPlaneReceiptIdentityMismatch {
                expected_shared_plane_receipt_identity: shared_plane_identified_request
                    .shared_plane_receipt_identity()
                    .to_string(),
                actual_shared_plane_receipt_identity: selection_receipt
                    .shared_plane_receipt_identity()
                    .to_string(),
            });
        }
        if shared_plane_identified_request.shared_plane_identity()
            != selection_receipt.shared_plane_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::SharedPlaneIdentityMismatch {
                    expected_shared_plane_identity: shared_plane_identified_request
                        .shared_plane_identity()
                        .to_string(),
                    actual_shared_plane_identity: selection_receipt
                        .shared_plane_identity()
                        .to_string(),
                },
            );
        }
        if shared_plane_identified_request
            .identity_receipt()
            .plane_agreement_identity()
            != selection_receipt.plane_agreement_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::PlaneAgreementIdentityMismatch {
                    expected_plane_agreement_identity: shared_plane_identified_request
                        .identity_receipt()
                        .plane_agreement_identity()
                        .to_string(),
                    actual_plane_agreement_identity: selection_receipt
                        .plane_agreement_identity()
                        .to_string(),
                },
            );
        }
        if readiness.local_frame_receipt().fact_digest()
            != selection_receipt.local_frame_fact_digest()
        {
            return Err(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::LocalFrameFactDigestMismatch {
                    expected_local_frame_fact_digest: readiness
                        .local_frame_receipt()
                        .fact_digest()
                        .to_string(),
                    actual_local_frame_fact_digest: selection_receipt
                        .local_frame_fact_digest()
                        .to_string(),
                },
            );
        }
        if readiness.local_frame_receipt().basis().frame_identity()
            != selection_receipt.frame_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::FrameIdentityMismatch {
                    expected_frame_identity: readiness
                        .local_frame_receipt()
                        .basis()
                        .frame_identity()
                        .to_string(),
                    actual_frame_identity: selection_receipt.frame_identity().to_string(),
                },
            );
        }
        if readiness.precision_receipt().fact_digest() != selection_receipt.precision_fact_digest()
        {
            return Err(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::PrecisionFactDigestMismatch {
                    expected_precision_fact_digest: readiness
                        .precision_receipt()
                        .fact_digest()
                        .to_string(),
                    actual_precision_fact_digest: selection_receipt
                        .precision_fact_digest()
                        .to_string(),
                },
            );
        }
        if readiness.topology_basis_identity() != selection_receipt.topology_basis_identity() {
            return Err(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::TopologyBasisIdentityMismatch {
                    expected_topology_basis_identity: readiness
                        .topology_basis_identity()
                        .to_string(),
                    actual_topology_basis_identity: selection_receipt
                        .topology_basis_identity()
                        .to_string(),
                },
            );
        }
        if readiness.movement_rotation_posture_identity()
            != selection_receipt.movement_rotation_posture_identity()
        {
            return Err(
                PlanarBooleanCommonPlaneLocalFrameSelectionError::MovementRotationPostureIdentityMismatch {
                    expected_movement_rotation_posture_identity: readiness
                        .movement_rotation_posture_identity()
                        .to_string(),
                    actual_movement_rotation_posture_identity: selection_receipt
                        .movement_rotation_posture_identity()
                        .to_string(),
                },
            );
        }

        let request = Self {
            shared_plane_identified_request,
            selection_receipt,
            local_frame_selection_identity: String::new(),
        };
        let local_frame_selection_identity =
            selected_request_identity(&request, request.selection_receipt());
        Ok(Self {
            local_frame_selection_identity,
            ..request
        })
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.shared_plane_identified_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.shared_plane_identified_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.shared_plane_identified_request.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        self.shared_plane_identified_request.request_identity()
    }

    pub fn scope_admission_identity(&self) -> &str {
        self.shared_plane_identified_request
            .scope_admission_identity()
    }

    pub fn plane_agreement_identity(&self) -> &str {
        self.shared_plane_identified_request
            .plane_agreement_identity()
    }

    pub fn posture_agreement_identity(&self) -> &str {
        self.shared_plane_identified_request
            .posture_agreement_identity()
    }

    pub fn precision_agreement_identity(&self) -> &str {
        self.shared_plane_identified_request
            .precision_agreement_identity()
    }

    pub fn shared_plane_identity(&self) -> &str {
        self.shared_plane_identified_request.shared_plane_identity()
    }

    pub fn shared_plane_receipt_identity(&self) -> &str {
        self.shared_plane_identified_request
            .shared_plane_receipt_identity()
    }

    pub fn local_frame_selection_identity(&self) -> &str {
        &self.local_frame_selection_identity
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        self.selection_receipt.local_frame_fact_digest()
    }

    pub fn frame_identity(&self) -> &str {
        self.selection_receipt.frame_identity()
    }

    pub fn precision_fact_digest(&self) -> &str {
        self.selection_receipt.precision_fact_digest()
    }

    pub fn topology_basis_identity(&self) -> &str {
        self.selection_receipt.topology_basis_identity()
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        self.selection_receipt.movement_rotation_posture_identity()
    }

    pub fn selection_receipt(&self) -> &PlanarBooleanCommonPlaneLocalFrameSelectionReceipt {
        &self.selection_receipt
    }

    pub fn shared_plane_identified_request(
        &self,
    ) -> &PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest {
        &self.shared_plane_identified_request
    }
}
