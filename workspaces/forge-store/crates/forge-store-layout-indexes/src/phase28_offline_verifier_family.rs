use crate::AdmittedOfflineVerifierLayoutRule;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_offline_verifier::{
    OfflineCustodyCapsuleObservation, OfflineExportBundleObservation,
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
struct OfflineVerifierLayoutAdmission {
    rule: AdmittedOfflineVerifierLayoutRule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineVerifierLayoutReport {
    admission: OfflineVerifierLayoutAdmission,
    family_id: DurableArtifactFamilyId,
    access_shape: OfflineVerifierAccessShape,
    posture: OfflineVerifierAuthorityPosture,
    evidence_kind: OfflineVerifierEvidenceKind,
    evidence_items: u64,
}

impl OfflineVerifierLayoutReport {
    fn new(
        evidence_kind: OfflineVerifierEvidenceKind,
        evidence_items: u64,
        rule: AdmittedOfflineVerifierLayoutRule,
    ) -> Self {
        Self {
            admission: OfflineVerifierLayoutAdmission { rule },
            family_id: DurableArtifactFamilyId::OfflineVerificationRecord,
            access_shape: OfflineVerifierAccessShape::FullDeclaredScan,
            posture: OfflineVerifierAuthorityPosture::TerminalOnly,
            evidence_kind,
            evidence_items,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn declared_access_shape(&self) -> OfflineVerifierAccessShape {
        self.access_shape
    }

    pub const fn authority_posture(&self) -> OfflineVerifierAuthorityPosture {
        self.posture
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

pub trait Phase28OfflineVerifierLayoutExt {
    fn admit_offline_verifier_layout(
        &self,
        rule: AdmittedOfflineVerifierLayoutRule,
    ) -> OfflineVerifierLayoutReport;
}

impl Phase28OfflineVerifierLayoutExt for OfflineExportBundleObservation {
    fn admit_offline_verifier_layout(
        &self,
        rule: AdmittedOfflineVerifierLayoutRule,
    ) -> OfflineVerifierLayoutReport {
        OfflineVerifierLayoutReport::new(
            OfflineVerifierEvidenceKind::ExportBundle,
            self.declarations().len() as u64 + self.digest_evidence_count(),
            rule,
        )
    }
}

impl Phase28OfflineVerifierLayoutExt for OfflineCustodyCapsuleObservation {
    fn admit_offline_verifier_layout(
        &self,
        rule: AdmittedOfflineVerifierLayoutRule,
    ) -> OfflineVerifierLayoutReport {
        let _ = self;
        OfflineVerifierLayoutReport::new(OfflineVerifierEvidenceKind::CustodyCapsule, 2, rule)
    }
}

impl Phase28OfflineVerifierLayoutExt for OfflineRepairBlastRadiusObservation {
    fn admit_offline_verifier_layout(
        &self,
        rule: AdmittedOfflineVerifierLayoutRule,
    ) -> OfflineVerifierLayoutReport {
        let _ = self;
        OfflineVerifierLayoutReport::new(OfflineVerifierEvidenceKind::RepairBlastRadius, 3, rule)
    }
}
