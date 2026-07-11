use forge_store_blob_chunks::BlobExportPublishedBundle;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase28_export_bundle_rule, AdmittedExportBundleLayoutRule, Phase28LayoutAuthorityPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExportLayoutAdmission {
    rule: AdmittedExportBundleLayoutRule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportLayoutEvidenceReport {
    admission: ExportLayoutAdmission,
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    posture: Phase28LayoutAuthorityPosture,
    declared_chunks: u64,
}

impl ExportLayoutEvidenceReport {
    pub fn from_blob_export_bundle(bundle: &BlobExportPublishedBundle) -> Self {
        Self::from_export_source(bundle.offline_declarations().len() as u64)
    }

    fn from_export_source(declared_chunks: u64) -> Self {
        Self {
            admission: ExportLayoutAdmission {
                rule: phase28_export_bundle_rule()
                    .expect("phase-28 export bundle rule must stay admitted"),
            },
            family_id: DurableArtifactFamilyId::ExportBundle,
            access_shape: S8AccessShape::ManifestGraphWalk,
            posture: Phase28LayoutAuthorityPosture::TerminalOnly,
            declared_chunks,
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

    pub const fn declared_chunks(&self) -> u64 {
        self.declared_chunks
    }

    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportLayoutFamilyHome;
