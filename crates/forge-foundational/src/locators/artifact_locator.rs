use crate::identities::BoundaryArtifactId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundaryArtifactField {
    Payload,
    Proofs,
    Basis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundaryArtifactLocator {
    artifact_id: BoundaryArtifactId,
    field: BoundaryArtifactField,
}

impl BoundaryArtifactLocator {
    pub const fn new(artifact_id: BoundaryArtifactId, field: BoundaryArtifactField) -> Self {
        Self { artifact_id, field }
    }

    pub const fn artifact_id(&self) -> BoundaryArtifactId {
        self.artifact_id
    }

    pub const fn field(&self) -> BoundaryArtifactField {
        self.field
    }
}
