use super::super::promotion::{LiveQueryFamily, LiveQueryPlan};
use super::super::refresh::{PatchWidthAssessment, PatchWidthResolution, RefreshFallback};
use super::super::relevance::{
    BridgeChangeSummary, ChangeRelevance, MaterializationScopeTransition, RelevantChangeClass,
};
use super::detail::{ProjectionFieldDelta, SuppressionDecision, SuppressionReason};
use super::envelope::LivePatchDigest;
use super::ordered_collection::{CollectionMembershipChange, CollectionOrderingChange};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaterializationScopeChange {
    EnteredScope,
    LeftScope,
}

impl MaterializationScopeChange {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EnteredScope => "entered_scope",
            Self::LeftScope => "left_scope",
        }
    }

    pub(in crate::live) fn try_from_transition(
        transition: &MaterializationScopeTransition,
    ) -> Option<Self> {
        match (transition.was_in_scope(), transition.is_in_scope()) {
            (false, true) => Some(Self::EnteredScope),
            (true, false) => Some(Self::LeftScope),
            (false, false) | (true, true) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedMaterializationPatchKind {
    Scope(MaterializationScopeChange),
    Membership(CollectionMembershipChange),
    Reordered(CollectionOrderingChange),
    RowUpdated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedMaterializationPatch {
    pub(in crate::live) digest: LivePatchDigest,
    pub(in crate::live) kind: BoundedMaterializationPatchKind,
    pub(in crate::live) projected_field_deltas: Vec<ProjectionFieldDelta>,
    pub(in crate::live) relation_deltas: Vec<String>,
}

impl BoundedMaterializationPatch {
    pub fn digest(&self) -> &LivePatchDigest {
        &self.digest
    }

    pub fn kind(&self) -> &BoundedMaterializationPatchKind {
        &self.kind
    }

    pub fn projected_field_deltas(&self) -> &[ProjectionFieldDelta] {
        &self.projected_field_deltas
    }

    pub fn relation_deltas(&self) -> &[String] {
        &self.relation_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedMaterializationLiveOutcome {
    Patch(BoundedMaterializationPatch),
    Suppressed(SuppressionReason),
    Refresh(RefreshFallback),
}

impl BoundedMaterializationLiveOutcome {
    pub fn suppression_decision(&self) -> SuppressionDecision {
        match self {
            Self::Patch(_) => SuppressionDecision::Deliver,
            Self::Refresh(_) => SuppressionDecision::Deliver,
            Self::Suppressed(reason) => SuppressionDecision::Suppress(reason.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveBoundedMaterializationPatchError {
    UnsupportedFamily,
    MissingMaterializationScopeTransition,
    NoMaterializationScopeDelta,
    MissingMembershipTransition,
    NoMembershipDelta,
    MissingOrderingDelta,
    MissingProjectedDelta,
    WidthBudgetExceeded { limit: usize, actual: usize },
    CoalescingRequired { limit: usize, actual: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BoundedMaterializationPatchBasis {
    digest_parts: Vec<String>,
    kind: BoundedMaterializationPatchKind,
    projected_field_deltas: Vec<ProjectionFieldDelta>,
    relation_deltas: Vec<String>,
}

enum BoundedMaterializationChangeClassification {
    Suppressed(SuppressionReason),
    PatchBasis(BoundedMaterializationPatchBasis),
}

impl LiveQueryPlan {
    pub fn bounded_materialization_live_outcome(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<BoundedMaterializationLiveOutcome, LiveBoundedMaterializationPatchError> {
        match self.classify_bounded_materialization_change(change)? {
            BoundedMaterializationChangeClassification::Suppressed(reason) => {
                Ok(BoundedMaterializationLiveOutcome::Suppressed(reason))
            }
            BoundedMaterializationChangeClassification::PatchBasis(basis) => {
                let patch = self.construct_bounded_materialization_patch(basis);
                self.assemble_bounded_materialization_outcome(patch)
            }
        }
    }

    fn classify_bounded_materialization_change(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<BoundedMaterializationChangeClassification, LiveBoundedMaterializationPatchError>
    {
        if self.descriptor.family() != &LiveQueryFamily::BoundedMaterialization {
            return Err(LiveBoundedMaterializationPatchError::UnsupportedFamily);
        }

        match self.classify_change(change) {
            ChangeRelevance::Irrelevant(reason) => {
                Ok(BoundedMaterializationChangeClassification::Suppressed(
                    SuppressionReason::IrrelevantChange(reason),
                ))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::BoundedMaterializationScopeChange) => {
                let transition = change.materialization_scope_transition().ok_or(
                    LiveBoundedMaterializationPatchError::MissingMaterializationScopeTransition,
                )?;
                let scope_change = MaterializationScopeChange::try_from_transition(transition)
                    .ok_or(LiveBoundedMaterializationPatchError::NoMaterializationScopeDelta)?;
                let relation_deltas = self.relation_deltas(change);
                Ok(BoundedMaterializationChangeClassification::PatchBasis(
                    BoundedMaterializationPatchBasis {
                        digest_parts: vec![
                            "kind:materialization_scope".to_string(),
                            format!("scope:{}", scope_change.as_str()),
                            format!("relations:{}", relation_deltas.len()),
                        ],
                        kind: BoundedMaterializationPatchKind::Scope(scope_change),
                        projected_field_deltas: self.projected_field_deltas(change),
                        relation_deltas,
                    },
                ))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionMembershipChange) => {
                let transition = change
                    .membership_transition()
                    .ok_or(LiveBoundedMaterializationPatchError::MissingMembershipTransition)?;
                let membership_change = CollectionMembershipChange::try_from_transition(transition)
                    .ok_or(LiveBoundedMaterializationPatchError::NoMembershipDelta)?;
                Ok(BoundedMaterializationChangeClassification::PatchBasis(
                    BoundedMaterializationPatchBasis {
                        digest_parts: vec![
                            "kind:bounded_collection_membership".to_string(),
                            format!("membership:{}", membership_change.as_str()),
                        ],
                        kind: BoundedMaterializationPatchKind::Membership(membership_change),
                        projected_field_deltas: self.projected_field_deltas(change),
                        relation_deltas: self.relation_deltas(change),
                    },
                ))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionOrderingChange) => {
                let ordering_field_deltas = self.ordering_field_deltas(change);
                if ordering_field_deltas.is_empty() {
                    return Err(LiveBoundedMaterializationPatchError::MissingOrderingDelta);
                }
                Ok(BoundedMaterializationChangeClassification::PatchBasis(
                    BoundedMaterializationPatchBasis {
                        digest_parts: vec![
                            "kind:bounded_collection_reordered".to_string(),
                            format!("ordering_fields:{}", ordering_field_deltas.len()),
                        ],
                        kind: BoundedMaterializationPatchKind::Reordered(
                            CollectionOrderingChange {
                                ordering_field_deltas,
                            },
                        ),
                        projected_field_deltas: self.projected_field_deltas(change),
                        relation_deltas: self.relation_deltas(change),
                    },
                ))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange) => {
                let projected_field_deltas = self.projected_field_deltas(change);
                if projected_field_deltas.is_empty() {
                    return Err(LiveBoundedMaterializationPatchError::MissingProjectedDelta);
                }
                Ok(BoundedMaterializationChangeClassification::PatchBasis(
                    BoundedMaterializationPatchBasis {
                        digest_parts: vec![
                            "kind:bounded_row_update".to_string(),
                            format!("projected_fields:{}", projected_field_deltas.len()),
                        ],
                        kind: BoundedMaterializationPatchKind::RowUpdated,
                        projected_field_deltas,
                        relation_deltas: self.relation_deltas(change),
                    },
                ))
            }
        }
    }

    fn construct_bounded_materialization_patch(
        &self,
        basis: BoundedMaterializationPatchBasis,
    ) -> BoundedMaterializationPatch {
        BoundedMaterializationPatch {
            digest: self.patch_digest(&basis.digest_parts),
            kind: basis.kind,
            projected_field_deltas: basis.projected_field_deltas,
            relation_deltas: basis.relation_deltas,
        }
    }

    fn assemble_bounded_materialization_outcome(
        &self,
        patch: BoundedMaterializationPatch,
    ) -> Result<BoundedMaterializationLiveOutcome, LiveBoundedMaterializationPatchError> {
        let width = self.assess_bounded_materialization_width(&patch);
        match width.resolution() {
            PatchWidthResolution::Deliver => Ok(BoundedMaterializationLiveOutcome::Patch(patch)),
            PatchWidthResolution::Refresh(fallback) => {
                Ok(BoundedMaterializationLiveOutcome::Refresh(fallback.clone()))
            }
            PatchWidthResolution::Coalesce => {
                Err(LiveBoundedMaterializationPatchError::CoalescingRequired {
                    limit: width.budget_limit(),
                    actual: width.measured_width(),
                })
            }
            PatchWidthResolution::Reject => {
                Err(LiveBoundedMaterializationPatchError::WidthBudgetExceeded {
                    limit: width.budget_limit(),
                    actual: width.measured_width(),
                })
            }
        }
    }

    fn assess_bounded_materialization_width(
        &self,
        patch: &BoundedMaterializationPatch,
    ) -> PatchWidthAssessment {
        self.evaluate_delivery_width(self.measure_bounded_materialization_width(patch))
    }

    pub(in crate::live) fn relation_deltas(&self, change: &BridgeChangeSummary) -> Vec<String> {
        change
            .relation_deltas()
            .iter()
            .map(|delta| delta.relation().to_string())
            .collect()
    }

    pub(in crate::live) fn measure_bounded_materialization_width(
        &self,
        patch: &BoundedMaterializationPatch,
    ) -> usize {
        patch.projected_field_deltas().len() + patch.relation_deltas().len() + 1
    }
}
