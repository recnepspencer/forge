use super::{
    UiInspectionAiHarnessLane, UiInspectionClosedSemanticLane, UiInspectionCloseoutGuarantee,
    UiInspectionCloseoutNonGoal, UiInspectionCloseoutReport, UiInspectionCostLane,
    UiInspectionDerivedIndexLane, UiInspectionRefLifecycleLane, UiInspectionSliceLane,
};
use crate::{
    UiEvidenceBudget, UiEvidenceFamily, UiEvidenceMaterializationPosture,
    UiEvidenceRetentionPosture, UiInspectionQueryForeignEvidenceKind, UiInspectionRelevanceOutcome,
    UiInspectionScope, UiInspectionTargetClass,
};

const MILESTONE35_EVIDENCE_FAMILIES: &[UiEvidenceFamily] = &[
    UiEvidenceFamily::Declaration,
    UiEvidenceFamily::Admission,
    UiEvidenceFamily::Graph,
    UiEvidenceFamily::Aspect,
    UiEvidenceFamily::Obligation,
];

const MILESTONE35_RELEVANCE_OUTCOMES: &[UiInspectionRelevanceOutcome] = &[
    UiInspectionRelevanceOutcome::Matched,
    UiInspectionRelevanceOutcome::EmptyLocal,
    UiInspectionRelevanceOutcome::UnsupportedScope {
        scope: UiInspectionScope::Graph,
    },
    UiInspectionRelevanceOutcome::ContradictoryRequest,
    UiInspectionRelevanceOutcome::BudgetExceeded {
        budget: UiEvidenceBudget::Narrow,
    },
    UiInspectionRelevanceOutcome::NotApplicableToTarget {
        target: UiInspectionTargetClass::ProductRoot,
    },
];

const MILESTONE35_REF_LIFECYCLE_LANES: &[UiInspectionRefLifecycleLane] = &[
    UiInspectionRefLifecycleLane::MaterializationPostureBoundRef,
    UiInspectionRefLifecycleLane::FollowupQueryExpansion,
    UiInspectionRefLifecycleLane::RetainedDetailExpansion,
    UiInspectionRefLifecycleLane::NotMaterializedExpansion,
    UiInspectionRefLifecycleLane::WrongGenerationExpansion,
    UiInspectionRefLifecycleLane::DiscardedTombstoneExpansion,
];

const MILESTONE35_MATERIALIZATION_POSTURES: &[UiEvidenceMaterializationPosture] = &[
    UiEvidenceMaterializationPosture::RefsOnly,
    UiEvidenceMaterializationPosture::SummaryAvailable,
    UiEvidenceMaterializationPosture::DetailAvailable,
];

const MILESTONE35_RETENTION_POSTURES: &[UiEvidenceRetentionPosture] = &[
    UiEvidenceRetentionPosture::CurrentGenerationOnly,
    UiEvidenceRetentionPosture::DiscardedWithTombstone,
];

const MILESTONE35_QUERY_CITATION_KINDS: &[UiInspectionQueryForeignEvidenceKind] = &[
    UiInspectionQueryForeignEvidenceKind::ProjectionConsumption,
    UiInspectionQueryForeignEvidenceKind::Inspection,
    UiInspectionQueryForeignEvidenceKind::CausalExplanation,
];

const MILESTONE35_DERIVED_INDEX_LANES: &[UiInspectionDerivedIndexLane] = &[
    UiInspectionDerivedIndexLane::DeclarationAuthoredEvidence,
    UiInspectionDerivedIndexLane::GraphNodeEvidence,
    UiInspectionDerivedIndexLane::GraphAspectEvidence,
];

const MILESTONE35_SLICE_LANES: &[UiInspectionSliceLane] = &[
    UiInspectionSliceLane::DeclarationIdentity,
    UiInspectionSliceLane::AuthoredSourceProvenance,
    UiInspectionSliceLane::GraphNodeIdentity,
    UiInspectionSliceLane::AspectNeighborhood,
    UiInspectionSliceLane::ObligationNeighborhood,
    UiInspectionSliceLane::FamilySummaries,
    UiInspectionSliceLane::OmissionByScope,
    UiInspectionSliceLane::OmissionByBudget,
];

const MILESTONE35_COST_LANES: &[UiInspectionCostLane] = &[
    UiInspectionCostLane::IndexedLookup,
    UiInspectionCostLane::NoBroadScan,
    UiInspectionCostLane::BudgetOmissionTracked,
    UiInspectionCostLane::MaterializationTracked,
    UiInspectionCostLane::TraversalDenialsExplicit,
];

const MILESTONE35_AI_HARNESS_LANES: &[UiInspectionAiHarnessLane] = &[
    UiInspectionAiHarnessLane::Inspect,
    UiInspectionAiHarnessLane::ExpandEvidenceRef,
    UiInspectionAiHarnessLane::CiteForeignEvidence,
    UiInspectionAiHarnessLane::SupportReport,
    UiInspectionAiHarnessLane::ClosureReport,
];

const MILESTONE35_CLOSED_SEMANTIC_LANES: &[UiInspectionClosedSemanticLane] = &[
    UiInspectionClosedSemanticLane::EvidenceFamilies,
    UiInspectionClosedSemanticLane::RelevanceNarrowing,
    UiInspectionClosedSemanticLane::StableEvidenceRefs,
    UiInspectionClosedSemanticLane::RefExpansionLifecycle,
    UiInspectionClosedSemanticLane::RetentionPosture,
    UiInspectionClosedSemanticLane::QueryForeignEvidenceCitation,
    UiInspectionClosedSemanticLane::DerivedIndexLookup,
    UiInspectionClosedSemanticLane::SliceProjection,
    UiInspectionClosedSemanticLane::CostPosture,
    UiInspectionClosedSemanticLane::AiHarnessParity,
    UiInspectionClosedSemanticLane::SupportAndClosureReports,
];

const MILESTONE35_GUARANTEES: &[UiInspectionCloseoutGuarantee] = &[
    UiInspectionCloseoutGuarantee::CallerBypassDiesAtCompileAndFacadeBoundary,
    UiInspectionCloseoutGuarantee::EquivalentQueriesConvergeUnderStableAuthorityGeneration,
    UiInspectionCloseoutGuarantee::OrdinaryInspectionStaysNarrowAndIndexBacked,
    UiInspectionCloseoutGuarantee::QueryOwnedTruthRemainsForeignOwned,
    UiInspectionCloseoutGuarantee::FutureFamiliesExtendOneSubstrate,
];

const MILESTONE35_NON_GOALS: &[UiInspectionCloseoutNonGoal] = &[
    UiInspectionCloseoutNonGoal::MeasurementEvidence,
    UiInspectionCloseoutNonGoal::MountedReceiptEvidence,
    UiInspectionCloseoutNonGoal::VisualSnapshotEvidence,
    UiInspectionCloseoutNonGoal::ReplayEvidence,
    UiInspectionCloseoutNonGoal::RendererLocalExplanation,
    UiInspectionCloseoutNonGoal::HostLocalExplanation,
    UiInspectionCloseoutNonGoal::LogLocalExplanation,
];

pub(crate) const MILESTONE35_CLOSEOUT_PROFILE: UiInspectionCloseoutReport =
    UiInspectionCloseoutReport::new(
        MILESTONE35_EVIDENCE_FAMILIES,
        MILESTONE35_RELEVANCE_OUTCOMES,
        MILESTONE35_REF_LIFECYCLE_LANES,
        MILESTONE35_MATERIALIZATION_POSTURES,
        MILESTONE35_RETENTION_POSTURES,
        MILESTONE35_QUERY_CITATION_KINDS,
        MILESTONE35_DERIVED_INDEX_LANES,
        MILESTONE35_SLICE_LANES,
        MILESTONE35_COST_LANES,
        MILESTONE35_AI_HARNESS_LANES,
        MILESTONE35_CLOSED_SEMANTIC_LANES,
        MILESTONE35_GUARANTEES,
        MILESTONE35_NON_GOALS,
    );
