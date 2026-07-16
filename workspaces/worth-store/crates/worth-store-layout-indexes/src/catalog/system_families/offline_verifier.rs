use worth_store_contracts::DurableArtifactFamilyId;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineVerifierLayoutProjection {
    family_id: DurableArtifactFamilyId,
    access_shape: OfflineVerifierAccessShape,
    authority_posture: OfflineVerifierAuthorityPosture,
    evidence_kind: OfflineVerifierEvidenceKind,
    evidence_items: u64,
}

pub fn project_offline_export_bundle(evidence_items: u64) -> OfflineVerifierLayoutProjection {
    projection(OfflineVerifierEvidenceKind::ExportBundle, evidence_items)
}

pub fn project_offline_custody_capsule(evidence_items: u64) -> OfflineVerifierLayoutProjection {
    projection(OfflineVerifierEvidenceKind::CustodyCapsule, evidence_items)
}

pub fn project_offline_repair_blast_radius(evidence_items: u64) -> OfflineVerifierLayoutProjection {
    projection(
        OfflineVerifierEvidenceKind::RepairBlastRadius,
        evidence_items,
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
