//! Store physical format vocabulary.
//!
//! Physical references are admitted only through physical authority. Raw
//! construction from placement coordinates is intentionally unavailable:
//!
//! ```compile_fail
//! use forge_store_physical_format::{
//!     PhysicalGeneration, PhysicalPageId, PhysicalRecordSlot, PhysicalReference,
//!     PhysicalSegmentId,
//! };
//!
//! let _forged = PhysicalReference::for_page_slot(
//!     PhysicalSegmentId::from_raw(1).unwrap(),
//!     PhysicalPageId::from_raw(1).unwrap(),
//!     PhysicalRecordSlot::from_raw(1).unwrap(),
//!     PhysicalGeneration::from_raw(1).unwrap(),
//! );
//! ```
//!
//! Admission witnesses are sealed proof values:
//!
//! ```compile_fail
//! use forge_store_physical_format::PhysicalReferenceAdmissionWitness;
//!
//! let _forged = PhysicalReferenceAdmissionWitness { reference: todo!() };
//! ```
//!
//! Generation owners are also sealed evidence, not raw diagnostic bags:
//!
//! ```compile_fail
//! use forge_store_physical_format::{
//!     PhysicalCellReuseDomain, PhysicalGeneration, PhysicalGenerationOwner,
//! };
//!
//! let _forged = PhysicalGenerationOwner {
//!     domain: PhysicalCellReuseDomain::SlotAllocation,
//!     generation: PhysicalGeneration::from_raw(1).unwrap(),
//! };
//! ```
//!
//! Page generation cannot substitute for slot generation:
//!
//! ```compile_fail
//! use forge_store_physical_format::{
//!     PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
//!     PhysicalReferenceAuthority, PhysicalSegmentId,
//! };
//!
//! let generations = PhysicalGenerationAuthority::s1();
//! let references = PhysicalReferenceAuthority::s1();
//! let page_cell = generations
//!     .page_cell(
//!         PhysicalSegmentId::from_raw(1).unwrap(),
//!         PhysicalPageId::from_raw(1).unwrap(),
//!     )
//!     .with_page_generation(PhysicalGeneration::from_raw(1).unwrap());
//!
//! let _ = references.admit_page_slot(page_cell);
//! ```
//!
//! Semantic artifact digests are not physical placement identity:
//!
//! ```compile_fail
//! use forge_store_contracts::StableDigest;
//! use forge_store_physical_format::PhysicalReferenceAuthority;
//!
//! let digest = StableDigest::new("sha256:not-physical-identity").unwrap();
//! let _ = PhysicalReferenceAuthority::s1().admit_root_publication(digest);
//! ```
//!
//! Header decode witnesses are sealed proof values:
//!
//! ```compile_fail
//! use forge_store_physical_format::PhysicalHeaderDecodeWitness;
//!
//! let _forged = PhysicalHeaderDecodeWitness {
//!     header: todo!(),
//!     owner: todo!(),
//!     counters: todo!(),
//! };
//! ```
//!
//! Payload views cannot be minted directly from raw bytes:
//!
//! ```compile_fail
//! use forge_store_physical_format::{PhysicalPayloadView, PhysicalHeaderDecodeWitness};
//!
//! let raw = b"not admitted payload";
//! let witness: PhysicalHeaderDecodeWitness = todo!();
//! let _forged = PhysicalPayloadView::new(raw, witness);
//! ```
//!
//! Framed record views cannot be minted without record-page admission:
//!
//! ```compile_fail
//! use forge_store_physical_format::{FramedRecordPayload, FramedRecordView};
//!
//! let raw = b"not admitted record";
//! let payload = FramedRecordPayload::new(raw);
//! let _forged = FramedRecordView::new(todo!(), payload, todo!());
//! ```
//!
//! Shortcut boundary denials are emitted by the facade boundary, not minted by
//! public callers:
//!
//! ```compile_fail
//! use forge_store_physical_format::PhysicalShortcutBoundaryDenial;
//!
//! let _forged = PhysicalShortcutBoundaryDenial::live_runtime_cache();
//! ```
//!
//! Shortcut boundary evidence also cannot be attached to facade denials outside
//! the physical-format crate:
//!
//! ```compile_fail
//! use forge_store_physical_format::{
//!     PhysicalShortcutBoundaryDenial, PlatformPhysicalFacadeDenial,
//!     PlatformPhysicalFacadeDenialKind,
//! };
//!
//! let denial = PlatformPhysicalFacadeDenial::new(
//!     PlatformPhysicalFacadeDenialKind::ShortcutBoundaryRejected,
//! );
//! let shortcut: PhysicalShortcutBoundaryDenial = todo!();
//! let _forged = denial.with_shortcut_denial(shortcut);
//! ```

#![forbid(unsafe_code)]

mod algorithm_review;
mod alignment;
mod allocation;
mod binary_format;
mod binary_format_denials;
#[cfg(test)]
mod binary_format_tests;
mod binary_format_witness;
mod byte_order;
mod denials;
mod extent_membership;
mod extent_record_authority;
mod extent_record_counters;
mod extent_record_denials;
#[cfg(test)]
mod extent_record_tests;
mod facade;
mod facade_append;
mod facade_counters;
mod facade_denials;
mod facade_evidence;
mod facade_locate;
mod facade_reports;
mod facade_requests;
mod facade_root_publication;
mod facade_storage;
#[cfg(test)]
mod facade_tests;
mod field_widths;
mod format_identity;
mod forward_compatibility;
mod free_space_policy;
mod generation_authority;
mod generation_cells;
mod generation_owner;
mod golden_bytes;
mod header_authority;
mod header_counters;
#[cfg(test)]
mod header_decode_tests;
mod header_denials;
mod header_kinds;
mod header_layout;
mod header_publication;
mod header_reserved;
mod header_witness;
mod ids;
mod manifest_authority;
mod manifest_counters;
mod manifest_denials;
mod manifest_entries;
#[cfg(test)]
mod manifest_tests;
mod manifest_universe;
mod manifests;
mod offline_manifest_codec;
mod offline_manifest_codec_decode;
mod offline_manifest_codec_decode_fields;
mod offline_manifest_codec_encode;
mod offline_persisted_layout;
mod offline_verifier;
mod offline_verifier_counters;
mod offline_verifier_denials;
mod offline_verifier_observation;
mod offline_verifier_report;
#[cfg(test)]
mod offline_verifier_tests;
mod operation_complexity;
#[cfg(test)]
mod operation_complexity_tests;
mod operation_counters;
mod page_record_authority;
mod page_record_counters;
mod page_record_denials;
#[cfg(test)]
mod page_record_test_support;
#[cfg(test)]
mod page_record_tests;
mod page_size;
mod payload_view;
mod record_framing;
mod reference_authority;
mod reference_counters;
mod reference_denials;
#[cfg(test)]
mod reference_identity_tests;
mod reference_witnesses;
mod references;
mod reserved_fields;
mod runtime_layout_observation;
mod shortcut_boundary_denials;
mod slot_directory;
mod slot_state;
mod vocabulary;

pub use algorithm_review::{PhysicalAlgorithmReviewConclusion, PhysicalAlgorithmReviewEvidence};
pub use alignment::{PhysicalAlignmentClass, PhysicalAlignmentSite};
pub use allocation::{AllocationClassKind, FreeSpaceMapVocabulary};
pub use binary_format::{
    PhysicalFormatAuthoritySource, PhysicalFormatDeclaration, PhysicalFormatDeclarationBuilder,
    PhysicalFormatIdentity,
};
pub use binary_format_denials::PhysicalBinaryFormatError;
pub use binary_format_witness::PhysicalBinaryEncodingWitness;
pub use byte_order::{PhysicalByteOrder, PhysicalByteOrderDeclaration};
pub use denials::PhysicalVocabularyError;
pub use extent_membership::ExtentMembership;
pub use extent_record_authority::{
    ExtentBackedRecordPlacement, ExtentBackedRecordView, ExtentRecordAppendReport,
    ExtentRecordAppendRequest, ExtentRecordLocateReport, PhysicalExtentRecordAuthority,
};
pub use extent_record_counters::ExtentRecordCounterSnapshot;
pub use extent_record_denials::{ExtentRecordDenial, ExtentRecordDenialKind};
pub use facade::{
    PlatformPhysicalFacade, PlatformPhysicalFacadeOperation, PlatformPhysicalFacadeVocabulary,
};
pub use facade_counters::PlatformPhysicalFacadeCounterSnapshot;
pub use facade_denials::{PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind};
pub use facade_evidence::PlatformPhysicalFacadeEvidence;
pub use facade_reports::{
    PlatformPhysicalAppendReport, PlatformPhysicalFramedRecord, PlatformPhysicalLocateReport,
    PlatformPhysicalRootPublicationReport, PlatformPhysicalRuntimeLayoutReport,
    PlatformPhysicalScanReport,
};
pub use facade_requests::{
    PlatformPhysicalAppendRequest, PlatformPhysicalOpenRequest, PlatformPhysicalRecordTarget,
};
pub use field_widths::{PhysicalFieldWidth, PhysicalFieldWidthKind};
pub use format_identity::{PhysicalFormatMagic, PhysicalFormatVersion};
pub use forward_compatibility::{
    PhysicalFormatEvolutionPosture, PhysicalForwardCompatibilityDeclaration,
    PhysicalForwardCompatibilityPolicy,
};
pub use free_space_policy::{
    PhysicalForegroundBoundednessOutcome, PhysicalForegroundBoundednessReport,
    PhysicalFragmentationPressureReport, PhysicalFreeSpaceSearchPolicy,
};
pub use generation_authority::{PhysicalGenerationAuthority, PhysicalGenerationAuthorityScope};
pub use generation_cells::{
    ExtentGenerationCell, ExtentGenerationCellBuilder, FreeSpaceReuseAddress, FreeSpaceReuseCell,
    FreeSpaceReuseCellBuilder, PageGenerationCell, PageGenerationCellBuilder, RootPublicationCell,
    RootPublicationCellBuilder, SegmentGenerationCell, SegmentGenerationCellBuilder,
    SlotGenerationCell, SlotGenerationCellBuilder,
};
pub use generation_owner::{PhysicalCellReuseDomain, PhysicalGenerationOwner};
pub use golden_bytes::PhysicalGoldenFormatHeaderFixture;
pub use header_authority::{PhysicalHeaderAuthority, PhysicalHeaderAuthorityScope};
pub use header_counters::PhysicalHeaderDecodeCounterSnapshot;
pub use header_denials::{PhysicalHeaderDecodeDenial, PhysicalHeaderDecodeDenialKind};
pub use header_kinds::{PhysicalFrameKind, PhysicalHeaderKind, PhysicalPageKind};
pub use header_layout::{
    PhysicalDecodedHeader, PhysicalFrameHeader, PhysicalPageHeader, PHYSICAL_HEADER_LENGTH,
};
pub use header_publication::PhysicalPublicationState;
pub use header_reserved::{PhysicalHeaderReservedField, PhysicalHeaderReservedFields};
pub use header_witness::{PhysicalHeaderDecodeReport, PhysicalHeaderDecodeWitness};
pub use ids::{
    PhysicalEpoch, PhysicalExtentId, PhysicalFrameId, PhysicalGeneration, PhysicalPageId,
    PhysicalRecordSlot, PhysicalRootReference, PhysicalSegmentId,
};
pub use manifest_authority::{ManifestDiscoveryAuthority, ManifestDiscoveryReport};
pub use manifest_counters::ManifestDiscoveryCounterSnapshot;
pub use manifest_denials::{ManifestDiscoveryDenial, ManifestDiscoveryDenialKind};
pub use manifest_entries::{
    AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry,
    SegmentManifestEntry, SegmentPageManifestEntry,
};
pub use manifest_universe::{PhysicalManifestUniverseBuilder, PhysicalRootManifest};
pub use manifests::{
    ExtentManifestVocabulary, ManifestVocabularyKind, PhysicalRootManifestVocabulary,
    SegmentManifestVocabulary,
};
pub use offline_manifest_codec::OfflineManifestCodec;
pub use offline_persisted_layout::{
    PersistedExtentBytes, PersistedPageBytes, PersistedPhysicalLayout,
    PersistedPhysicalLayoutBuilder,
};
pub use offline_verifier::OfflinePhysicalVerifier;
pub use offline_verifier_counters::OfflineVerifierCounterSnapshot;
pub use offline_verifier_denials::{OfflineVerifierDenial, OfflineVerifierDenialKind};
pub use offline_verifier_observation::{
    OfflineVerifierLayoutObservation, OfflineVerifierObservationSource,
};
pub use offline_verifier_report::{
    ManifestTraversalReport, MinimalManifestVerifierReport, PhysicalLayoutReport,
};
pub use operation_complexity::{
    PhysicalComplexityStatus, PhysicalLocalityClass, PhysicalOperationComplexityContract,
    PhysicalOperationEvidenceRequirement, PhysicalOperationKind,
};
pub use operation_counters::{PhysicalOperationCounterRow, PhysicalOperationCounterSnapshot};
pub use page_record_authority::{
    PhysicalPageRecordAuthority, RecordAppendReport, RecordLocateReport, SlotAppendRequest,
};
pub use page_record_counters::PageRecordCounterSnapshot;
pub use page_record_denials::{PageRecordDenial, PageRecordDenialKind};
pub use page_size::PhysicalPageSizeClass;
pub use payload_view::{PhysicalPayloadView, PhysicalPayloadViewAdmission};
pub use record_framing::{
    FramedRecordPayload, FramedRecordView, RecordPagePayload, RecordPlacementClass,
    RecordPlacementWitness,
};
pub use reference_authority::{PhysicalReferenceAuthority, PhysicalReferenceAuthorityScope};
pub use reference_counters::PhysicalReferenceValidationCounterSnapshot;
pub use reference_denials::{
    PhysicalReferenceDenialKind, PhysicalReferenceValidationDenial, StalePhysicalReference,
};
pub use reference_witnesses::{
    PhysicalReferenceAdmissionWitness, PhysicalReferenceValidationWitness,
};
pub use references::{PhysicalReference, PhysicalReferenceKind};
pub use reserved_fields::{PhysicalReservedFieldPolicy, PhysicalReservedFieldPolicyDeclaration};
pub use runtime_layout_observation::{RuntimeLayoutObservation, RuntimeLayoutObservationSource};
pub use shortcut_boundary_denials::{PhysicalShortcutBoundary, PhysicalShortcutBoundaryDenial};
pub use slot_directory::{SlotDirectory, SlotDirectoryEntry};
pub use slot_state::SlotDirectoryEntryState;
pub use vocabulary::{PhysicalFormatVocabulary, PhysicalVocabularyTerm};
