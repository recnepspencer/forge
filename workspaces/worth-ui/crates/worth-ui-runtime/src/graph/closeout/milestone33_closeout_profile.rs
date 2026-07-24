use crate::graph::{
    UiGraphClosedSemanticLane, UiGraphCloseoutGuarantee, UiGraphCloseoutNonGoal,
    UiGraphCloseoutReport, UiGraphEvidenceRefKind, UiGraphInspectionStopPoint,
    UiGraphInspectionSupportReport, UiGraphInspectionTargetKind,
};

const MILESTONE33_CLOSED_SEMANTIC_LANES: &[UiGraphClosedSemanticLane] = &[
    UiGraphClosedSemanticLane::NodeIdentity,
    UiGraphClosedSemanticLane::DeclarationCorrespondence,
    UiGraphClosedSemanticLane::RepeatedInstanceBasis,
    UiGraphClosedSemanticLane::WorldProfile,
    UiGraphClosedSemanticLane::AttachmentPosture,
    UiGraphClosedSemanticLane::ParentChildTopology,
    UiGraphClosedSemanticLane::SlotOccupancyTopology,
    UiGraphClosedSemanticLane::PageRegionMosaicMembership,
    UiGraphClosedSemanticLane::ParticipationPosture,
    UiGraphClosedSemanticLane::MountEligibility,
    UiGraphClosedSemanticLane::AspectIndexes,
    UiGraphClosedSemanticLane::BoundedLookup,
    UiGraphClosedSemanticLane::InspectionSupport,
];

const MILESTONE33_GUARANTEES: &[UiGraphCloseoutGuarantee] = &[
    UiGraphCloseoutGuarantee::GraphTruthOwnedByCommittedSnapshot,
    UiGraphCloseoutGuarantee::DeclarationAuthorityLowersOnceIntoGraphCorrespondence,
    UiGraphCloseoutGuarantee::GraphAndIndexMutationCommitAsOneGenerationTransition,
    UiGraphCloseoutGuarantee::OrdinaryLookupRemainsReceiptBackedAndBounded,
    UiGraphCloseoutGuarantee::FormalInspectionCarriesThinTargetsEvidenceAndStopPoints,
    UiGraphCloseoutGuarantee::HandoffConsumesProofBearingGraphAuthorityRatherThanRawInternals,
];

const MILESTONE33_NON_GOALS: &[UiGraphCloseoutNonGoal] = &[
    UiGraphCloseoutNonGoal::QueryExecution,
    UiGraphCloseoutNonGoal::TouchedObligationSelection,
    UiGraphCloseoutNonGoal::HostObservation,
    UiGraphCloseoutNonGoal::MeasurementRuntimeTruth,
    UiGraphCloseoutNonGoal::InteractionRuntimeTruth,
    UiGraphCloseoutNonGoal::SideTopologiesOutsideGraphAuthority,
];

const MILESTONE33_INSPECTION_TARGETS: &[UiGraphInspectionTargetKind] = &[
    UiGraphInspectionTargetKind::GraphNode,
    UiGraphInspectionTargetKind::TopologyNode,
    UiGraphInspectionTargetKind::DeclarationInstances,
    UiGraphInspectionTargetKind::ParentChild,
    UiGraphInspectionTargetKind::SlotOccupancy,
    UiGraphInspectionTargetKind::PageParticipation,
    UiGraphInspectionTargetKind::PublishedAspect,
    UiGraphInspectionTargetKind::ConsumedAspect,
    UiGraphInspectionTargetKind::MountEligibility,
];

const MILESTONE33_EVIDENCE_REFS: &[UiGraphEvidenceRefKind] = &[
    UiGraphEvidenceRefKind::GraphNode,
    UiGraphEvidenceRefKind::Declaration,
    UiGraphEvidenceRefKind::MountEligibility,
    UiGraphEvidenceRefKind::Aspect,
    UiGraphEvidenceRefKind::Page,
];

const MILESTONE33_STOP_POINTS: &[UiGraphInspectionStopPoint] = &[
    UiGraphInspectionStopPoint::NodeIdentity,
    UiGraphInspectionStopPoint::DeclarationCorrespondence,
    UiGraphInspectionStopPoint::TopologyTruth,
    UiGraphInspectionStopPoint::ParticipationTruth,
    UiGraphInspectionStopPoint::AttachmentPosture,
    UiGraphInspectionStopPoint::PublishedAspectIndex,
    UiGraphInspectionStopPoint::ConsumedAspectIndex,
    UiGraphInspectionStopPoint::MountEligibility,
];

pub(crate) const MILESTONE33_CLOSEOUT_PROFILE: UiGraphCloseoutReport = UiGraphCloseoutReport::new(
    MILESTONE33_CLOSED_SEMANTIC_LANES,
    MILESTONE33_GUARANTEES,
    MILESTONE33_NON_GOALS,
    UiGraphInspectionSupportReport::new(
        MILESTONE33_INSPECTION_TARGETS,
        MILESTONE33_EVIDENCE_REFS,
        MILESTONE33_STOP_POINTS,
    ),
);
