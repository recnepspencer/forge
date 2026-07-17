//! Graph truth lane: admission → identity → topology → neighborhood → inspection → mutation → closeout.

mod admission;
mod allocation_neighborhood;
#[cfg(test)]
pub(crate) use allocation_neighborhood::tests::{
    allocation_constraint_bound_reconciliation_test_support,
    allocation_constraint_equal_share_test_support, allocation_constraint_projection_tests,
    allocation_constraint_sibling_support_test_support,
};
pub(crate) use allocation_neighborhood::UiAllocationNeighborhoodMintAuthority;
pub(crate) use allocation_neighborhood::UiGraphConstraintMintAuthority;
pub(crate) use allocation_neighborhood::{
    UiAdmittedAllocationConstraintBasis, UiAllocationConstraintProvenance,
    UiGraphScrollPlanningAuthority,
};
#[cfg(test)]
pub(crate) mod allocation_neighborhood_test_support;
mod closeout;
mod identity;
mod indexes;
mod inspection;
#[cfg(test)]
mod measurement_neighborhood_hint;
#[cfg(test)]
mod measurement_neighborhood_hint_tests;
mod mounted_receipt;
mod mutation;
mod participation;
mod snapshot;
mod topology;

// --- admission (declaration → graph instantiation) ---
pub(crate) use admission::admit_graph_handoffs;
pub use admission::{
    UiGraphCoreIndexContributionSeed, UiGraphInstantiationDenial, UiGraphInstantiationLocalDenial,
    UiGraphInstantiationLocalDenialKind, UiGraphInstantiationPlan, UiGraphNodeInstantiationEntry,
    UiGraphParticipationSeed, UiGraphTopologyLocalDenial, UiGraphTopologySeed,
    UiRuntimeInstanceBasisAdmission,
};

// --- allocation neighborhood (graph → planning handoff; admission sealed pub(crate)) ---
pub(crate) use allocation_neighborhood::select_replan_neighborhoods;
pub(crate) use allocation_neighborhood::UiGraphNeighborhoodActivationTransition;
pub(crate) use allocation_neighborhood::UiGraphReplanConsequences;
pub(crate) use allocation_neighborhood::UiGraphReplanTransactionBasis;
pub use allocation_neighborhood::{
    UiAdmittedAllocationCatalogBasisSet, UiAdmittedAllocationInvalidationTargetSet,
    UiAdmittedReplanNeighborhood, UiAdmittedReplanNeighborhoodSet,
    UiAllocationCatalogBasisAdmissionDenial, UiAllocationNeighborhoodDenial,
    UiReplanLocalityDenial, UiReplanLocalityProof, UiReplanNeighborhoodSelectionCounters,
    UiReplanOverlapDisposition, UiReplanRootPosture, UiReplanWidenReason,
};
pub(crate) use allocation_neighborhood::{
    UiAdmittedAllocationInvalidationTarget, UiAdmittedAllocationPlanReference,
    UiGraphReplanAdmission, UiGraphReplanAuthority, UiGraphReplanTargetDisposition,
    UiReplanGenerationKey,
};

// --- closeout ---
pub use closeout::{
    UiGraphAuthority, UiGraphClosedSemanticLane, UiGraphCloseoutGuarantee, UiGraphCloseoutNonGoal,
    UiGraphCloseoutReport, UiGraphInspectionStopPoint, UiGraphInspectionSupportReport,
    UiGraphMountedReceiptAuthorityRecord, UiGraphNodeRecord, UiGraphTopologyRecord,
};

// --- identity ---
pub use identity::{
    UiGraphGeneration, UiGraphGenerationRelation, UiGraphNodeIdentity, UiGraphSessionIdentityError,
    UiGraphSessionLabel, UiGraphSnapshotComparable, UiGraphWorldDifferenceKind,
    UiGraphWorldProfile, UiPreviewSessionIdentity, UiRepeatedInstanceBasis,
    UiRepeatedInstanceBasisDenial, UiRepeatedInstanceBasisKind, UiRuntimeDataInstanceKeyKind,
    UiRuntimeDataInstanceKeyToken,
};

// --- indexes / lookup ---
pub use indexes::{
    UiGraphAspectConsumer, UiGraphAspectConsumerKind, UiGraphAspectPublisher,
    UiGraphAspectPublisherKind, UiGraphCoreIndexes, UiGraphLookup, UiGraphLookupCostClass,
    UiGraphLookupFamily, UiGraphLookupReceipt, UiGraphLookupSurface, UiGraphMosaicMembershipIndex,
    UiGraphMountedReceiptIndex, UiGraphPageMembershipIndex, UiGraphPageParticipationIndex,
    UiGraphPageParticipationMember, UiGraphParentChildIndex, UiGraphRegionMembershipIndex,
    UiGraphSlotOccupancyIndex,
};

// --- inspection ---
pub(crate) use inspection::UiGraphEvidenceRecord;
pub use inspection::{
    project_aspect_evidence_ref, project_aspect_evidence_refs, UiAspectEvidenceLane,
    UiAspectEvidenceRefProjection, UiAspectEvidenceSubjectKind, UiGraphEvidenceRef,
    UiGraphEvidenceRefKind, UiGraphInspection, UiGraphInspectionSupport, UiGraphInspectionTarget,
    UiGraphInspectionTargetKind,
};
pub(crate) use inspection::{
    UiGraphAspectEvidenceIndexes, UiGraphNodeEvidenceIndex, WorthUiAspectInspectionBoundary,
    WorthUiGraphInspectionBoundary,
};

#[cfg(test)]
pub(crate) use measurement_neighborhood_hint::UiGraphMeasurementNeighborhoodHint;

// --- mounted receipt ---
pub(crate) use mounted_receipt::materialize_graph_mounted_receipts;
pub use mounted_receipt::{
    UiGraphMountedPostureRelationship, UiGraphMountedReceiptAuthoritySeed,
    UiGraphMountedReceiptAuthoritySeedStore, UiGraphMountedReceiptMutation,
    UiGraphMountedReceiptMutationKind, UiGraphMountedReceiptReservation, UiGraphMountedReceiptSlot,
    UiGraphMountedReceiptTransition, UiMountedReceiptIdentity,
};

// --- mutation ---
#[cfg(any(test, feature = "certification-support"))]
pub(crate) use mutation::UiGraphMutationStage;
pub use mutation::{UiGraphMutationCommitDenial, UiGraphMutationCommitResult};

// --- participation ---
pub(crate) use participation::materialize_graph_participation_posture;
pub use participation::{
    UiGraphAxisParticipation, UiGraphPageParticipationMutation,
    UiGraphPageParticipationMutationKind, UiGraphParticipationAxis,
    UiGraphParticipationEvidenceHandle, UiGraphParticipationMutation, UiGraphParticipationPosture,
    UiGraphParticipationReasonCode, UiGraphParticipationReasonSource, UiGraphParticipationStatus,
};

// --- snapshot ---
pub use snapshot::{
    UiGraphAttachmentPosture, UiGraphDeclarationCorrespondence, UiGraphNode, UiGraphSnapshot,
};

// --- topology ---
pub(crate) use topology::materialize_graph_topology;
pub use topology::{
    UiGraphContainmentClaim, UiGraphMembershipFacts, UiGraphMosaicMembership, UiGraphNodeTopology,
    UiGraphPageMembership, UiGraphParentResolutionClaim, UiGraphRegionMembership,
    UiGraphSlotTopology, UiGraphTopology,
};
