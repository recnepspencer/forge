#![forbid(unsafe_code)]
#![doc = include_str!("physical_integrity_compile_fail_proofs.md")]

pub mod layout_access;

mod authenticity_decode_gate;
mod authenticity_integrity_counters;
mod checksum_algorithm;
mod checksum_compatibility;
mod checksum_declaration;
mod checksum_denial;
mod checksum_detection_model;
mod checksum_execution;
mod checksum_foundational_identity;
mod checksum_scope;
mod chunk_integrity;
mod chunk_integrity_counters;
mod chunk_integrity_denials;
mod chunk_integrity_reports;
mod chunk_integrity_request;
mod compaction_source_clearance;
mod container_integrity;
mod container_integrity_boundaries;
mod container_integrity_counters;
mod container_integrity_denials;
mod container_integrity_frame_header;
mod container_integrity_reports;
mod container_integrity_slot_directory;
mod damage_classification;
mod damage_handoff;
mod entry_admission;
mod entry_basis;
mod entry_denial;
mod entry_request;
mod entry_witness;
mod generation_integrity;
mod index_page_integrity;
mod index_page_integrity_counters;
mod index_page_integrity_denials;
mod index_page_integrity_reports;
mod index_page_integrity_request;
mod inspection_lease;
mod integrity_authority_basis_entries;
mod integrity_authority_basis_tokens;
mod integrity_authority_claim_basis;
mod integrity_authority_counter_entries;
mod integrity_checked_physical_form;
mod integrity_evidence_authority;
mod integrity_evidence_bundle;
mod integrity_evidence_denial;
mod integrity_evidence_profile;
mod integrity_evidence_proof_report;
mod integrity_evidence_quarantine;
mod integrity_evidence_roles;
mod integrity_evidence_source;
mod logical_decode_gate;
mod manifest_allocation_map;
mod manifest_integrity;
mod manifest_integrity_counters;
mod manifest_integrity_denials;
mod manifest_integrity_reports;
mod manifest_integrity_request;
mod manifest_root_posture;
mod manifest_source_precedence;
mod offline_scrub_input;
mod physical_integrity_admission;
mod physical_integrity_request;
mod physical_scope_admission;
mod physical_scope_basis;
mod physical_scope_denial;
mod physical_scope_family_inputs;
mod physical_scope_request;
mod pre_decode_counters;
mod pre_decode_denial;
mod pre_decode_surface;
mod protected_physical_byte_view;
mod quarantine_authority;
mod quarantine_denial;
mod quarantine_finding;
mod quarantine_locality;
mod quarantine_posture;
mod quarantine_receipt;
mod quarantine_record;
mod quarantine_request;
mod scrub_counters;
mod scrub_denial;
mod scrub_execution;
mod scrub_plan;
mod scrub_plan_identity;
mod scrub_planning_memory_envelope;
mod scrub_resume;
mod scrub_scheduler_demand;
mod scrub_window;
mod wal_frame_integrity;
mod wal_frame_integrity_counters;
mod wal_frame_integrity_denials;
mod wal_frame_integrity_reports;
mod wal_frame_integrity_request;

pub use authenticity_decode_gate::{
    AuthenticityPolicyPhysicalDecodeGate, AuthenticityRequiredPhysicalDecodeGate,
};
pub use authenticity_integrity_counters::{
    AuthenticityPolicyDecodeCounters, AuthenticityRequiredDecodeCounters,
};
pub use checksum_algorithm::{ChecksumAlgorithmClaim, ChecksumAlgorithmId};
pub use checksum_compatibility::ChecksumCompatibilityPosture;
pub use checksum_declaration::{
    ChecksumAlgorithmDeclaration, ChecksumCoverageBasis, S3ChecksumDeclarationAdmission,
};
pub use checksum_denial::ChecksumAlgorithmMismatchDenial;
pub use checksum_detection_model::{
    ChecksumAuthenticityPosture, ChecksumAuthorizationPosture, ChecksumCollisionPosture,
    ChecksumCorruptionClass, ChecksumDetectionModel,
};
pub(crate) use checksum_execution::execute_declared_checksum;
pub use checksum_execution::ExecutedPhysicalChecksum;
pub(crate) use checksum_foundational_identity::foundational_identity_for_checksum_basis;
pub use checksum_foundational_identity::FoundationalChecksumEvidenceIdentity;
pub use checksum_scope::ChecksumScopeDeclaration;
pub use chunk_integrity::ChunkIntegrityAuthority;
pub use chunk_integrity_counters::ChunkIntegrityCounters;
pub use chunk_integrity_denials::{
    ChunkDamageLocality, ChunkIntegrityDenial, ChunkIntegrityDenialKind,
    ChunkIntegrityStreamingWindowDenial,
};
pub use chunk_integrity_reports::{
    ChunkIntegrityInputIdentity, ChunkIntegrityLifecycleClaims, ChunkIntegrityReport,
};
pub use chunk_integrity_request::{ChunkIntegrityInspectionRequest, ChunkIntegrityStreamingWindow};
pub use compaction_source_clearance::{
    CompactionSourceClearanceDenial, CompactionSourceClearanceKind,
    CompactionSourceIntegrityClearance,
};
pub use container_integrity::PhysicalContainerIntegrity;
pub use container_integrity_boundaries::PhysicalBoundaryLocalization;
pub use container_integrity_counters::ContainerIntegrityCounters;
pub use container_integrity_denials::{
    AmbiguousBoundaryDamage, PhysicalContainerIntegrityDenial,
    PhysicalContainerIntegrityDenialKind, TornFrameDenial,
};
pub use container_integrity_reports::{
    ExtentIntegrityReport, FrameIntegrityReport, PageIntegrityReport, SlotDirectoryIntegrityReport,
    SlotStateIntegrityReport,
};
pub use damage_classification::{
    DamageClassification, IntactPhysicalBoundary, QuarantinedPhysicalDamage,
};
pub use damage_handoff::{
    classify_physical_damage_for_handoff, quarantine_handoff_posture, PhysicalDamageHandoffEvidence,
};
pub use entry_admission::IntegrityEntryAdmission;
pub use entry_basis::{IntegrityEntryBasis, ScrubEnvelopeLimits, VerifierResidentLimits};
pub use entry_denial::{IntegrityEntryDenial, IntegrityEntryDenialKind};
pub use entry_request::IntegrityEntryRequest;
pub use entry_witness::IntegrityEntryWitness;
pub use generation_integrity::GenerationIntegrityReport;
pub use index_page_integrity::DerivedIndexIntegrityAuthority;
pub use index_page_integrity_counters::IndexPageIntegrityCounters;
pub use index_page_integrity_denials::{IndexPageIntegrityDenial, IndexPageIntegrityDenialKind};
pub use index_page_integrity_reports::{
    AuthorityDamageBoundary, DerivedDamageClassification, DerivedRebuildInput,
    IndeterminatePhysicalDamage, IndexPageIntegrityReport, IntactIndexPageBoundary,
    RebuildabilityPrerequisite, RebuildableDerivedDamage, RebuildableDerivedDamagePrerequisites,
    UnrecoverableAuthorityDamage,
};
pub use index_page_integrity_request::DerivedIndexIntegrityInspectionRequest;
pub use inspection_lease::IntegrityInspectionLease;
pub use integrity_authority_claim_basis::{
    checkpoint_authority_digest, frame_authority_digest, manifest_authority_digest,
    page_authority_digest, wal_frame_authority_digest,
};
pub use integrity_checked_physical_form::{
    IntegrityCheckedFrame, IntegrityCheckedPage, IntegrityCheckedPhysicalFormKind,
};
pub use integrity_evidence_authority::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceEquivalence,
};
pub use integrity_evidence_bundle::{
    IntegrityDiagnosticReport, IntegrityEvidenceCounters, IntegrityEvidenceLocality,
    IntegrityEvidenceOutcome, IntegrityPerformanceReceipt, IntegrityProvenanceAttachment,
    PhysicalIntegrityCertificationReceipt, PhysicalIntegrityEvidenceBundle,
    StoreIntegrityBoundaryClaim,
};
pub use integrity_evidence_denial::PhysicalIntegrityEvidenceDenial;
pub use integrity_evidence_profile::{
    PhysicalIntegrityEvidenceProfile, PhysicalIntegrityEvidenceRichness,
};
pub use integrity_evidence_proof_report::{
    IntegrityProofProgressionOutcome, IntegrityProofProgressionReport,
    IntegrityProofProgressionSnapshot,
};
pub use integrity_evidence_roles::{
    FoundationalBoundaryRoleMapping, StoreDerivedProjectionBoundaryClaim,
    StorePhysicalAuthorityBoundaryClaim, StorePlannedWorkBoundaryClaim,
    StorePlannedWorkBoundaryKind, StorePlannedWorkBoundaryReport,
    StoreReceiptEvidenceBoundaryClaim, StoreSupportOnlyBoundaryClaim,
};
pub use integrity_evidence_source::{
    IntegrityEvidenceMaterializationPath, StoreExecutedIntegrityEvidence,
};
pub use logical_decode_gate::{
    LogicalDecodeGate, LogicalDecodeGateEvidence, LogicalDecodeGateIdentity, S3LogicalDecoder,
};
pub(crate) use manifest_allocation_map::allocation_map_report;
pub use manifest_integrity::ManifestIntegrityAuthority;
pub use manifest_integrity_counters::ManifestIntegrityCounters;
pub use manifest_integrity_denials::{
    ManifestGenerationMismatchDenial, ManifestIntegrityDenial, ManifestIntegrityDenialKind,
    ManifestReferenceMismatchDenial,
};
pub use manifest_integrity_reports::{
    AllocationMapIntegrityReport, ManifestIntegrityReport, ManifestReferenceBasis,
    RootManifestIntegrityReport, SegmentManifestIntegrityReport,
};
pub(crate) use manifest_integrity_request::ManifestRootIntegrityEvidence;
pub use manifest_integrity_request::{
    AuthoritativeManifestFailure, DerivedManifestOverrideAttempt, ManifestExpectedReference,
    ManifestIntegrityInspectionRequest,
};
pub(crate) use manifest_root_posture::admit_root_posture;
pub(crate) use manifest_source_precedence::deny_derived_override;
pub use offline_scrub_input::{
    OfflineScrubInspectionInput, OfflineScrubInspectionInputDenial, OfflineScrubVerifierBasis,
};
pub use physical_integrity_admission::{
    PhysicalIntegrityAdmission, PhysicalIntegrityAdmissionSeed,
};
pub use physical_integrity_request::{DeclaredPhysicalChecksum, PhysicalIntegrityAdmissionRequest};
pub use physical_scope_admission::PhysicalScopeAdmission;
pub use physical_scope_basis::PhysicalScopeBasis;
pub use physical_scope_denial::{
    ChecksumScopeMismatchDenial, IntactWrongScopeDenial, PhysicalScopeDenial,
    PhysicalScopeDenialKind,
};
pub use physical_scope_family_inputs::ScopedPhysicalValidatorInput;
pub use physical_scope_request::PhysicalScopeAdmissionRequest;
pub use pre_decode_counters::{
    PreDecodeAdmissionCounters, SemanticDecoderInvocationCounter, SkippedLogicalDecodeCounter,
};
#[cfg(any(test, feature = "test-support"))]
pub use pre_decode_denial::test_pre_decode_denial_for_kind;
pub use pre_decode_denial::{PreDecodePhysicalDenial, PreDecodePhysicalDenialKind};

pub use protected_physical_byte_view::ProtectedPhysicalByteView;
pub use quarantine_authority::PhysicalQuarantineAuthority;
pub use quarantine_denial::{QuarantineSealDenial, QuarantineSealDenialKind};
pub use quarantine_finding::ExecutedQuarantineFinding;
pub use quarantine_locality::{PhysicalLocalityReport, QuarantineLocalityBoundary};
pub use quarantine_posture::{QuarantineHandoffPosture, QuarantineLifecyclePosture};
pub use quarantine_receipt::{FoundationalQuarantineReceiptBasis, QuarantineReceipt};
pub use quarantine_record::QuarantineRecord;
pub use quarantine_request::QuarantineSealRequest;
pub use scrub_counters::ScrubCounterSnapshot;
pub use scrub_denial::{
    ScrubExecutionDenial, ScrubExecutionDenialKind, ScrubOverBudgetClass, ScrubPlanDenial,
    ScrubPlanDenialKind,
};
pub use scrub_execution::{
    ScrubExecution, ScrubExecutionReceipt, ScrubIntegrityFinding, ScrubProgressReport,
};
pub use scrub_scheduler_demand::scrub_scan_scheduler_demand;
pub use scrub_plan::{
    PlannedScrubWindow, PlannedScrubWindowStatus, ScrubPlan, ScrubPlanBudget, ScrubPlanRequest,
};
pub use scrub_planning_memory_envelope::{
    ScrubPlanningMemoryEnvelope, ScrubPlanningMemoryEnvelopeDenial,
};
pub use scrub_resume::ScrubResumeToken;
pub use scrub_window::{
    ScrubLocalitySummary, ScrubMode, ScrubWindow, ScrubWindowOrdinal, ScrubWindowSource,
};
pub use wal_frame_integrity::WalFrameIntegrityAuthority;
pub use wal_frame_integrity_counters::WalFrameIntegrityCounters;
pub use wal_frame_integrity_denials::{
    CheckpointAdjacentDamageDenial, WalFrameDamageDenial, WalFrameDamageDenialKind,
};
pub use wal_frame_integrity_reports::{
    CheckpointRecordIntegrityReport, WalFrameIntegrityInputIdentity, WalFrameIntegrityReport,
    WalTailIntegrityPosture,
};
pub use wal_frame_integrity_request::WalFrameIntegrityInspectionRequest;
