mod authority_closeout;
mod bridge;
mod core;
mod derived_topology_closeout;
mod error;
mod facade;
mod primitive_corpus;
#[cfg(test)]
mod projection_closeout;
mod query_boundary_cleanup_closeout;
mod rejections;
mod requirements;
mod shared;
#[cfg(test)]
mod structure_guard;
#[cfg(test)]
mod structure_guard_support;
pub(crate) mod support;
mod tracing;

#[cfg(test)]
mod tests;
mod topology_operator_closeout;

pub use authority_closeout::read_view::MilestoneOneCertificationHarness;
pub use authority_closeout::read_view::TracedMilestoneOneCertificationReport;
pub use core::{
    CertificationBridgeExpectation, CertificationCanonicalRow, CertificationParityRow,
    CertificationRejectionRow, CertificationRequiredOutput, CertificationSuiteDefinition,
    CertificationSuiteRequirements, CertificationValidatorExpectation,
};
pub use derived_topology_closeout::TracedMilestoneTwoDerivedReadReport;
pub use error::{MilestoneOneCertificationError, TopologyCertificationError};
pub use facade::{
    certify_milestone_one_branch_local_primitive_scenarios, certify_milestone_one_closeout,
    certify_milestone_one_default_primitive_corpus, certify_milestone_one_primitive_corpus,
    certify_milestone_one_primitive_scenarios, certify_milestone_one_read_basis_traced,
    certify_milestone_three_ambiguous_local_rewire_continuity,
    certify_milestone_three_bowtie_adjacent_rewire,
    certify_milestone_three_broken_radial_localization,
    certify_milestone_three_cancellation_chain_parity, certify_milestone_three_closeout,
    certify_milestone_three_hostile_suite, certify_milestone_three_split_collapse_churn,
    certify_milestone_two_closeout, certify_milestone_two_default_derived_corpus,
    certify_milestone_two_read_basis_traced, certify_milestone_two_verified_topology_commit_traced,
    certify_topology_query_boundary_cleanup_closeout, certify_verified_topology_commit_traced,
};
pub use query_boundary_cleanup_closeout::{
    TopologyQueryBoundaryCleanupArea, TopologyQueryBoundaryCleanupCloseoutReport,
    TopologyQueryBoundaryCleanupRow, TopologyQueryBoundaryCleanupStatus,
};
pub use requirements::{
    milestone_one_closeout_requirements, milestone_one_closeout_suite_definition,
    milestone_three_closeout_requirements, milestone_three_closeout_suite_definition,
    milestone_two_closeout_requirements, milestone_two_closeout_suite_definition,
};
pub use support::reporting::{
    AdmittedRangeSweepReport, AdmittedRangeSweepRow, BranchLocalTopologyReport,
    BridgeFamilyCoverageReport, BridgeFamilyCoverageRow, BridgeProofReport,
    DerivedEquivalenceContractAggregateReport, DerivedEquivalenceContractAggregateRow,
    DerivedFallbackAggregateReport, DerivedFallbackAggregateRow, DerivedFamilyCoverageMatrix,
    DerivedFamilyCoverageRow, DerivedFamilyParityMatrix, DerivedFamilyParityRow,
    DerivedInvalidationAggregateReport, DerivedInvalidationAggregateRow,
    DerivedRebuildAggregateReport, DerivedRebuildAggregateRow, DerivedValidatorCoverageReport,
    DerivedValidatorCoverageRow, DeterministicDigest, FailureLocalityReport, FailureLocalityRow,
    IllegalTopologyRejectionCaseReport, IllegalTopologyRejectionReport,
    MilestoneOneBranchLocalAggregateReport, MilestoneOneCertificationReport,
    MilestoneOneCloseoutReport, MilestoneOneCounters, MilestoneOneRejectionClassReport,
    MilestoneOneRejectionClassRow, MilestoneOneReplayAggregateReport,
    MilestoneOneValidationAggregateReport, MilestoneOneValidationAggregateRow,
    MilestoneOneValidatorCoverageReport, MilestoneOneValidatorCoverageRow,
    MilestoneTwoBranchLocalParityReport, MilestoneTwoCloseoutReport, MilestoneTwoCounters,
    MilestoneTwoDerivedCorpusReport, MilestoneTwoDerivedReadReport, MilestoneTwoReplayParityReport,
    NamingAttachmentAggregateReport, NamingAttachmentAggregateRow, NamingAttachmentReport,
    NamingAttachmentRow, PrimitiveCorpusCaseReport, PrimitiveCorpusCoverageEntry,
    PrimitiveCorpusCoverageMatrix, PrimitiveCorpusParityEntry, PrimitiveCorpusParityReport,
    PrimitiveCorpusRejectedCaseReport, PrimitiveCorpusReport, PrimitiveFamilyCoverageEntry,
    PrimitiveFamilyCoverageMatrix, PrimitiveRejectionReport, ReplayParityReport,
    ReplayParityStatus, TopologyLocalizationAggregateEntityRow,
    TopologyLocalizationAggregateRelationRow, TopologyLocalizationAggregateReport,
    TopologyLocalizationEntityRow, TopologyLocalizationRelationRow, TopologyLocalizationReport,
};
pub use topology_operator_closeout::{
    MilestoneThreeAmbiguousLocalRewireWitness, MilestoneThreeBowtieAdjacentWitness,
    MilestoneThreeBrokenRadialWitness, MilestoneThreeChangedScopeCoverageRow,
    MilestoneThreeDerivedFallbackPolicyDenialRow, MilestoneThreeDerivedRegionCoverageRow,
    MilestoneThreeDerivedReuseLegalityRow, MilestoneThreeDerivedWorkBreadthClass,
    MilestoneThreeDerivedWorkBreadthRow, MilestoneThreeDeterminismRuleKind,
    MilestoneThreeDeterminismRuleRow, MilestoneThreeFailureLocalityRow,
    MilestoneThreeHostileCertificationCategory, MilestoneThreeHostileCertificationCategoryRow,
    MilestoneThreeHostileCertificationStatus, MilestoneThreeHostileCoverageRow,
    MilestoneThreeHostileFamilyCoverageRow, MilestoneThreeHostileNamingDistributionRow,
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileRejectionDistributionRow,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
    MilestoneThreeHostileSuiteReport, MilestoneThreeMutationBranchLocalParityRow,
    MilestoneThreeMutationBreadthCounterRow, MilestoneThreeMutationFalloutBreadthRow,
    MilestoneThreeMutationFalloutClass, MilestoneThreeMutationReplayParityReport,
    MilestoneThreeMutationReplayParityRow, MilestoneThreeMutationReplayStepRow,
    MilestoneThreeMutationTopologyQueryTraversalRow,
    MilestoneThreeMutationTopologyQueryTraversalView, MilestoneThreeNamingContinuityBreadthRow,
    MilestoneThreeNamingContinuityMatrixRow, MilestoneThreeOperatorFamilyClosureRow,
    MilestoneThreePrimitiveFamilyClosureRow, MilestoneThreeRejectedMutationScopeReportRow,
    MilestoneThreeReplayBranchBreadthRow, MilestoneThreeReturnGateBlockerRow,
    MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep,
    MilestoneThreeSideQuestBlockerRow, MilestoneThreeSideQuestCloseoutReport,
    MilestoneThreeSideQuestContractRow, MilestoneThreeSplitCollapseChurnWitness,
    MilestoneThreeTopologyMutationDigestRow, MilestoneThreeValidationBreadthRow,
    MilestoneThreeValidatorFamily, MilestoneThreeValidatorFamilyCoverageRow,
};
pub use tracing::{
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure,
    BridgeTraceAnchor, DecisionTrace, DerivedTraceAnchor, DerivedTraceEvidence, IntegrityMarkers,
    NamedCounter, PerformanceAccounting, TraceAvailability, TraceWarning,
};
