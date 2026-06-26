//! Store certification vocabulary.
//!
//! Raw Store digests cannot satisfy canonical artifact evidence:
//!
//! ```compile_fail
//! use forge_store_certification::PhysicalFoundationEvidenceBundle;
//! use forge_store_contracts::{StableArtifactId, StableDigest};
//! use forge_store_readiness::FoundationalVocabularyAdoptionMap;
//!
//! let adoption = FoundationalVocabularyAdoptionMap::s1_all_public_lanes().unwrap();
//! let raw_digest = StableDigest::new("sha256:raw-store-digest").unwrap();
//!
//! let _builder = PhysicalFoundationEvidenceBundle::builder(adoption)
//!     .with_canonical_artifact_digest(
//!         StableArtifactId::new("artifact_digest").unwrap(),
//!         raw_digest,
//!     );
//! ```
//!
//! Public callers cannot skip the scenario harness progression:
//!
//! ```compile_fail
//! use forge_store_certification::{
//!     PhysicalProofOracleKind, PhysicalScenarioDefinition, PhysicalScenarioExecution,
//!     PhysicalScenarioQualityHarness, PhysicalStoryStep, PhysicalSubstrateLane,
//! };
//!
//! let definition = PhysicalScenarioDefinition::story("direct_execution_forge")
//!     .physical_substrate_lane(PhysicalSubstrateLane::HappyAuthority)
//!     .proves_law("external callers must not mint execution directly")
//!     .step(PhysicalStoryStep::GivenCleanPhysicalStore)
//!     .requires_oracle(PhysicalProofOracleKind::ScenarioPlanOwnsStrategy)
//!     .define()
//!     .unwrap();
//! let harness = PhysicalScenarioQualityHarness::roadmap_2();
//! let plan = harness.lower(definition).unwrap();
//!
//! let _forged = PhysicalScenarioExecution::from_plan(plan);
//! ```
//!
//! Raw digests cannot be supplied as binary format evidence:
//!
//! ```compile_fail
//! use forge_store_certification::BinaryPhysicalFormatEvidence;
//! use forge_store_contracts::StableDigest;
//! use forge_store_physical_format::PhysicalBinaryEncodingWitness;
//!
//! let witness = PhysicalBinaryEncodingWitness::s1_canonical().unwrap();
//! let digest = StableDigest::new("sha256:raw-binary-format").unwrap();
//! let _evidence = BinaryPhysicalFormatEvidence::from_witness(&witness, digest);
//! ```
//!
//! S.2 readiness is minted only by admitted S.1 physical substrate closeout:
//!
//! ```compile_fail
//! use forge_store_readiness::S2PhysicalSubstrateReadiness;
//! use forge_store_contracts::ROADMAP_2_S1_SCOPE;
//!
//! let _forged = S2PhysicalSubstrateReadiness {
//!     scope: ROADMAP_2_S1_SCOPE,
//!     facts: todo!(),
//!     sealed: true,
//! };
//! ```
//!
//! Raw closeout evidence descriptors cannot be assembled outside certification:
//!
//! ```compile_fail
//! use forge_store_certification::PhysicalPageSegmentExtentSubstrateEvidence;
//!
//! let _forged = PhysicalPageSegmentExtentSubstrateEvidence::new(
//!     unimplemented!(),
//!     unimplemented!(),
//!     unimplemented!(),
//!     unimplemented!(),
//!     unimplemented!(),
//!     unimplemented!(),
//!     unimplemented!(),
//!     unimplemented!(),
//!     unimplemented!(),
//!     unimplemented!(),
//! );
//! ```
//!
//! Raw closeout runs cannot be assembled outside certification:
//!
//! ```compile_fail
//! use forge_store_certification::PhysicalPageSegmentExtentSubstrateRun;
//! use forge_store_contracts::StableArtifactId;
//!
//! let _forged = PhysicalPageSegmentExtentSubstrateRun::new(
//!     StableArtifactId::new("synthetic-closeout").unwrap(),
//!     unimplemented!(),
//! );
//! ```
//!
//! S.3 physical integrity readiness cannot be minted from raw payload fields:
//!
//! ```compile_fail
//! use forge_store_certification::S3PhysicalIntegrityReadiness;
//!
//! let _forged = S3PhysicalIntegrityReadiness::from_s2_bounded_residency_closeout(
//!     todo!(),
//!     todo!(),
//! );
//! ```

#![doc = include_str!("receipt_authority_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

mod allocation_envelope_evidence;
#[cfg(test)]
mod allocation_envelope_evidence_tests;
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
mod certification_matrix;
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
mod harness;
#[cfg(test)]
mod harness_tests;
mod header_decode_evidence;
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
mod physical_foundation_evidence;
mod physical_identity_evidence;
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
mod protected_integrity_view_evidence;
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
mod s2_acceptance_suite_transcript;
mod s2_entry_boundary_evidence;
mod s3_readiness_handoff;
mod scale_fixture;
mod scale_property;
mod scenario_definition;
mod scenario_execution;
mod scenario_plan;
mod scenario_plan_rules;
mod speculative_work_evidence;
#[cfg(test)]
mod speculative_work_evidence_tests;
mod story_transcript;
mod synthetic_closeout_rejection;

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
pub use forge_store_readiness::S2PhysicalSubstrateReadiness;
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
pub use physical_identity_evidence::PhysicalIdentityEvidenceRow;
pub use physical_identity_evidence::{
    PhysicalIdentityEvidenceDenial, PhysicalIdentityEvidenceReport,
};
pub use physical_substrate_certification_authority::{
    certify_physical_page_segment_extent_substrate, certify_s2_physical_substrate_readiness,
};
pub use physical_substrate_certification_denial::PhysicalSubstrateCertificationDenial;
pub use physical_substrate_closeout::{
    PhysicalPageSegmentExtentSubstrateCloseout, PhysicalPageSegmentExtentSubstrateEvidence,
    PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCloseoutDenial,
};
pub use physical_substrate_closeout_story::{
    PhysicalSubstrateCloseoutStoryDenial, PhysicalSubstrateCloseoutStoryReport,
    PhysicalSubstrateCloseoutStoryRow,
};
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
pub use s2_acceptance_suite_transcript::S2AcceptanceSuiteKind;
pub use s2_entry_boundary_evidence::{
    S2EntryBoundaryEvidenceDenial, S2EntryBoundaryEvidenceReport, S2EntryBoundaryEvidenceRow,
    S2ForbiddenEntryAttempt,
};
pub use s3_readiness_handoff::S3PhysicalIntegrityReadiness;
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
pub use speculative_work_evidence::{
    SpeculativeWorkEvidenceDenial, SpeculativeWorkEvidenceReport, SpeculativeWorkEvidenceRow,
};
pub use story_transcript::PhysicalStoryTranscript;
pub use synthetic_closeout_rejection::{
    SyntheticCloseoutRejectionDenial, SyntheticCloseoutShortcutAttempt,
    SyntheticCloseoutShortcutRejectionReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreCertificationProgram {
    Generic,
    Domain,
}
