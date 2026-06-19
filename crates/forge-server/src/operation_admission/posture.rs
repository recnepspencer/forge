use crate::ForgeServerOperationRequest;

use super::{
    ForgeServerOperationAuthorityFootprint, ForgeServerOperationAuthorityMetadata,
    ForgeServerOperationAuthorizationProof, ForgeServerOperationFootprintReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationAdmissionPosture {
    operation_request: ForgeServerOperationRequest,
    authority_metadata: ForgeServerOperationAuthorityMetadata,
    authority_footprint: ForgeServerOperationAuthorityFootprint,
    footprint_receipt: ForgeServerOperationFootprintReceipt,
    authorization_proof: ForgeServerOperationAuthorizationProof,
    canonical_digest: String,
}

impl ForgeServerOperationAdmissionPosture {
    pub(crate) fn new(
        operation_request: ForgeServerOperationRequest,
        authority_metadata: ForgeServerOperationAuthorityMetadata,
        authority_footprint: ForgeServerOperationAuthorityFootprint,
        footprint_receipt: ForgeServerOperationFootprintReceipt,
        authorization_proof: ForgeServerOperationAuthorizationProof,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-operation-admission-posture-v4|metadata={}|footprint={}|receipt={}|authorization={}",
            authority_metadata.canonical_digest(),
            authority_footprint.canonical_digest(),
            footprint_receipt.canonical_digest(),
            authorization_proof.canonical_digest(),
        );
        Self {
            operation_request,
            authority_metadata,
            authority_footprint,
            footprint_receipt,
            authorization_proof,
            canonical_digest,
        }
    }

    pub fn operation_request(&self) -> &ForgeServerOperationRequest {
        &self.operation_request
    }

    pub fn authority_metadata(&self) -> &ForgeServerOperationAuthorityMetadata {
        &self.authority_metadata
    }

    pub fn authority_footprint(&self) -> &ForgeServerOperationAuthorityFootprint {
        &self.authority_footprint
    }

    pub fn footprint_receipt(&self) -> &ForgeServerOperationFootprintReceipt {
        &self.footprint_receipt
    }

    pub fn authorization_proof(&self) -> &ForgeServerOperationAuthorizationProof {
        &self.authorization_proof
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
