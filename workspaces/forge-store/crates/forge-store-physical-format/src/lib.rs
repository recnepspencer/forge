//! Store physical format vocabulary.
//!
//! Construction-boundary compile-fail proofs live in [`physical_format_compile_fail`]
//! and the internal `compile_fail` module tree.
#![forbid(unsafe_code)]

mod binary_format;
mod compile_fail;
mod blob_manifest;
mod checksum;
mod denial;
mod extent_record;
mod facade;
mod format_identity;
mod generation;
mod header;
mod manifest;
mod offline_verifier;
mod page_record;
mod payload;
mod record_framing;
mod reference;
mod security_metadata;

// Lifecycle-ordered public exports (≤12 groups).
pub use format_identity::{
    PhysicalEpoch, PhysicalExtentId, PhysicalFormatMagic, PhysicalFormatVersion,
    PhysicalFormatVocabulary, PhysicalFrameId, PhysicalGeneration, PhysicalPageId,
    PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId, PhysicalVocabularyTerm,
};
pub use binary_format::{
    AllocationClassKind, FreeSpaceMapVocabulary, PhysicalAlgorithmReviewConclusion,
    PhysicalAlgorithmReviewEvidence, PhysicalAlignmentClass, PhysicalAlignmentSite,
    PhysicalBinaryEncodingWitness, PhysicalBinaryFormatError, PhysicalByteOrder,
    PhysicalByteOrderDeclaration, PhysicalComplexityStatus, PhysicalFieldWidth,
    PhysicalFieldWidthKind, PhysicalForegroundBoundednessOutcome,
    PhysicalForegroundBoundednessReport, PhysicalFormatAuthoritySource,
    PhysicalFormatDeclaration, PhysicalFormatDeclarationBuilder, PhysicalFormatEvolutionPosture,
    PhysicalFormatIdentity, PhysicalForwardCompatibilityDeclaration,
    PhysicalForwardCompatibilityPolicy, PhysicalFragmentationPressureReport,
    PhysicalFreeSpaceSearchPolicy, PhysicalGoldenFormatHeaderFixture, PhysicalLocalityClass,
    PhysicalOperationComplexityContract, PhysicalOperationCounterRow,
    PhysicalOperationCounterSnapshot, PhysicalOperationEvidenceRequirement, PhysicalOperationKind,
    PhysicalPageSizeClass, PhysicalReservedFieldPolicy, PhysicalReservedFieldPolicyDeclaration,
};
pub use header::{
    PhysicalDecodedHeader, PhysicalFrameHeader, PhysicalFrameKind, PhysicalHeaderAuthority,
    PhysicalHeaderAuthorityScope, PhysicalHeaderDecodeCounterSnapshot, PhysicalHeaderDecodeDenial,
    PhysicalHeaderDecodeDenialKind, PhysicalHeaderDecodeReport, PhysicalHeaderDecodeWitness,
    PhysicalHeaderKind, PhysicalHeaderReservedField, PhysicalHeaderReservedFields, PhysicalPageHeader,
    PhysicalPageKind, PhysicalPublicationState, PHYSICAL_HEADER_LENGTH,
};
pub use payload::{PhysicalPayloadView, PhysicalPayloadViewAdmission};
pub use record_framing::{
    FramedRecordPayload, FramedRecordView, RecordPagePayload, RecordPlacementClass,
    RecordPlacementWitness,
};
pub use generation::{
    ExtentGenerationCell, ExtentGenerationCellBuilder, FreeSpaceReuseAddress, FreeSpaceReuseCell,
    FreeSpaceReuseCellBuilder, PageGenerationCell, PageGenerationCellBuilder,
    PhysicalCellReuseDomain, PhysicalGenerationAuthority, PhysicalGenerationAuthorityScope,
    PhysicalGenerationOwner, RootPublicationCell, RootPublicationCellBuilder,
    SegmentGenerationCell, SegmentGenerationCellBuilder, SlotGenerationCell,
    SlotGenerationCellBuilder,
};
pub use reference::{
    CheckpointAdjacencyPosture, CurrentRootManifestAdmission, ManifestMembershipDenial,
    ManifestMembershipProof, PhysicalFutureChunkId, PhysicalFutureChunkReference,
    PhysicalReference, PhysicalReferenceAdmissionWitness, PhysicalReferenceAuthority,
    PhysicalReferenceAuthorityScope, PhysicalReferenceDenialKind, PhysicalReferenceKind,
    PhysicalReferenceScope, PhysicalReferenceValidationCounterSnapshot,
    PhysicalReferenceValidationDenial, PhysicalReferenceValidationWitness, PhysicalScopeFamily,
    RootManifestIntegrityPosture, RootPublicationValidationWitness, StalePhysicalReference,
};
pub use extent_record::{
    ExtentBackedRecordPlacement, ExtentBackedRecordView, ExtentMembership, ExtentRecordAppendReport,
    ExtentRecordAppendRequest, ExtentRecordCounterSnapshot, ExtentRecordDenial,
    ExtentRecordDenialKind, ExtentRecordLocateReport, PhysicalExtentRecordAuthority,
};
pub use page_record::{
    PageRecordCounterSnapshot, PageRecordDenial, PageRecordDenialKind, PhysicalPageRecordAuthority,
    RecordAppendReport, RecordLocateReport, SlotAppendRequest, SlotDirectory, SlotDirectoryEntry,
    SlotDirectoryEntryState,
};
pub use manifest::{
    AllocationClassManifestEntry, ExtentManifestEntry, ExtentManifestVocabulary,
    FreeSpaceManifestEntry, ManifestDiscoveryAuthority, ManifestDiscoveryCounterSnapshot,
    ManifestDiscoveryDenial, ManifestDiscoveryDenialKind, ManifestDiscoveryReport,
    ManifestVocabularyKind, PhysicalManifestUniverseBuilder, PhysicalReclaimRegion,
    PhysicalReclaimRegionDenial, PhysicalRootManifest, PhysicalRootManifestVocabulary,
    ReclaimedByteInterpretation, SegmentManifestEntry, SegmentManifestVocabulary,
    SegmentPageManifestEntry,
};
pub use offline_verifier::{
    ManifestTraversalReport, MinimalManifestVerifierReport, OfflineManifestCodec,
    OfflinePhysicalVerifier, OfflineVerifierCounterSnapshot, OfflineVerifierDenial,
    OfflineVerifierDenialKind, OfflineVerifierLayoutObservation, OfflineVerifierObservationSource,
    PersistedExtentBytes, PersistedPageBytes, PersistedPhysicalLayout,
    PersistedPhysicalLayoutBuilder, PhysicalLayoutReport, RuntimeLayoutObservation,
    RuntimeLayoutObservationSource,
};
pub use facade::{
    PlatformPhysicalAppendReport, PlatformPhysicalAppendRequest, PlatformPhysicalFacade,
    PlatformPhysicalFacadeCounterSnapshot, PlatformPhysicalFacadeDenial,
    PlatformPhysicalFacadeDenialKind, PlatformPhysicalFacadeEvidence,
    PlatformPhysicalFacadeOperation, PlatformPhysicalFacadeVocabulary, PlatformPhysicalFramedRecord,
    PlatformPhysicalLocateReport, PlatformPhysicalOpenRequest, PlatformPhysicalRecordTarget,
    PlatformPhysicalRootPublicationReport, PlatformPhysicalRuntimeLayoutReport,
    PlatformPhysicalScanReport,
};
pub use checksum::{
    ChecksumCompatibilityFieldPosture, ChecksumCoverageAuthoritySource, ChecksumCoverageDisposition,
    ChecksumCoverageEncoding, ChecksumCoverageMap, ChecksumCoverageMapBuilder,
    ChecksumCoverageMapDenial, ChecksumCoverageRegion, ChecksumFieldHandling,
    ChecksumGenerationFieldPosture, ChecksumHeaderField, ChecksumLengthFieldPosture,
    ChecksumPaddingPosture, ChecksumPayloadRegion, ChecksumReservedFieldPosture,
    ChecksumUnknownFieldPosture, PhysicalChunkChecksum, PhysicalChunkChecksumAlgorithm,
    PhysicalChunkChecksumAuthority, PhysicalChunkChecksumDenial, PhysicalChunkChecksumWitness,
    PhysicalChunkPayloadIntegrityWitness, StorePhysicalChunkWriteReceipt,
    StorePhysicalChunkWriteSource, s1_required_covered_header_fields,
};
pub use blob_manifest::{
    BlobPhysicalManifestDenial, BlobPhysicalManifestDenialKind, BlobPhysicalManifestRow,
    BlobPhysicalManifestRowKind, BlobPhysicalManifestValidation,
};
pub use denial::{
    PhysicalShortcutBoundary, PhysicalShortcutBoundaryDenial, PhysicalVocabularyError,
};
pub use security_metadata::{
    AllocationClassSecurityMetadataEnvelope, ExtentSecurityMetadataEnvelope,
    FreeSpaceSecurityMetadataEnvelope, PhysicalRawSecurityMetadataProjectionSource,
    PhysicalSecurityMetadataDeclaration, PhysicalSecurityMetadataDeclarationKind,
    PhysicalSecurityMetadataDenial, PhysicalSecurityMetadataEnvelope,
    PhysicalSecurityMetadataResultExclusion, PhysicalSecurityScopePropagationDenial,
    PhysicalSecurityScopePropagationDenialKind, SegmentPageSecurityMetadataEnvelope,
    SegmentSecurityMetadataEnvelope,
};

#[path = "compile_fail/physical_format_compile_fail.rs"]
#[doc(hidden)]
pub mod physical_format_compile_fail;