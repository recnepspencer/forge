//! Physical substrate evidence families.

mod blob;
mod buffer_pool;
mod foundational;
mod io_scheduler;
mod isolation;
mod physical_format;
mod physical_integrity;
mod recovery;

pub use blob::{
    LargeStorePressureEvidenceBundle, LargeStorePressureEvidenceDenial, LargeStoreShortcutAttempt,
};
pub use buffer_pool::{
    AllocationEnvelopeEvidenceDenial, AllocationEnvelopeEvidenceReport, AllocationEnvelopeEvidenceRow,
    DirtyPublicationEvidenceDenial, DirtyPublicationEvidenceReport, DirtyPublicationEvidenceRow,
    EvictionProtectionEvidenceDenial, EvictionProtectionEvidenceReport, EvictionProtectionEvidenceRow,
    PinLifecycleEvidenceDenial, PinLifecycleEvidenceReport, PinLifecycleEvidenceRow,
    RecordViewEvidenceDenial, RecordViewEvidenceReport, RecordViewEvidenceRow,
    ResidentFrameAuthorityEvidenceDenial, ResidentFrameAuthorityEvidenceReport,
    ResidentFrameAuthorityEvidenceRow, SpeculativeWorkEvidenceDenial, SpeculativeWorkEvidenceReport,
    SpeculativeWorkEvidenceRow,
};
pub use foundational::{
    AllocationEnvelopePerformanceReceipt, BufferPoolProvenanceAttachment,
    CompletedResidencyBoundaryReceipt, CopyMaterializationPerformanceReceipt,
    FoundationalBoundaryAuthorityResult, FoundationalBoundaryEvidenceDenial,
    FoundationalEvidenceProfile, FoundationalEvidenceRichness, MaterializationProfileReport,
    PhysicalFoundationEvidenceBundle, PhysicalFoundationEvidenceBundleBuilder,
    PhysicalFoundationEvidenceDenial, PhysicalFoundationEvidenceEntry,
    PhysicalFoundationEvidenceIdentity, ResidentMemoryPerformanceReceipt,
    ZeroCopyLayoutPostureReport, certify_s0_handoff_gate_proof_evidence,
    S0HandoffGateCertificationDenial, S2EntryBoundaryEvidenceDenial, S2EntryBoundaryEvidenceReport,
    S2EntryBoundaryEvidenceRow, S2ForbiddenEntryAttempt,
};
pub use io_scheduler::{
    BackgroundClassEnvelopeEvidence, BackgroundEnvelopeEvidenceBundle,
    BackgroundEnvelopeEvidenceDenial, RequiredInterferenceKind,
};
pub use physical_format::{
    BinaryPhysicalFormatEvidence, BinaryPhysicalFormatEvidenceDenial,
    PhysicalComplexityEvidenceDenial, PhysicalComplexityEvidenceReport, PhysicalComplexityProofBundle,
    PhysicalExtentRecordFramingEvidenceDenial, PhysicalExtentRecordFramingEvidenceReport,
    PhysicalExtentRecordFramingEvidenceRow, PhysicalHeaderDecodeEvidenceDenial,
    PhysicalHeaderDecodeEvidenceReport, PhysicalHeaderDecodeEvidenceRow,
    PhysicalIdentityEvidenceDenial, PhysicalIdentityEvidenceReport, PhysicalIdentityEvidenceRow,
    PhysicalManifestDiscoveryEvidenceDenial, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow, PhysicalPageRecordFramingEvidenceDenial,
    PhysicalPageRecordFramingEvidenceReport, PhysicalPageRecordFramingEvidenceRow,
    PlatformPhysicalFacadeEvidenceDenial, PlatformPhysicalFacadeEvidenceReport,
    PlatformPhysicalFacadeEvidenceRow,
};
pub use physical_integrity::{
    offline_observer_requires_physical_references, PhysicalOfflineVerifierEvidenceDenial,
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
    ProtectedIntegrityViewEvidence, ProtectedIntegrityViewEvidenceDenial,
};