use crate::BackupExportTerminalProjectionPreparation;
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_layout_indexes::observation::AccessShape;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupLayoutEvidenceReport {
    family_id: DurableArtifactFamilyId,
    access_shape: AccessShape,
}

impl BackupLayoutEvidenceReport {
    pub fn from_terminal_projection(terminal: &BackupExportTerminalProjectionPreparation) -> Self {
        let _ = terminal;
        Self {
            family_id: DurableArtifactFamilyId::ExportBundle,
            access_shape: AccessShape::ManifestGraphWalk,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }
    pub const fn declared_access_shape(&self) -> AccessShape {
        self.access_shape
    }
    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}
