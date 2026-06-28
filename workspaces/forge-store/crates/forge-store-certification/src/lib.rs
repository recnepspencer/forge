#![doc = include_str!("certification_compile_fail_proofs.md")]
#![doc = include_str!("receipt_authority_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

mod allocation_envelope_evidence;
#[cfg(test)]
mod allocation_envelope_evidence_tests;
#[cfg(test)]
mod aspect_native_authority_denial_tests;
#[cfg(test)]
mod aspect_native_diagnostic_evidence_tests;
#[cfg(test)]
mod aspect_native_harness_authoring_tests;
#[cfg(test)]
mod aspect_native_identity_tests;
#[cfg(test)]
mod aspect_native_performance_evidence_tests;
#[cfg(test)]
mod aspect_native_vocabulary_tests;
#[cfg(test)]
mod authority_projection_readmission_tests;
mod background_envelope_evidence;
#[cfg(test)]
mod background_envelope_evidence_tests;
mod binary_format_evidence;
mod bounded_memory_closeout;
#[cfg(test)]
mod bounded_memory_closeout_pressure_support;
#[cfg(test)]
mod bounded_memory_closeout_test_support;
#[cfg(test)]
mod bounded_memory_closeout_tests;
mod bounded_memory_harness_closeout;
mod bounded_memory_residency_suite;
mod buffer_pool_certification_bundle;
mod buffer_pool_scenario_definitions;
mod buffer_pool_scenario_plans;
mod buffer_pool_story_lanes;
mod buffer_pool_transcripts;
#[cfg(test)]
mod canonical_basis_entry_construction_tests;
#[cfg(test)]
mod canonical_basis_entry_denial_tests;
#[cfg(test)]
mod canonical_basis_entry_order_tests;
mod canonical_basis_source_inventory;
mod canonical_basis_source_registry;
mod canonical_basis_source_scan;
#[cfg(test)]
mod canonical_basis_source_tests;
mod certification_matrix;
#[cfg(test)]
mod checksum_declaration_tests;
#[cfg(test)]
mod chunk_integrity_without_blob_lifecycle_tests;
#[cfg(test)]
mod cross_family_wrong_scope_tests;
#[cfg(test)]
mod derived_index_damage_tests;
#[cfg(test)]
mod digest_authority_denial_tests;
#[cfg(test)]
mod digest_authority_equivalence_tests;
mod dirty_publication_evidence;
#[cfg(test)]
mod dirty_publication_evidence_test_support;
#[cfg(test)]
mod dirty_publication_evidence_tests;
mod drivers;
mod eviction_protection_evidence;
#[cfg(test)]
mod eviction_protection_evidence_tests;
mod extent_record_framing_evidence;
#[cfg(test)]
mod extent_record_framing_evidence_tests;
mod foundational_boundary_evidence;
#[cfg(test)]
mod foundational_boundary_evidence_tests;
mod foundational_boundary_performance;
#[cfg(test)]
mod foundational_integrity_evidence_tests;
mod harness;
#[cfg(test)]
mod harness_tests;
mod header_decode_evidence;
#[cfg(test)]
mod hostile_readmission_json_fixture_boundary_tests;
#[cfg(test)]
mod json_fixture_boundary_tests;
mod lanes;
mod large_store_pressure_evidence;
#[cfg(test)]
mod large_store_pressure_tests;
mod layout_observers;
mod manifest_discovery_evidence;
#[cfg(test)]
mod manifest_discovery_evidence_tests;
mod observed_trace;
mod observer_trace;
mod observers;
mod offline_verifier_evidence;
#[cfg(test)]
mod offline_verifier_evidence_tests;
mod oracles;
mod page_record_framing_evidence;
#[cfg(test)]
mod page_record_framing_evidence_tests;
mod physical_complexity_evidence;
#[cfg(test)]
mod physical_complexity_evidence_tests;
#[cfg(test)]
mod physical_container_integrity_hardening_tests;
#[cfg(test)]
mod physical_container_integrity_test_support;
#[cfg(test)]
mod physical_container_integrity_tests;
mod physical_foundation_evidence;
mod physical_identity_evidence;
mod physical_integrity_closeout_bundle;
mod physical_integrity_closeout_denial;
mod physical_integrity_closeout_exports;
mod physical_integrity_closeout_handoff;
mod physical_integrity_closeout_harness;
mod physical_integrity_closeout_harness_execution;
mod physical_integrity_closeout_harness_runner;
#[cfg(test)]
mod physical_integrity_closeout_harness_test_support;
mod physical_integrity_closeout_line_cap;
#[cfg(test)]
mod physical_integrity_closeout_line_cap_test_support;
#[cfg(test)]
mod physical_integrity_closeout_line_cap_tests;
mod physical_integrity_closeout_owned_file;
mod physical_integrity_closeout_proof;
mod physical_integrity_closeout_report;
mod physical_integrity_closeout_suite;
mod physical_integrity_closeout_suite_kind;
#[cfg(test)]
mod physical_integrity_closeout_test_support;
#[cfg(test)]
mod physical_integrity_closeout_tests;
#[cfg(test)]
mod physical_integrity_entry_authority_tests;
#[cfg(test)]
mod physical_scope_admission_test_support;
#[cfg(test)]
mod physical_scope_admission_tests;
mod physical_substrate_certification_authority;
mod physical_substrate_certification_denial;
mod physical_substrate_certification_reports;
mod physical_substrate_certification_scan;
mod physical_substrate_closeout;
mod physical_substrate_closeout_story;
#[cfg(test)]
mod physical_substrate_closeout_tests;
mod physical_substrate_complexity_suite;
mod physical_substrate_foundation_suite;
mod physical_substrate_manifest_suite;
mod physical_substrate_story_suite;
mod pin_lifecycle_evidence;
#[cfg(test)]
mod pin_lifecycle_evidence_tests;
mod platform_facade_evidence;
#[cfg(test)]
mod platform_facade_evidence_tests;
#[cfg(test)]
mod pre_decode_physical_admission_test_support;
#[cfg(test)]
mod pre_decode_physical_admission_tests;
mod protected_integrity_view_evidence;
#[cfg(test)]
mod public_facade_dependency_tests;
#[cfg(test)]
mod quarantine_sealing_tests;
mod record_view_evidence;
#[cfg(test)]
mod record_view_evidence_admission_tests;
#[cfg(test)]
mod record_view_evidence_conflict_tests;
#[cfg(test)]
mod record_view_evidence_test_support;
mod resident_frame_authority_evidence;
#[cfg(test)]
mod resident_frame_authority_evidence_tests;
mod runtime_verifier_comparison;
#[cfg(test)]
mod runtime_verifier_comparison_tests;
mod runtime_verifier_diagnostics;
mod runtime_verifier_support;
#[cfg(test)]
mod s0_handoff_contract_tests;
mod s0_handoff_gate_evidence;
mod s2_acceptance_suite_transcript;
mod s2_entry_boundary_evidence;
#[cfg(test)]
mod s3_integrity_readiness_test_support;
mod s3_readiness_handoff;
#[cfg(test)]
mod s4_integrity_damage_map_tests;
#[cfg(test)]
mod s4_integrity_handoff_test_support;
#[cfg(test)]
mod s4_integrity_handoff_tests;
#[cfg(test)]
mod s4_quarantine_receipt_binding_tests;
#[cfg(test)]
mod s4_recovery_blocking_damage_test_support;
#[cfg(test)]
mod s4_recovery_entry_admission_tests;
mod s4_recovery_harness;
mod s4_recovery_harness_exports;
mod scale_fixture;
mod scale_property;
mod scenario_definition;
mod scenario_execution;
mod scenario_plan;
mod scenario_plan_rules;
mod scenario_planned_work_evidence;
#[cfg(test)]
mod scrub_execution_tests;
mod speculative_work_evidence;
#[cfg(test)]
mod speculative_work_evidence_tests;
mod store_certification_program;
mod store_json_residue_certification;
mod store_json_residue_denial;
mod store_json_residue_entry;
mod store_json_residue_exports;
mod store_json_residue_inventory;
mod store_json_residue_prelude_scan;
mod store_json_residue_scan;
#[cfg(test)]
mod store_json_residue_tests;
mod story_transcript;
mod synthetic_closeout_exports;
mod synthetic_closeout_rejection;
#[cfg(test)]
mod terminal_projection_json_fixture_boundary_tests;
#[cfg(test)]
mod wal_frame_integrity_tests;

pub use allocation_envelope_evidence::{
    AllocationEnvelopeEvidenceDenial, AllocationEnvelopeEvidenceReport,
    AllocationEnvelopeEvidenceRow,
};
pub use background_envelope_evidence::{
    BackgroundClassEnvelopeEvidence, BackgroundEnvelopeEvidenceBundle,
    BackgroundEnvelopeEvidenceDenial, RequiredInterferenceKind,
};
pub use binary_format_evidence::{
    BinaryPhysicalFormatEvidence, BinaryPhysicalFormatEvidenceDenial,
};
pub use bounded_memory_closeout::{BoundedMemoryCloseoutDenial, BoundedMemoryCloseoutReport};
pub use bounded_memory_harness_closeout::{
    HarnessCloseoutEvidenceReport, HarnessCloseoutTranscriptEvidence,
};
pub use bounded_memory_residency_suite::{
    BoundedMemoryOperationKind, BoundedMemoryResidencySuite, BoundedMemoryResidencySuiteDenial,
    BoundedOperationEnvelopeCounters, BoundedOperationEnvelopeReport, S2BoundaryDenialKind,
};
pub use buffer_pool_certification_bundle::{
    BufferPoolCertificationBundle, BufferPoolCertificationBundleDenial,
};
pub use buffer_pool_scenario_definitions::{
    LargeStoreMemoryPressureScenario, LargeStoreScenarioDenial,
};
pub use buffer_pool_scenario_plans::{BufferPoolScenarioPlan, BufferPoolScenarioPlanDenial};
pub use buffer_pool_transcripts::BufferPoolPressureTranscriptIdentity;
pub use canonical_basis_source_inventory::{
    certify_scanned_store_canonical_basis_source_inventory,
    certify_store_canonical_basis_source_inventory, certify_store_canonical_basis_source_rows,
    current_store_canonical_basis_inventory, StoreCanonicalBasisInventoryDenial,
    StoreCanonicalBasisInventoryRow,
};
pub use certification_matrix::S1CertificationRow;
pub use dirty_publication_evidence::{
    DirtyPublicationEvidenceDenial, DirtyPublicationEvidenceReport, DirtyPublicationEvidenceRow,
};
pub use drivers::{PhysicalScenarioDriverKind, PhysicalScenarioDriverRequirement};
pub use eviction_protection_evidence::{
    EvictionProtectionEvidenceDenial, EvictionProtectionEvidenceReport,
    EvictionProtectionEvidenceRow,
};
pub use extent_record_framing_evidence::{
    PhysicalExtentRecordFramingEvidenceDenial, PhysicalExtentRecordFramingEvidenceReport,
    PhysicalExtentRecordFramingEvidenceRow,
};
pub use forge_store_readiness::{S2PhysicalSubstrateReadiness, S3PhysicalIntegrityReadiness};
pub use foundational_boundary_evidence::{
    AllocationEnvelopePerformanceReceipt, BufferPoolProvenanceAttachment,
    CompletedResidencyBoundaryReceipt, CopyMaterializationPerformanceReceipt,
    FoundationalBoundaryAuthorityResult, FoundationalBoundaryEvidenceDenial,
    FoundationalEvidenceProfile, FoundationalEvidenceRichness, MaterializationProfileReport,
    ResidentMemoryPerformanceReceipt, ZeroCopyLayoutPostureReport,
};
pub use harness::{PhysicalScenarioHarnessDenial, PhysicalScenarioQualityHarness};
pub use header_decode_evidence::{
    PhysicalHeaderDecodeEvidenceDenial, PhysicalHeaderDecodeEvidenceReport,
    PhysicalHeaderDecodeEvidenceRow,
};
pub use lanes::{LaneFamilyExtension, PhysicalSubstrateLane, RoadmapLaneFamily};
pub use large_store_pressure_evidence::{
    LargeStorePressureEvidenceBundle, LargeStorePressureEvidenceDenial, LargeStoreShortcutAttempt,
};
pub use layout_observers::{
    OfflineVerifierObserver, PhysicalLayoutParity, PhysicalLayoutParityDenial,
    PhysicalLayoutParityReport, RuntimeLayoutObserver,
};
pub use manifest_discovery_evidence::{
    PhysicalManifestDiscoveryEvidenceDenial, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow,
};
pub use observed_trace::{
    FixtureAdversaryPosture, FixtureAdversaryReport, LargeStorePressureClass,
    ObservedPhysicalTrace, PhysicalCounterExpectationKind, RuntimeVerifierParityTrace,
    RuntimeVerifierRelationship, ScenarioCounterExpectation, ScenarioCounterObservation,
    ScenarioCounterTrace, ScenarioDenialBoundary, ScenarioDenialTrace, ShortcutRejectionTrace,
};
pub use observer_trace::ScenarioObserverTrace;
pub use observers::{PhysicalScenarioObserverKind, PhysicalScenarioObserverRequirement};
pub use offline_verifier_evidence::{
    offline_observer_requires_physical_references, PhysicalOfflineVerifierEvidenceDenial,
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
};
pub use oracles::{
    PhysicalOracleDenialKind, PhysicalOracleJudgment, PhysicalOracleOutcome,
    PhysicalProofOracleKind, PhysicalProofOracleVerdict,
};
pub use page_record_framing_evidence::{
    PhysicalPageRecordFramingEvidenceDenial, PhysicalPageRecordFramingEvidenceReport,
    PhysicalPageRecordFramingEvidenceRow,
};
pub use physical_complexity_evidence::{
    PhysicalComplexityEvidenceDenial, PhysicalComplexityEvidenceReport,
    PhysicalComplexityProofBundle,
};
pub use physical_foundation_evidence::{
    PhysicalFoundationEvidenceBundle, PhysicalFoundationEvidenceBundleBuilder,
    PhysicalFoundationEvidenceDenial, PhysicalFoundationEvidenceEntry,
    PhysicalFoundationEvidenceIdentity,
};
pub use physical_identity_evidence::{
    PhysicalIdentityEvidenceDenial, PhysicalIdentityEvidenceReport, PhysicalIdentityEvidenceRow,
};
pub use physical_integrity_closeout_exports::*;
pub use pin_lifecycle_evidence::{
    PinLifecycleEvidenceDenial, PinLifecycleEvidenceReport, PinLifecycleEvidenceRow,
};
pub use platform_facade_evidence::{
    PlatformPhysicalFacadeEvidenceDenial, PlatformPhysicalFacadeEvidenceReport,
    PlatformPhysicalFacadeEvidenceRow,
};
pub use protected_integrity_view_evidence::{
    ProtectedIntegrityViewEvidence, ProtectedIntegrityViewEvidenceDenial,
};
pub use record_view_evidence::{
    RecordViewEvidenceDenial, RecordViewEvidenceReport, RecordViewEvidenceRow,
};
pub use resident_frame_authority_evidence::{
    ResidentFrameAuthorityEvidenceDenial, ResidentFrameAuthorityEvidenceReport,
    ResidentFrameAuthorityEvidenceRow,
};
pub use runtime_verifier_comparison::{
    PhysicalRuntimeVerifierComparison, RuntimeVerifierComparisonClassification,
    RuntimeVerifierComparisonDenial, RuntimeVerifierComparisonReport,
};
pub use runtime_verifier_diagnostics::{
    RuntimeVerifierDiagnosticDenial, RuntimeVerifierDiagnosticKind, RuntimeVerifierDiagnosticReport,
};
pub use runtime_verifier_support::{RuntimeVerifierSupportDenial, RuntimeVerifierSupportReport};
pub use s0_handoff_gate_evidence::{
    certify_s0_handoff_gate_proof_evidence, S0HandoffGateCertificationDenial,
};
pub use s2_acceptance_suite_transcript::S2AcceptanceSuiteKind;
pub use s2_entry_boundary_evidence::{
    S2EntryBoundaryEvidenceDenial, S2EntryBoundaryEvidenceReport, S2EntryBoundaryEvidenceRow,
    S2ForbiddenEntryAttempt,
};
pub use s4_recovery_harness_exports::*;
pub use scale_fixture::{
    PhysicalHostileScaleCondition, PhysicalHostileScaleFixtureDenial,
    PhysicalHostileScaleFixtureReport, PhysicalHostileScaleFixtureSource,
};
pub use scale_property::PhysicalScalePropertyEvidence;
pub use scenario_definition::{
    PhysicalScenarioDefinition, PhysicalScenarioDefinitionBuilder,
    PhysicalScenarioDefinitionDenial, PhysicalStoryStep, ScenarioLane,
};
pub use scenario_execution::{PhysicalScenarioExecution, PhysicalScenarioExecutionReport};
pub use scenario_plan::{
    ArtifactPolicy, ExpectedPhysicalFootprint, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioPlan, PhysicalScenarioPlanDenial,
    PhysicalScenarioPlanIdentity, StorageBoundaryCrossing, WorkloadScale,
};
pub use scenario_planned_work_evidence::PhysicalScenarioPlannedWorkBoundaryReport;
pub use speculative_work_evidence::{
    SpeculativeWorkEvidenceDenial, SpeculativeWorkEvidenceReport, SpeculativeWorkEvidenceRow,
};
pub use store_certification_program::StoreCertificationProgram;
pub use store_json_residue_exports::*;
pub use story_transcript::PhysicalStoryTranscript;
pub use synthetic_closeout_exports::*;
