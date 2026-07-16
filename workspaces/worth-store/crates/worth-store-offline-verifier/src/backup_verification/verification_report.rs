use std::path::PathBuf;

use worth_store_physical_format::{BackupBundleArtifactFamily, BackupBundleArtifactFormat};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupArtifactSemanticDefectKind {
    OwnerReferenceInvalid,
    VerifierAllocationFailed,
    BufferBudgetTooSmall,
    Io,
    LengthMismatch,
    DigestMismatch,
    MalformedOwnerEncoding,
    OwnerBindingMismatch,
    CoverageMismatch,
    OwnerIntegrityMismatch,
    UnpublishedArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupVerificationDefect {
    VerificationCounterOverflow,
    PublishedManifestChanged,
    MissingComponent {
        output_name: String,
    },
    ExtraComponent {
        path: PathBuf,
    },
    ComponentLengthMismatch {
        output_name: String,
        expected: u64,
        actual: u64,
    },
    ComponentDigestMismatch {
        output_name: String,
    },
    CoverageFamilyMismatch {
        output_name: String,
    },
    MissingArtifactFamily(BackupBundleArtifactFamily),
    RootGenerationMismatch,
    CheckpointGenerationMismatch,
    RootCoverageMismatch,
    CheckpointCoverageMismatch,
    WalCoverageGapOrOverlap,
    OwnerSemanticMismatch {
        output_name: String,
        format: BackupBundleArtifactFormat,
        kind: BackupArtifactSemanticDefectKind,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupVerificationReadAccounting {
    Complete,
    LowerBound { unmeasured_owner_attempts: u64 },
    UnavailableAfterAcquisitionDenial,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupVerificationReport {
    defects: Vec<BackupVerificationDefect>,
    admitted_read_bytes: u64,
    inspected_bytes: u64,
    inspected_files: u64,
    peak_buffer_bytes: u64,
    owner_verified_artifacts: u64,
    owner_verified_bytes: u64,
    owner_decoder_allocation_bytes: u64,
    manifest_owned_allocation_bytes: u64,
    peak_owned_allocation_bytes: u64,
    read_accounting: BackupVerificationReadAccounting,
}

pub(crate) struct BackupVerificationReportEvidence {
    pub(crate) defects: Vec<BackupVerificationDefect>,
    pub(crate) admitted_read_bytes: u64,
    pub(crate) inspected_bytes: u64,
    pub(crate) inspected_files: u64,
    pub(crate) peak_buffer_bytes: u64,
    pub(crate) owner_verified_artifacts: u64,
    pub(crate) owner_verified_bytes: u64,
    pub(crate) owner_decoder_allocation_bytes: u64,
    pub(crate) manifest_owned_allocation_bytes: u64,
    pub(crate) peak_owned_allocation_bytes: u64,
    pub(crate) read_accounting: BackupVerificationReadAccounting,
}

impl BackupVerificationReport {
    pub(crate) fn new(evidence: BackupVerificationReportEvidence) -> Self {
        Self {
            defects: evidence.defects,
            admitted_read_bytes: evidence.admitted_read_bytes,
            inspected_bytes: evidence.inspected_bytes,
            inspected_files: evidence.inspected_files,
            peak_buffer_bytes: evidence.peak_buffer_bytes,
            owner_verified_artifacts: evidence.owner_verified_artifacts,
            owner_verified_bytes: evidence.owner_verified_bytes,
            owner_decoder_allocation_bytes: evidence.owner_decoder_allocation_bytes,
            manifest_owned_allocation_bytes: evidence.manifest_owned_allocation_bytes,
            peak_owned_allocation_bytes: evidence.peak_owned_allocation_bytes,
            read_accounting: evidence.read_accounting,
        }
    }
    pub fn defects(&self) -> &[BackupVerificationDefect] {
        &self.defects
    }
    pub const fn inspected_bytes(&self) -> u64 {
        self.inspected_bytes
    }
    pub const fn admitted_read_bytes(&self) -> u64 {
        self.admitted_read_bytes
    }
    pub const fn inspected_files(&self) -> u64 {
        self.inspected_files
    }
    pub const fn peak_buffer_bytes(&self) -> u64 {
        self.peak_buffer_bytes
    }
    pub const fn owner_verified_artifacts(&self) -> u64 {
        self.owner_verified_artifacts
    }
    pub const fn owner_verified_bytes(&self) -> u64 {
        self.owner_verified_bytes
    }
    pub const fn owner_decoder_allocation_bytes(&self) -> u64 {
        self.owner_decoder_allocation_bytes
    }
    pub const fn manifest_owned_allocation_bytes(&self) -> u64 {
        self.manifest_owned_allocation_bytes
    }
    pub const fn peak_owned_allocation_bytes(&self) -> u64 {
        self.peak_owned_allocation_bytes
    }
    pub const fn read_accounting(&self) -> BackupVerificationReadAccounting {
        self.read_accounting
    }
}
