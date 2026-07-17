use super::{
    inspection_closeout_profile::MILESTONE35_CLOSEOUT_PROFILE, UiInspectionAiHarnessLane,
    UiInspectionClosedSemanticLane, UiInspectionCloseoutGuarantee, UiInspectionCloseoutNonGoal,
    UiInspectionCostLane, UiInspectionDerivedIndexLane, UiInspectionRefLifecycleLane,
    UiInspectionSliceLane,
};
use crate::{
    UiEvidenceFamily, UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture,
    UiInspectionQueryForeignEvidenceKind, UiInspectionRelevanceOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionCloseoutReport {
    evidence_families: &'static [UiEvidenceFamily],
    relevance_outcomes: &'static [UiInspectionRelevanceOutcome],
    ref_lifecycle_lanes: &'static [UiInspectionRefLifecycleLane],
    materialization_postures: &'static [UiEvidenceMaterializationPosture],
    retention_postures: &'static [UiEvidenceRetentionPosture],
    query_citation_kinds: &'static [UiInspectionQueryForeignEvidenceKind],
    derived_index_lanes: &'static [UiInspectionDerivedIndexLane],
    slice_lanes: &'static [UiInspectionSliceLane],
    cost_lanes: &'static [UiInspectionCostLane],
    ai_harness_lanes: &'static [UiInspectionAiHarnessLane],
    closed_semantic_lanes: &'static [UiInspectionClosedSemanticLane],
    guarantees: &'static [UiInspectionCloseoutGuarantee],
    non_goals: &'static [UiInspectionCloseoutNonGoal],
}

pub(crate) struct UiInspectionCloseoutProfile {
    pub(super) evidence_families: &'static [UiEvidenceFamily],
    pub(super) relevance_outcomes: &'static [UiInspectionRelevanceOutcome],
    pub(super) ref_lifecycle_lanes: &'static [UiInspectionRefLifecycleLane],
    pub(super) materialization_postures: &'static [UiEvidenceMaterializationPosture],
    pub(super) retention_postures: &'static [UiEvidenceRetentionPosture],
    pub(super) query_citation_kinds: &'static [UiInspectionQueryForeignEvidenceKind],
    pub(super) derived_index_lanes: &'static [UiInspectionDerivedIndexLane],
    pub(super) slice_lanes: &'static [UiInspectionSliceLane],
    pub(super) cost_lanes: &'static [UiInspectionCostLane],
    pub(super) ai_harness_lanes: &'static [UiInspectionAiHarnessLane],
    pub(super) closed_semantic_lanes: &'static [UiInspectionClosedSemanticLane],
    pub(super) guarantees: &'static [UiInspectionCloseoutGuarantee],
    pub(super) non_goals: &'static [UiInspectionCloseoutNonGoal],
}

impl UiInspectionCloseoutReport {
    pub(super) const fn new(profile: UiInspectionCloseoutProfile) -> Self {
        Self {
            evidence_families: profile.evidence_families,
            relevance_outcomes: profile.relevance_outcomes,
            ref_lifecycle_lanes: profile.ref_lifecycle_lanes,
            materialization_postures: profile.materialization_postures,
            retention_postures: profile.retention_postures,
            query_citation_kinds: profile.query_citation_kinds,
            derived_index_lanes: profile.derived_index_lanes,
            slice_lanes: profile.slice_lanes,
            cost_lanes: profile.cost_lanes,
            ai_harness_lanes: profile.ai_harness_lanes,
            closed_semantic_lanes: profile.closed_semantic_lanes,
            guarantees: profile.guarantees,
            non_goals: profile.non_goals,
        }
    }

    pub const fn milestone35() -> Self {
        MILESTONE35_CLOSEOUT_PROFILE
    }

    pub const fn evidence_families(self) -> &'static [UiEvidenceFamily] {
        self.evidence_families
    }

    pub const fn relevance_outcomes(self) -> &'static [UiInspectionRelevanceOutcome] {
        self.relevance_outcomes
    }

    pub const fn ref_lifecycle_lanes(self) -> &'static [UiInspectionRefLifecycleLane] {
        self.ref_lifecycle_lanes
    }

    pub const fn materialization_postures(self) -> &'static [UiEvidenceMaterializationPosture] {
        self.materialization_postures
    }

    pub const fn retention_postures(self) -> &'static [UiEvidenceRetentionPosture] {
        self.retention_postures
    }

    pub const fn query_citation_kinds(self) -> &'static [UiInspectionQueryForeignEvidenceKind] {
        self.query_citation_kinds
    }

    pub const fn derived_index_lanes(self) -> &'static [UiInspectionDerivedIndexLane] {
        self.derived_index_lanes
    }

    pub const fn slice_lanes(self) -> &'static [UiInspectionSliceLane] {
        self.slice_lanes
    }

    pub const fn cost_lanes(self) -> &'static [UiInspectionCostLane] {
        self.cost_lanes
    }

    pub const fn ai_harness_lanes(self) -> &'static [UiInspectionAiHarnessLane] {
        self.ai_harness_lanes
    }

    pub const fn closed_semantic_lanes(self) -> &'static [UiInspectionClosedSemanticLane] {
        self.closed_semantic_lanes
    }

    pub const fn guarantees(self) -> &'static [UiInspectionCloseoutGuarantee] {
        self.guarantees
    }

    pub const fn non_goals(self) -> &'static [UiInspectionCloseoutNonGoal] {
        self.non_goals
    }
}
