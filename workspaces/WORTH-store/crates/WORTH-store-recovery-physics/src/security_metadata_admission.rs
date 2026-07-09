use worth_store_aspect_native::StorePhysicalBoundaryWitness;
use worth_store_security::{
    StorePhysicalSecurityMetadataCarrier, StoreRawSecurityScopeDeclaration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryRootSecurityMetadataAdmission {
    metadata: StorePhysicalSecurityMetadataCarrier,
}

impl RecoveryRootSecurityMetadataAdmission {
    pub const fn from_physical_metadata(metadata: StorePhysicalSecurityMetadataCarrier) -> Self {
        Self { metadata }
    }

    pub const fn metadata(self) -> StorePhysicalSecurityMetadataCarrier {
        self.metadata
    }

    pub fn to_raw_security_scope_declaration(
        self,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> StoreRawSecurityScopeDeclaration {
        StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
            physical_witness,
            self.metadata.key_scope(),
            self.metadata.key_version_posture(),
            self.metadata.tenant_scope(),
            Some(self.metadata.authenticity_requirement()),
            Some(self.metadata.custody_posture()),
        )
    }
}
