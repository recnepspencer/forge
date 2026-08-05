use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityRejection {
    kind: CompatibilityRejectionKind,
    family_id: ArtifactFamilyId,
    reason: String,
}
impl CompatibilityRejection {
    pub fn new(
        kind: CompatibilityRejectionKind,
        family_id: ArtifactFamilyId,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            family_id,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> CompatibilityRejectionKind {
        self.kind
    }

    pub fn store_error_kind(&self) -> StoreErrorKind {
        self.kind.store_error_kind()
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
