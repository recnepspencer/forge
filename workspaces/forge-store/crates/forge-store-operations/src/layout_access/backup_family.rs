use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase28_export_bundle_rule, AdmittedExportBundleLayoutRule, Phase28LayoutAuthorityPosture,
};

use crate::BackupExportTerminalProjectionPreparation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BackupLayoutAdmission {
    rule: AdmittedExportBundleLayoutRule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupLayoutEvidenceReport {
    admission: BackupLayoutAdmission,
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    posture: Phase28LayoutAuthorityPosture,
}

impl BackupLayoutEvidenceReport {
    pub fn from_terminal_projection(terminal: &BackupExportTerminalProjectionPreparation) -> Self {
        let _ = terminal;
        Self {
            admission: BackupLayoutAdmission {
                rule: phase28_export_bundle_rule()
                    .expect("phase-28 export bundle rule must stay admitted"),
            },
            family_id: DurableArtifactFamilyId::ExportBundle,
            access_shape: S8AccessShape::ManifestGraphWalk,
            posture: Phase28LayoutAuthorityPosture::TerminalOnly,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }
    pub const fn declared_access_shape(&self) -> S8AccessShape {
        self.access_shape
    }
    pub const fn authority_posture(&self) -> Phase28LayoutAuthorityPosture {
        self.posture
    }
    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}
