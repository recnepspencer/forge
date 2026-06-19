use crate::ForgeServerAdmission;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationAuthorizationProof {
    admission: ForgeServerAdmission,
    operation_reference_digest: String,
    footprint_digest: String,
    authorization_lane: String,
    canonical_digest: String,
}

impl ForgeServerOperationAuthorizationProof {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        operation_reference_digest: impl Into<String>,
        footprint_digest: impl Into<String>,
        authorization_lane: impl Into<String>,
    ) -> Self {
        let operation_reference_digest = operation_reference_digest.into();
        let footprint_digest = footprint_digest.into();
        let authorization_lane = authorization_lane.into();
        let canonical_digest = format!(
            "forge-server-operation-authorization-proof-v5|operation_reference={operation_reference_digest}|footprint={footprint_digest}|lane={authorization_lane}",
        );
        Self {
            admission,
            operation_reference_digest,
            footprint_digest,
            authorization_lane,
            canonical_digest,
        }
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn operation_reference_digest(&self) -> &str {
        &self.operation_reference_digest
    }

    pub fn authorization_lane(&self) -> &str {
        &self.authorization_lane
    }

    pub fn footprint_digest(&self) -> &str {
        &self.footprint_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
