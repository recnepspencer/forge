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
//! use forge_store_certification::S2PhysicalSubstrateReadiness;
//! use forge_store_contracts::ROADMAP_2_S1_SCOPE;
//!
//! let _forged = S2PhysicalSubstrateReadiness {
//!     scope: ROADMAP_2_S1_SCOPE,
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

#![forbid(unsafe_code)]

mod binary_format_evidence;
mod certification_matrix;
mod drivers;
mod extent_record_framing_evidence;
#[cfg(test)]
mod extent_record_framing_evidence_tests;
mod harness;
#[cfg(test)]
mod harness_tests;
mod header_decode_evidence;
mod lanes;
mod layout_observers;
mod manifest_discovery_evidence;
#[cfg(test)]
mod manifest_discovery_evidence_tests;
mod observed_trace;
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
mod platform_facade_evidence;
#[cfg(test)]
mod platform_facade_evidence_tests;
mod runtime_verifier_comparison;
#[cfg(test)]
mod runtime_verifier_comparison_tests;
mod runtime_verifier_diagnostics;
mod runtime_verifier_support;
mod s2_physical_substrate_readiness;
mod scale_fixture;
mod scale_property;
mod scenario_definition;
mod scenario_execution;
mod scenario_plan;
mod scenario_plan_rules;
mod story_transcript;

pub use binary_format_evidence::{
    BinaryPhysicalFormatEvidence, BinaryPhysicalFormatEvidenceDenial,
};
pub use certification_matrix::S1CertificationRow;
pub use drivers::{PhysicalScenarioDriverKind, PhysicalScenarioDriverRequirement};
pub use extent_record_framing_evidence::{
    PhysicalExtentRecordFramingEvidenceDenial, PhysicalExtentRecordFramingEvidenceReport,
    PhysicalExtentRecordFramingEvidenceRow,
};
pub use harness::{PhysicalScenarioHarnessDenial, PhysicalScenarioQualityHarness};
pub use header_decode_evidence::{
    PhysicalHeaderDecodeEvidenceDenial, PhysicalHeaderDecodeEvidenceReport,
    PhysicalHeaderDecodeEvidenceRow,
};
pub use lanes::{LaneFamilyExtension, PhysicalSubstrateLane, RoadmapLaneFamily};
pub use layout_observers::{
    OfflineVerifierObserver, PhysicalLayoutParity, PhysicalLayoutParityDenial,
    PhysicalLayoutParityReport, RuntimeLayoutObserver,
};
pub use manifest_discovery_evidence::{
    PhysicalManifestDiscoveryEvidenceDenial, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow,
};
pub use observed_trace::{
    FixtureAdversaryPosture, FixtureAdversaryReport, ObservedPhysicalTrace,
    PhysicalCounterExpectationKind, RuntimeVerifierParityTrace, RuntimeVerifierRelationship,
    ScenarioCounterExpectation, ScenarioCounterObservation, ScenarioCounterTrace,
    ScenarioDenialBoundary, ScenarioDenialTrace, ShortcutRejectionTrace,
};
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
pub use platform_facade_evidence::{
    PlatformPhysicalFacadeEvidenceDenial, PlatformPhysicalFacadeEvidenceReport,
    PlatformPhysicalFacadeEvidenceRow,
};
pub use runtime_verifier_comparison::{
    PhysicalRuntimeVerifierComparison, RuntimeVerifierComparisonClassification,
    RuntimeVerifierComparisonDenial, RuntimeVerifierComparisonReport,
};
pub use runtime_verifier_diagnostics::{
    RuntimeVerifierDiagnosticDenial, RuntimeVerifierDiagnosticKind, RuntimeVerifierDiagnosticReport,
};
pub use runtime_verifier_support::{RuntimeVerifierSupportDenial, RuntimeVerifierSupportReport};
pub use s2_physical_substrate_readiness::S2PhysicalSubstrateReadiness;
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
pub use story_transcript::PhysicalStoryTranscript;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreCertificationProgram {
    Generic,
    Domain,
}
