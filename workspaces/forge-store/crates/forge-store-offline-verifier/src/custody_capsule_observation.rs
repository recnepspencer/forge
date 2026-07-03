use forge_store_security::{
    StoreRawSecurityScopeDeclaration, StoreSecurityScopeDeclarationProvenance,
    StoreTrustBoundaryReadmissionTrigger,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineCustodyCapsuleObservation {
    raw_declaration: StoreRawSecurityScopeDeclaration,
    readmission_trigger: StoreTrustBoundaryReadmissionTrigger,
}

impl OfflineCustodyCapsuleObservation {
    pub fn from_deserialized_capsule(
        raw_declaration: StoreRawSecurityScopeDeclaration,
        readmission_trigger: StoreTrustBoundaryReadmissionTrigger,
    ) -> Result<Self, OfflineCustodyCapsuleObservationDenial> {
        match raw_declaration.provenance() {
            StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => Ok(Self {
                raw_declaration,
                readmission_trigger,
            }),
            _ => Err(OfflineCustodyCapsuleObservationDenial::NotDeserializedRawInput),
        }
    }

    pub const fn raw_declaration(&self) -> StoreRawSecurityScopeDeclaration {
        self.raw_declaration
    }

    pub fn readmission_trigger(&self) -> StoreTrustBoundaryReadmissionTrigger {
        self.readmission_trigger.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineCustodyCapsuleObservationDenial {
    NotDeserializedRawInput,
}
