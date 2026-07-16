#![forbid(unsafe_code)]
#![doc = include_str!("physical_integrity_compile_fail_proofs.md")]

mod admission;
mod authority;
mod blob_chunks;
mod checksums;
mod compaction_source_clearance;
mod containers;
mod damage_classification;
mod damage_handoff;
mod evidence;
mod generation_integrity;
mod index_pages;
mod manifests;
mod offline_classification;
mod operational_repair;
mod quarantine;
mod scrub;
mod wal_frames;

pub use admission::entry::entry_admission::IntegrityEntryAdmission;
pub use admission::entry::entry_basis::{
    IntegrityEntryBasis, ScrubEnvelopeLimits, VerifierResidentLimits,
};
pub use admission::entry::entry_denial::{IntegrityEntryDenial, IntegrityEntryDenialKind};
pub use admission::entry::entry_request::IntegrityEntryRequest;
pub use admission::entry::entry_witness::IntegrityEntryWitness;
pub use admission::physical_scope::physical_scope_admission::PhysicalScopeAdmission;
pub use admission::physical_scope::physical_scope_basis::PhysicalScopeBasis;
pub use admission::physical_scope::physical_scope_denial::{
    ChecksumScopeMismatchDenial, IntactWrongScopeDenial, PhysicalScopeDenial,
    PhysicalScopeDenialKind,
};
pub use admission::physical_scope::physical_scope_family_inputs::ScopedPhysicalValidatorInput;
pub use admission::physical_scope::physical_scope_request::PhysicalScopeAdmissionRequest;
pub use admission::pre_decode::authenticity_decode_gate::{
    AuthenticityPolicyPhysicalDecodeGate, AuthenticityRequiredPhysicalDecodeGate,
};
pub use admission::pre_decode::authenticity_integrity_counters::{
    AuthenticityPolicyDecodeCounters, AuthenticityRequiredDecodeCounters,
};
pub use admission::pre_decode::inspection_lease::IntegrityInspectionLease;
pub use admission::pre_decode::integrity_checked_physical_form::{
    IntegrityCheckedFrame, IntegrityCheckedPage, IntegrityCheckedPhysicalFormKind,
};
pub use admission::pre_decode::logical_decode_gate::{
    LogicalDecodeGate, LogicalDecodeGateEvidence, LogicalDecodeGateIdentity, LogicalDecoder,
};
pub use admission::pre_decode::physical_integrity_admission::{
    PhysicalIntegrityAdmission, PhysicalIntegrityAdmissionSeed,
};
pub use admission::pre_decode::physical_integrity_request::{
    DeclaredPhysicalChecksum, PhysicalIntegrityAdmissionRequest,
};
pub use admission::pre_decode::pre_decode_counters::{
    PreDecodeAdmissionCounters, SemanticDecoderInvocationCounter, SkippedLogicalDecodeCounter,
};
#[cfg(any(test, feature = "test-support"))]
pub use admission::pre_decode::pre_decode_denial::test_pre_decode_denial_for_kind;
pub use admission::pre_decode::pre_decode_denial::{
    PreDecodePhysicalDenial, PreDecodePhysicalDenialKind,
};
pub use authority::integrity_authority_claim_basis::{
    checkpoint_authority_digest, frame_authority_digest, manifest_authority_digest,
    page_authority_digest, wal_frame_authority_digest,
};
pub use blob_chunks::chunk_integrity::ChunkIntegrityAuthority;
pub use blob_chunks::chunk_integrity_counters::ChunkIntegrityCounters;
pub use blob_chunks::chunk_integrity_denials::{
    ChunkDamageLocality, ChunkIntegrityDenial, ChunkIntegrityDenialKind,
    ChunkIntegrityStreamingWindowDenial,
};
pub use blob_chunks::chunk_integrity_reports::{
    ChunkIntegrityInputIdentity, ChunkIntegrityLifecycleClaims, ChunkIntegrityReport,
};
pub use blob_chunks::chunk_integrity_request::{
    ChunkIntegrityInspectionRequest, ChunkIntegrityStreamingWindow,
};
pub use checksums::checksum_algorithm::{ChecksumAlgorithmClaim, ChecksumAlgorithmId};
pub use checksums::checksum_compatibility::ChecksumCompatibilityPosture;
pub use checksums::checksum_declaration::{
    ChecksumAlgorithmDeclaration, ChecksumCoverageBasis, ChecksumDeclarationAdmission,
};
pub use checksums::checksum_denial::ChecksumAlgorithmMismatchDenial;
pub use checksums::checksum_detection_model::{
    ChecksumAuthenticityPosture, ChecksumAuthorizationPosture, ChecksumCollisionPosture,
    ChecksumCorruptionClass, ChecksumDetectionModel,
};
pub(crate) use checksums::checksum_execution::execute_declared_checksum;
pub use checksums::checksum_execution::ExecutedPhysicalChecksum;
pub(crate) use checksums::checksum_foundational_identity::foundational_identity_for_checksum_basis;
pub use checksums::checksum_foundational_identity::FoundationalChecksumEvidenceIdentity;
pub use checksums::checksum_scope::ChecksumScopeDeclaration;
pub use compaction_source_clearance::{
    CompactionSourceClearanceDenial, CompactionSourceClearanceKind,
    CompactionSourceIntegrityClearance,
};
pub use containers::container_integrity::PhysicalContainerIntegrity;
pub use containers::container_integrity_boundaries::PhysicalBoundaryLocalization;
pub use containers::container_integrity_counters::ContainerIntegrityCounters;
pub use containers::container_integrity_denials::{
    AmbiguousBoundaryDamage, PhysicalContainerIntegrityDenial,
    PhysicalContainerIntegrityDenialKind, TornFrameDenial,
};
pub use containers::container_integrity_reports::{
    ExtentIntegrityReport, FrameIntegrityReport, PageIntegrityReport, SlotDirectoryIntegrityReport,
    SlotStateIntegrityReport,
};
pub use damage_classification::{
    DamageClassification, IntactPhysicalBoundary, QuarantinedPhysicalDamage,
};
pub use damage_handoff::{
    classify_physical_damage_for_handoff, quarantine_handoff_posture, PhysicalDamageHandoffEvidence,
};
pub use evidence::integrity_evidence_authority::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceEquivalence,
};
pub use evidence::integrity_evidence_bundle::{
    IntegrityDiagnosticReport, IntegrityEvidenceCounters, IntegrityEvidenceLocality,
    IntegrityEvidenceOutcome, IntegrityPerformanceReceipt, IntegrityProvenanceAttachment,
    PhysicalIntegrityCertificationReceipt, PhysicalIntegrityEvidenceBundle,
    StoreIntegrityBoundaryClaim,
};
pub use evidence::integrity_evidence_denial::PhysicalIntegrityEvidenceDenial;
pub use evidence::integrity_evidence_profile::{
    PhysicalIntegrityEvidenceProfile, PhysicalIntegrityEvidenceRichness,
};
pub use evidence::integrity_evidence_proof_report::{
    IntegrityProofProgressionOutcome, IntegrityProofProgressionReport,
    IntegrityProofProgressionSnapshot,
};
pub use evidence::integrity_evidence_roles::{
    FoundationalBoundaryRoleMapping, StoreDerivedProjectionBoundaryClaim,
    StorePhysicalAuthorityBoundaryClaim, StorePlannedWorkBoundaryClaim,
    StorePlannedWorkBoundaryKind, StorePlannedWorkBoundaryReport,
    StoreReceiptEvidenceBoundaryClaim, StoreSupportOnlyBoundaryClaim,
};
pub use evidence::integrity_evidence_source::{
    IntegrityEvidenceMaterializationPath, StoreExecutedIntegrityEvidence,
};
pub use generation_integrity::GenerationIntegrityReport;
pub use index_pages::index_page_integrity::DerivedIndexIntegrityAuthority;
pub use index_pages::index_page_integrity_counters::IndexPageIntegrityCounters;
pub use index_pages::index_page_integrity_denials::{
    IndexPageIntegrityDenial, IndexPageIntegrityDenialKind,
};
pub use index_pages::index_page_integrity_reports::{
    AuthorityDamageBoundary, DerivedDamageClassification, DerivedRebuildInput,
    IndeterminatePhysicalDamage, IndexPageIntegrityReport, IntactIndexPageBoundary,
    RebuildabilityPrerequisite, RebuildableDerivedDamage, RebuildableDerivedDamagePrerequisites,
    UnrecoverableAuthorityDamage,
};
pub use index_pages::index_page_integrity_request::DerivedIndexIntegrityInspectionRequest;
pub(crate) use manifests::manifest_allocation_map::allocation_map_report;
pub use manifests::manifest_integrity::ManifestIntegrityAuthority;
pub use manifests::manifest_integrity_counters::ManifestIntegrityCounters;
pub use manifests::manifest_integrity_denials::{
    ManifestGenerationMismatchDenial, ManifestIntegrityDenial, ManifestIntegrityDenialKind,
    ManifestReferenceMismatchDenial,
};
pub use manifests::manifest_integrity_reports::{
    AllocationMapIntegrityReport, ManifestIntegrityReport, ManifestReferenceBasis,
    RootManifestIntegrityReport, SegmentManifestIntegrityReport,
};
pub(crate) use manifests::manifest_integrity_request::ManifestRootIntegrityEvidence;
pub use manifests::manifest_integrity_request::{
    AuthoritativeManifestFailure, DerivedManifestOverrideAttempt, ManifestExpectedReference,
    ManifestIntegrityInspectionRequest,
};
pub(crate) use manifests::manifest_root_posture::admit_root_posture;
pub(crate) use manifests::manifest_source_precedence::deny_derived_override;
pub use scrub::offline_scrub_input::{
    OfflineScrubInspectionInput, OfflineScrubInspectionInputDenial, OfflineScrubVerifierBasis,
};

pub use admission::pre_decode::protected_physical_byte_view::ProtectedPhysicalByteView;
pub use offline_classification::{
    classify_offline_integrity, OfflineIntegrityObservation, OfflineIntegrityPosture,
};
pub use operational_repair::{
    IntegrityOperationalRepairOwner, IntegrityRepairArtifactFamily,
    IntegrityRepairClassificationDenial, IntegrityRepairClassificationPlan,
    IntegrityRepairClassificationReceipt, IntegrityRepairOwnerBinding, IntegrityRepairRegion,
    IntegrityRepairRegionClass,
};
pub use quarantine::quarantine_authority::PhysicalQuarantineAuthority;
pub use quarantine::quarantine_denial::{QuarantineSealDenial, QuarantineSealDenialKind};
pub use quarantine::quarantine_finding::ExecutedQuarantineFinding;
pub use quarantine::quarantine_locality::{PhysicalLocalityReport, QuarantineLocalityBoundary};
pub use quarantine::quarantine_outcome::{
    QuarantineSealCounterSnapshot, QuarantineSealOutcome, QuarantineSealOutcomeView,
};
pub use quarantine::quarantine_posture::{QuarantineHandoffPosture, QuarantineLifecyclePosture};
pub use quarantine::quarantine_receipt::{FoundationalQuarantineReceiptBasis, QuarantineReceipt};
pub use quarantine::quarantine_record::QuarantineRecord;
pub use quarantine::quarantine_request::QuarantineSealRequest;
pub use scrub::scrub_counters::ScrubCounterSnapshot;
pub use scrub::scrub_denial::{
    ScrubExecutionDenial, ScrubExecutionDenialKind, ScrubOverBudgetClass, ScrubPlanDenial,
    ScrubPlanDenialKind,
};
pub use scrub::scrub_execution::{
    ScrubExecution, ScrubExecutionReceipt, ScrubIntegrityFinding, ScrubProgressReport,
};
pub use scrub::scrub_plan::{
    PlannedScrubWindow, PlannedScrubWindowStatus, ScrubPlan, ScrubPlanBudget, ScrubPlanRequest,
};
pub use scrub::scrub_planning_memory_envelope::{
    ScrubPlanningMemoryEnvelope, ScrubPlanningMemoryEnvelopeDenial,
};
pub use scrub::scrub_resume::ScrubResumeToken;
pub use scrub::scrub_scheduler_demand::scrub_scan_scheduler_demand;
pub use scrub::scrub_window::{
    ScrubLocalitySummary, ScrubMode, ScrubWindow, ScrubWindowOrdinal, ScrubWindowSource,
};
pub use wal_frames::wal_frame_integrity::WalFrameIntegrityAuthority;
pub use wal_frames::wal_frame_integrity_counters::WalFrameIntegrityCounters;
pub use wal_frames::wal_frame_integrity_denials::{
    CheckpointAdjacentDamageDenial, WalFrameDamageDenial, WalFrameDamageDenialKind,
};
pub use wal_frames::wal_frame_integrity_reports::{
    CheckpointRecordIntegrityReport, WalFrameIntegrityInputIdentity, WalFrameIntegrityReport,
    WalTailIntegrityPosture,
};
pub use wal_frames::wal_frame_integrity_request::WalFrameIntegrityInspectionRequest;
