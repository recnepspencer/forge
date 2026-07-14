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
    AllocationEnvelopeEvidenceDenial, AllocationEnvelopeEvidenceReport,
    AllocationEnvelopeEvidenceRow, DirtyPublicationEvidenceDenial, DirtyPublicationEvidenceReport,
    DirtyPublicationEvidenceRow, EvictionProtectionEvidenceDenial,
    EvictionProtectionEvidenceReport, EvictionProtectionEvidenceRow, PinLifecycleEvidenceDenial,
    PinLifecycleEvidenceReport, PinLifecycleEvidenceRow, RecordViewEvidenceDenial,
    RecordViewEvidenceReport, RecordViewEvidenceRow, ResidentFrameAuthorityEvidenceDenial,
    ResidentFrameAuthorityEvidenceReport, ResidentFrameAuthorityEvidenceRow,
    SpeculativeWorkEvidenceDenial, SpeculativeWorkEvidenceReport, SpeculativeWorkEvidenceRow,
};
pub use foundational::{
    certify_aspect_native_boundary_audit, AllocationEnvelopePerformanceReceipt,
    AspectNativeBoundaryCertificationDenial, BufferPoolProvenanceAttachment,
    CompletedResidencyBoundaryReceipt, CopyMaterializationPerformanceReceipt,
    FoundationalBoundaryAuthorityResult, FoundationalBoundaryEvidenceDenial,
    FoundationalEvidenceProfile, FoundationalEvidenceRichness, MaterializationProfileReport,
    PhysicalFoundationEvidenceBundle, PhysicalFoundationEvidenceBundleBuilder,
    PhysicalFoundationEvidenceDenial, PhysicalFoundationEvidenceEntry,
    PhysicalFoundationEvidenceIdentity, ResidentMemoryPerformanceReceipt,
    S2EntryBoundaryEvidenceDenial, S2EntryBoundaryEvidenceReport, S2EntryBoundaryEvidenceRow,
    S2ForbiddenEntryAttempt, ZeroCopyLayoutPostureReport,
};
pub use io_scheduler::{
    BackgroundClassEnvelopeEvidence, BackgroundEnvelopeEvidenceBundle,
    BackgroundEnvelopeEvidenceDenial, RequiredInterferenceKind,
};
pub use physical_format::{
    BinaryPhysicalFormatEvidence, BinaryPhysicalFormatEvidenceDenial,
    PhysicalComplexityEvidenceDenial, PhysicalComplexityEvidenceReport,
    PhysicalComplexityProofBundle, PhysicalExtentRecordFramingEvidenceDenial,
    PhysicalExtentRecordFramingEvidenceReport, PhysicalExtentRecordFramingEvidenceRow,
    PhysicalHeaderDecodeEvidenceDenial, PhysicalHeaderDecodeEvidenceReport,
    PhysicalHeaderDecodeEvidenceRow, PhysicalIdentityEvidenceDenial,
    PhysicalIdentityEvidenceReport, PhysicalIdentityEvidenceRow,
    PhysicalManifestDiscoveryEvidenceDenial, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow, PhysicalPageRecordFramingEvidenceDenial,
    PhysicalPageRecordFramingEvidenceReport, PhysicalPageRecordFramingEvidenceRow,
    PhysicalStoreRuntimeEvidenceDenial, PhysicalStoreRuntimeEvidenceReport,
    PhysicalStoreRuntimeEvidenceRow,
};
pub use physical_integrity::{
    offline_observer_requires_physical_references, PhysicalOfflineVerifierEvidenceDenial,
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
    ProtectedIntegrityViewEvidence, ProtectedIntegrityViewEvidenceDenial,
};
