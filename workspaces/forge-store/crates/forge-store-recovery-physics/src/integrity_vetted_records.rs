use crate::{RecoveryIntegrityHandoffReceipt, IntegrityHandoffDenial};
use forge_store_physical_format::{
    PhysicalGenerationOwner, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    checkpoint_authority_digest, frame_authority_digest, manifest_authority_digest,
    page_authority_digest, wal_frame_authority_digest, CheckpointRecordIntegrityReport,
    FrameIntegrityReport, IntegrityEvidenceCounters, ManifestIntegrityCounters,
    ManifestIntegrityReport, PageIntegrityReport, PhysicalBoundaryLocalization,
    SegmentManifestIntegrityReport, WalFrameIntegrityCounters, WalFrameIntegrityInputIdentity,
    WalFrameIntegrityReport, WalTailIntegrityPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityVettedWalFrame {
    input_identity: WalFrameIntegrityInputIdentity,
    tail_posture: WalTailIntegrityPosture,
    counters: WalFrameIntegrityCounters,
    receipt: RecoveryIntegrityHandoffReceipt,
}

impl IntegrityVettedWalFrame {
    pub fn from_integrity_report(
        report: &WalFrameIntegrityReport,
        receipt: RecoveryIntegrityHandoffReceipt,
    ) -> Result<Self, IntegrityHandoffDenial> {
        receipt.require_scope(report.basis().scope())?;
        receipt.require_counters(IntegrityEvidenceCounters::WalFrame(report.counters()))?;
        let expected = wal_frame_authority_digest(report).map_err(authority_basis_denial)?;
        receipt.require_physical_authority_basis(&expected)?;
        Ok(Self {
            input_identity: report.input_identity(),
            tail_posture: report.tail_posture(),
            counters: report.counters(),
            receipt,
        })
    }

    pub const fn input_identity(&self) -> WalFrameIntegrityInputIdentity {
        self.input_identity
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }

    pub const fn counters(&self) -> WalFrameIntegrityCounters {
        self.counters
    }

    pub const fn receipt(&self) -> &RecoveryIntegrityHandoffReceipt {
        &self.receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityVettedCheckpointRecord {
    input_identity: WalFrameIntegrityInputIdentity,
    tail_posture: WalTailIntegrityPosture,
    counters: WalFrameIntegrityCounters,
    receipt: RecoveryIntegrityHandoffReceipt,
}

impl IntegrityVettedCheckpointRecord {
    pub fn from_integrity_report(
        report: &CheckpointRecordIntegrityReport,
        receipt: RecoveryIntegrityHandoffReceipt,
    ) -> Result<Self, IntegrityHandoffDenial> {
        receipt.require_scope(report.basis().scope())?;
        receipt.require_counters(IntegrityEvidenceCounters::WalFrame(report.counters()))?;
        let expected = checkpoint_authority_digest(report).map_err(authority_basis_denial)?;
        receipt.require_physical_authority_basis(&expected)?;
        Ok(Self {
            input_identity: report.input_identity(),
            tail_posture: report.tail_posture(),
            counters: report.counters(),
            receipt,
        })
    }

    pub const fn input_identity(&self) -> WalFrameIntegrityInputIdentity {
        self.input_identity
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }

    pub const fn counters(&self) -> WalFrameIntegrityCounters {
        self.counters
    }

    pub const fn receipt(&self) -> &RecoveryIntegrityHandoffReceipt {
        &self.receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityVettedRootManifestRecord {
    posture: RootManifestIntegrityPosture,
    root_owner: Option<PhysicalGenerationOwner>,
    counters: ManifestIntegrityCounters,
    receipt: RecoveryIntegrityHandoffReceipt,
}

impl IntegrityVettedRootManifestRecord {
    pub fn from_manifest_report(
        report: &ManifestIntegrityReport,
        receipt: RecoveryIntegrityHandoffReceipt,
    ) -> Result<Self, IntegrityHandoffDenial> {
        receipt.require_counters(IntegrityEvidenceCounters::Manifest(report.counters()))?;
        let expected = manifest_authority_digest(report).map_err(authority_basis_denial)?;
        receipt.require_physical_authority_basis(&expected)?;
        Ok(Self {
            posture: report.root().posture(),
            root_owner: report.root().root_owner(),
            counters: report.counters(),
            receipt,
        })
    }

    pub const fn posture(&self) -> RootManifestIntegrityPosture {
        self.posture
    }

    pub const fn root_owner(&self) -> Option<PhysicalGenerationOwner> {
        self.root_owner
    }

    pub const fn counters(&self) -> ManifestIntegrityCounters {
        self.counters
    }

    pub const fn receipt(&self) -> &RecoveryIntegrityHandoffReceipt {
        &self.receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityVettedSegmentManifestRecord {
    segment: SegmentManifestIntegrityReport,
    counters: ManifestIntegrityCounters,
    receipt: RecoveryIntegrityHandoffReceipt,
}

impl IntegrityVettedSegmentManifestRecord {
    pub fn from_manifest_report(
        report: &ManifestIntegrityReport,
        receipt: RecoveryIntegrityHandoffReceipt,
    ) -> Result<Self, IntegrityHandoffDenial> {
        receipt.require_counters(IntegrityEvidenceCounters::Manifest(report.counters()))?;
        let expected = manifest_authority_digest(report).map_err(authority_basis_denial)?;
        receipt.require_physical_authority_basis(&expected)?;
        Ok(Self {
            segment: *report.segment(),
            counters: report.counters(),
            receipt,
        })
    }

    pub const fn segment(&self) -> SegmentManifestIntegrityReport {
        self.segment
    }

    pub const fn counters(&self) -> ManifestIntegrityCounters {
        self.counters
    }

    pub const fn receipt(&self) -> &RecoveryIntegrityHandoffReceipt {
        &self.receipt
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityVettedPageFrameKind {
    Page,
    Frame,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityVettedPageFrameRecord {
    kind: IntegrityVettedPageFrameKind,
    scope: PhysicalReferenceScope,
    boundary: PhysicalBoundaryLocalization,
    counters: forge_store_physical_integrity::ContainerIntegrityCounters,
    receipt: RecoveryIntegrityHandoffReceipt,
}

impl IntegrityVettedPageFrameRecord {
    pub fn from_page_report(
        report: &PageIntegrityReport,
        receipt: RecoveryIntegrityHandoffReceipt,
    ) -> Result<Self, IntegrityHandoffDenial> {
        receipt.require_scope(report.basis().scope())?;
        receipt.require_counters(IntegrityEvidenceCounters::Container(report.counters()))?;
        let expected = page_authority_digest(report).map_err(authority_basis_denial)?;
        receipt.require_physical_authority_basis(&expected)?;
        Ok(Self {
            kind: IntegrityVettedPageFrameKind::Page,
            scope: report.basis().scope(),
            boundary: report.boundary(),
            counters: report.counters(),
            receipt,
        })
    }

    pub fn from_frame_report(
        report: &FrameIntegrityReport,
        receipt: RecoveryIntegrityHandoffReceipt,
    ) -> Result<Self, IntegrityHandoffDenial> {
        receipt.require_scope(report.basis().scope())?;
        receipt.require_counters(IntegrityEvidenceCounters::Container(report.counters()))?;
        let expected = frame_authority_digest(report).map_err(authority_basis_denial)?;
        receipt.require_physical_authority_basis(&expected)?;
        Ok(Self {
            kind: IntegrityVettedPageFrameKind::Frame,
            scope: report.basis().scope(),
            boundary: report.boundary(),
            counters: report.counters(),
            receipt,
        })
    }

    pub const fn kind(&self) -> IntegrityVettedPageFrameKind {
        self.kind
    }

    pub const fn scope(&self) -> PhysicalReferenceScope {
        self.scope
    }

    pub const fn boundary(&self) -> PhysicalBoundaryLocalization {
        self.boundary
    }

    pub const fn counters(&self) -> forge_store_physical_integrity::ContainerIntegrityCounters {
        self.counters
    }

    pub const fn receipt(&self) -> &RecoveryIntegrityHandoffReceipt {
        &self.receipt
    }
}

fn authority_basis_denial(
    _: forge_store_physical_integrity::PhysicalIntegrityEvidenceDenial,
) -> IntegrityHandoffDenial {
    IntegrityHandoffDenial::new(crate::IntegrityHandoffDenialKind::ReceiptBasisMismatch)
}
