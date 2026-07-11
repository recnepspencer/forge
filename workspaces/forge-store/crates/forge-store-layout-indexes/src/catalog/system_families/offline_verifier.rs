use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_offline_verifier::{
    OfflineCustodyCapsuleObservation, OfflineExportBundleObservation, OfflineLayoutReport,
    OfflineRepairBlastRadiusObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineVerifierAuthorityPosture {
    TerminalOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineVerifierAccessShape {
    FullDeclaredScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineVerifierEvidenceKind {
    ExportBundle,
    CustodyCapsule,
    RepairBlastRadius,
    LayoutReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineVerifierLayoutProjection {
    family_id: DurableArtifactFamilyId,
    access_shape: OfflineVerifierAccessShape,
    authority_posture: OfflineVerifierAuthorityPosture,
    evidence_kind: OfflineVerifierEvidenceKind,
    evidence_items: u64,
}

pub fn project_offline_export_bundle(
    observation: &OfflineExportBundleObservation,
) -> OfflineVerifierLayoutProjection {
    projection(
        OfflineVerifierEvidenceKind::ExportBundle,
        observation.declarations().len() as u64 + observation.digest_evidence_count(),
    )
}

pub fn project_offline_custody_capsule(
    observation: &OfflineCustodyCapsuleObservation,
) -> OfflineVerifierLayoutProjection {
    let _observed_evidence = (
        observation.raw_declaration(),
        observation.readmission_trigger(),
    );
    projection(OfflineVerifierEvidenceKind::CustodyCapsule, 2)
}

pub fn project_offline_repair_blast_radius(
    observation: &OfflineRepairBlastRadiusObservation,
) -> OfflineVerifierLayoutProjection {
    let _observed_evidence = (
        observation.raw_declaration(),
        observation.physical_region(),
        observation.evidence_kind(),
    );
    projection(OfflineVerifierEvidenceKind::RepairBlastRadius, 3)
}

pub fn project_offline_layout_report(
    report: &OfflineLayoutReport,
) -> OfflineVerifierLayoutProjection {
    projection(
        OfflineVerifierEvidenceKind::LayoutReport,
        report.discovered_records().len() as u64,
    )
}

impl OfflineVerifierLayoutProjection {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn declared_access_shape(&self) -> OfflineVerifierAccessShape {
        self.access_shape
    }

    pub const fn authority_posture(&self) -> OfflineVerifierAuthorityPosture {
        self.authority_posture
    }

    pub const fn evidence_kind(&self) -> OfflineVerifierEvidenceKind {
        self.evidence_kind
    }

    pub const fn evidence_items(&self) -> u64 {
        self.evidence_items
    }

    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}

const fn projection(
    evidence_kind: OfflineVerifierEvidenceKind,
    evidence_items: u64,
) -> OfflineVerifierLayoutProjection {
    OfflineVerifierLayoutProjection {
        family_id: DurableArtifactFamilyId::OfflineVerificationRecord,
        access_shape: OfflineVerifierAccessShape::FullDeclaredScan,
        authority_posture: OfflineVerifierAuthorityPosture::TerminalOnly,
        evidence_kind,
        evidence_items,
    }
}
