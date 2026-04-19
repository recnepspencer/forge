//! Public API boundary for `worth-topo`.

pub use crate::bridge::{
    build_worth_milestone_one_bridge, worth_milestone_one_bridge_aspect_registrations,
    worth_milestone_one_bridge_mapping_registrations,
};
pub use crate::certification::{
    certify_milestone_one_branch_local_primitive_scenarios, certify_milestone_one_closeout,
    certify_milestone_one_default_primitive_corpus, certify_milestone_one_primitive_corpus,
    certify_milestone_one_primitive_scenarios, certify_milestone_one_read_view_traced,
    certify_milestone_two_closeout, certify_milestone_two_default_derived_corpus,
    certify_milestone_two_read_view_traced, certify_milestone_two_verified_topology_commit_traced,
    certify_verified_topology_commit_traced, milestone_one_closeout_requirements,
    milestone_one_closeout_suite_definition, milestone_two_closeout_requirements,
    milestone_two_closeout_suite_definition, WorthAdmittedRangeSweepReport,
    WorthAdmittedRangeSweepRow, WorthBranchLocalTopologyReport, WorthBridgeFamilyCoverageReport,
    WorthBridgeFamilyCoverageRow, WorthBridgeProofReport, WorthCertificationBridgeExpectation,
    WorthCertificationCanonicalRow, WorthCertificationParityRow, WorthCertificationRejectionRow,
    WorthCertificationRequiredOutput, WorthCertificationSuiteDefinition,
    WorthCertificationSuiteRequirements, WorthCertificationValidatorExpectation,
    WorthDerivedEquivalenceContractAggregateReport, WorthDerivedEquivalenceContractAggregateRow,
    WorthDerivedFallbackAggregateReport, WorthDerivedFallbackAggregateRow,
    WorthDerivedFamilyCoverageMatrix, WorthDerivedFamilyCoverageRow,
    WorthDerivedFamilyParityMatrix, WorthDerivedFamilyParityRow,
    WorthDerivedInvalidationAggregateReport, WorthDerivedInvalidationAggregateRow,
    WorthDerivedRebuildAggregateReport, WorthDerivedRebuildAggregateRow,
    WorthDerivedValidatorCoverageReport, WorthDerivedValidatorCoverageRow,
    WorthDeterministicDigest, WorthFailureLocalityReport, WorthFailureLocalityRow,
    WorthIllegalTopologyRejectionCaseReport, WorthIllegalTopologyRejectionReport,
    WorthMilestoneOneBranchLocalAggregateReport, WorthMilestoneOneCertificationError,
    WorthMilestoneOneCertificationHarness, WorthMilestoneOneCertificationReport,
    WorthMilestoneOneCloseoutReport, WorthMilestoneOneCounters,
    WorthMilestoneOneRejectionClassReport, WorthMilestoneOneRejectionClassRow,
    WorthMilestoneOneReplayAggregateReport, WorthMilestoneOneValidationAggregateReport,
    WorthMilestoneOneValidationAggregateRow, WorthMilestoneOneValidatorCoverageReport,
    WorthMilestoneOneValidatorCoverageRow, WorthMilestoneTwoBranchLocalParityReport,
    WorthMilestoneTwoCloseoutReport, WorthMilestoneTwoCounters,
    WorthMilestoneTwoDerivedCorpusReport, WorthMilestoneTwoDerivedReadReport,
    WorthMilestoneTwoReplayParityReport, WorthNamingAttachmentAggregateReport,
    WorthNamingAttachmentAggregateRow, WorthNamingAttachmentReport, WorthNamingAttachmentRow,
    WorthPrimitiveCorpusCaseReport, WorthPrimitiveCorpusCoverageEntry,
    WorthPrimitiveCorpusCoverageMatrix, WorthPrimitiveCorpusParityEntry,
    WorthPrimitiveCorpusParityReport, WorthPrimitiveCorpusRejectedCaseReport,
    WorthPrimitiveCorpusReport, WorthPrimitiveFamilyCoverageEntry,
    WorthPrimitiveFamilyCoverageMatrix, WorthPrimitiveRejectionReport, WorthReplayParityReport,
    WorthReplayParityStatus, WorthTopologyLocalizationAggregateEntityRow,
    WorthTopologyLocalizationAggregateRelationRow, WorthTopologyLocalizationAggregateReport,
    WorthTopologyLocalizationEntityRow, WorthTopologyLocalizationRelationRow,
    WorthTopologyLocalizationReport, WorthTracedMilestoneOneCertificationReport,
    WorthTracedMilestoneTwoDerivedReadReport,
};
pub use crate::data::topology_view::{
    WorthTopologyBody, WorthTopologyEdge, WorthTopologyFace, WorthTopologyHalfEdge,
    WorthTopologyLoop, WorthTopologyLump, WorthTopologyModel, WorthTopologyRegion,
    WorthTopologyShell, WorthTopologyVertex, WorthTopologyView, WorthTopologyWire,
};
pub use crate::diagnostics::{
    build_derived_fallback_report, build_derived_invalidation_report,
    build_derived_read_diagnostics, build_derived_rebuild_report, WorthDerivedFallbackReport,
    WorthDerivedInvalidationReport, WorthDerivedInvalidationTargetRow, WorthDerivedReadDiagnostics,
    WorthDerivedRebuildReport,
};
pub use crate::edit::{
    WorthBoundaryMembershipKind, WorthLoopEndpointKind, WorthLoopSuccessorKind,
    WorthShellOrWireMembershipKind, WorthTopologyDerivedRegion, WorthTopologyEditAction,
    WorthTopologyEditApplicationMode, WorthTopologyEditApplied, WorthTopologyEditBatch,
    WorthTopologyEditChangedScope, WorthTopologyEditContract, WorthTopologyEditError,
    WorthTopologyEditFamily, WorthTopologyEditNamingOutcome, WorthTopologyEditNamingReport,
    WorthTopologyEditNamingRow, WorthTopologyEditNamingScope, WorthTopologyEditRunner,
    WorthTopologyEditRuntimeTrace, WorthTracedTopologyEditApplied, WorthTracedTopologyEditCommit,
};
pub use crate::interpretation::{
    build_topology_read_artifact, certify_topology_view, interpret_topology_view,
    InterpretationReport, InterpretedTopologyView, WorthBoundaryInterpretationSummary,
    WorthRadialInterpretationSummary, WorthShellInterpretation, WorthTopologyInterpretationSet,
    WorthTopologyInterpreter, WorthWireInterpretation,
};
pub use crate::materialization::{
    MaterializationBreadthReport, MaterializationFallbackClass, MaterializationReport,
    MaterializedTopologyView, WorthTopologyMaterializationError, WorthTopologyMaterializer,
};
pub use crate::parity::{
    build_derived_equivalence_contract, compare_derived_equivalence_contracts,
    digest_derived_validation_report, digest_interpreted_topology_view,
    digest_materialized_topology_view, WorthDerivedEquivalenceContractReport,
    WorthDerivedParityComparisonReport,
};
pub use crate::reader::{
    WorthTopologyReadError, WorthTopologyReader, WorthTracedCertifiedTopologyInterpretation,
    WorthTracedDerivedEquivalenceContract, WorthTracedDerivedReadDiagnostics,
    WorthTracedMaterializedTopologyView, WorthTracedTopologyReadArtifact,
};
pub use crate::runtime_invariants::{
    build_worth_milestone_one_runtime, configure_worth_milestone_one_runtime_builder,
    worth_milestone_one_runtime_builder, worth_milestone_one_runtime_invariants,
    WorthMilestoneOneRuntimeSetupError,
};
pub use crate::validators::{
    topology_validation_report, validate_interpreted_topology, validate_materialized_topology,
    validate_named_topology_truth, validate_topology_view, DerivedTopologyValidationReport,
    WorthTopologyValidationError, WorthTopologyValidationInputClass, WorthTopologyValidationPhase,
    WorthTopologyValidationReport, WorthTopologyValidationRow, WorthTopologyValidator,
};
