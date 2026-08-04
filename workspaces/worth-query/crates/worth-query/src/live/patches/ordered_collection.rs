use super::super::promotion::{LiveQueryFamily, LiveQueryPlan};
use super::super::refresh::{PatchWidthAssessment, PatchWidthResolution, RefreshFallback};
use super::super::relevance::{
    BridgeChangeSummary, ChangeRelevance, MembershipTransition, RelevantChangeClass,
};
use super::detail::{
    OrderingFieldDelta, ProjectionFieldDelta, SuppressionDecision, SuppressionReason,
};
use super::envelope::LivePatchDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectionMembershipChange {
    EnteredCollection,
    LeftCollection,
}

impl CollectionMembershipChange {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EnteredCollection => "entered_collection",
            Self::LeftCollection => "left_collection",
        }
    }

    pub(in crate::live) fn try_from_transition(transition: &MembershipTransition) -> Option<Self> {
        match (transition.was_member(), transition.is_member()) {
            (false, true) => Some(Self::EnteredCollection),
            (true, false) => Some(Self::LeftCollection),
            (false, false) | (true, true) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionOrderingChange {
    pub(in crate::live) ordering_field_deltas: Vec<OrderingFieldDelta>,
}

impl CollectionOrderingChange {
    pub fn ordering_field_deltas(&self) -> &[OrderingFieldDelta] {
        &self.ordering_field_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrderedCollectionPatchKind {
    Membership(CollectionMembershipChange),
    Reordered(CollectionOrderingChange),
    RowUpdated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderedCollectionPatch {
    pub(in crate::live) digest: LivePatchDigest,
    pub(in crate::live) kind: OrderedCollectionPatchKind,
    pub(in crate::live) projected_field_deltas: Vec<ProjectionFieldDelta>,
}

impl OrderedCollectionPatch {
    pub fn digest(&self) -> &LivePatchDigest {
        &self.digest
    }

    pub fn kind(&self) -> &OrderedCollectionPatchKind {
        &self.kind
    }

    pub fn projected_field_deltas(&self) -> &[ProjectionFieldDelta] {
        &self.projected_field_deltas
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrderedCollectionLiveOutcome {
    Patch(OrderedCollectionPatch),
    Suppressed(SuppressionReason),
    Refresh(RefreshFallback),
}

impl OrderedCollectionLiveOutcome {
    pub fn suppression_decision(&self) -> SuppressionDecision {
        match self {
            Self::Patch(_) => SuppressionDecision::Deliver,
            Self::Refresh(_) => SuppressionDecision::Deliver,
            Self::Suppressed(reason) => SuppressionDecision::Suppress(reason.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveCollectionPatchError {
    UnsupportedFamily,
    UnsupportedRelevantClass(RelevantChangeClass),
    MissingMembershipTransition,
    NoMembershipDelta,
    MissingOrderingDelta,
    MissingProjectedDelta,
    WidthBudgetExceeded { limit: usize, actual: usize },
    CoalescingRequired { limit: usize, actual: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OrderedCollectionPatchBasis {
    digest_parts: Vec<String>,
    kind: OrderedCollectionPatchKind,
    projected_field_deltas: Vec<ProjectionFieldDelta>,
}

enum OrderedCollectionChangeClassification {
    Suppressed(SuppressionReason),
    PatchBasis(OrderedCollectionPatchBasis),
}

impl LiveQueryPlan {
    pub fn ordered_collection_live_outcome(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<OrderedCollectionLiveOutcome, LiveCollectionPatchError> {
        match self.classify_ordered_collection_change(change)? {
            OrderedCollectionChangeClassification::Suppressed(reason) => {
                Ok(OrderedCollectionLiveOutcome::Suppressed(reason))
            }
            OrderedCollectionChangeClassification::PatchBasis(basis) => {
                let patch = self.construct_ordered_collection_patch(basis);
                self.assemble_ordered_collection_outcome(patch)
            }
        }
    }

    fn classify_ordered_collection_change(
        &self,
        change: &BridgeChangeSummary,
    ) -> Result<OrderedCollectionChangeClassification, LiveCollectionPatchError> {
        if self.descriptor.family() != &LiveQueryFamily::OrderedCollection {
            return Err(LiveCollectionPatchError::UnsupportedFamily);
        }

        match self.classify_change(change) {
            ChangeRelevance::Irrelevant(reason) => {
                Ok(OrderedCollectionChangeClassification::Suppressed(
                    SuppressionReason::IrrelevantChange(reason),
                ))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionMembershipChange) => {
                let transition = change
                    .membership_transition()
                    .ok_or(LiveCollectionPatchError::MissingMembershipTransition)?;
                let membership_change = CollectionMembershipChange::try_from_transition(transition)
                    .ok_or(LiveCollectionPatchError::NoMembershipDelta)?;
                Ok(OrderedCollectionChangeClassification::PatchBasis(
                    OrderedCollectionPatchBasis {
                        digest_parts: vec![
                            "kind:collection_membership".to_string(),
                            format!("membership:{}", membership_change.as_str()),
                        ],
                        kind: OrderedCollectionPatchKind::Membership(membership_change),
                        projected_field_deltas: self.projected_field_deltas(change),
                    },
                ))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionOrderingChange) => {
                let ordering_field_deltas = self.ordering_field_deltas(change);
                if ordering_field_deltas.is_empty() {
                    return Err(LiveCollectionPatchError::MissingOrderingDelta);
                }
                Ok(OrderedCollectionChangeClassification::PatchBasis(
                    OrderedCollectionPatchBasis {
                        digest_parts: vec![
                            "kind:collection_reordered".to_string(),
                            format!("ordering_fields:{}", ordering_field_deltas.len()),
                        ],
                        kind: OrderedCollectionPatchKind::Reordered(CollectionOrderingChange {
                            ordering_field_deltas,
                        }),
                        projected_field_deltas: self.projected_field_deltas(change),
                    },
                ))
            }
            ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange) => {
                let projected_field_deltas = self.projected_field_deltas(change);
                if projected_field_deltas.is_empty() {
                    return Err(LiveCollectionPatchError::MissingProjectedDelta);
                }
                Ok(OrderedCollectionChangeClassification::PatchBasis(
                    OrderedCollectionPatchBasis {
                        digest_parts: vec![
                            "kind:collection_row_update".to_string(),
                            format!("projected_fields:{}", projected_field_deltas.len()),
                        ],
                        kind: OrderedCollectionPatchKind::RowUpdated,
                        projected_field_deltas,
                    },
                ))
            }
            ChangeRelevance::Relevant(other) => {
                Err(LiveCollectionPatchError::UnsupportedRelevantClass(other))
            }
        }
    }

    fn construct_ordered_collection_patch(
        &self,
        basis: OrderedCollectionPatchBasis,
    ) -> OrderedCollectionPatch {
        OrderedCollectionPatch {
            digest: self.patch_digest(&basis.digest_parts),
            kind: basis.kind,
            projected_field_deltas: basis.projected_field_deltas,
        }
    }

    fn assemble_ordered_collection_outcome(
        &self,
        patch: OrderedCollectionPatch,
    ) -> Result<OrderedCollectionLiveOutcome, LiveCollectionPatchError> {
        let width = self.assess_ordered_collection_width(&patch);
        match width.resolution() {
            PatchWidthResolution::Deliver => Ok(OrderedCollectionLiveOutcome::Patch(patch)),
            PatchWidthResolution::Coalesce => Err(LiveCollectionPatchError::CoalescingRequired {
                limit: width.budget_limit(),
                actual: width.measured_width(),
            }),
            PatchWidthResolution::Refresh(fallback) => {
                Ok(OrderedCollectionLiveOutcome::Refresh(fallback.clone()))
            }
            PatchWidthResolution::Reject => Err(LiveCollectionPatchError::WidthBudgetExceeded {
                limit: width.budget_limit(),
                actual: width.measured_width(),
            }),
        }
    }

    fn assess_ordered_collection_width(
        &self,
        patch: &OrderedCollectionPatch,
    ) -> PatchWidthAssessment {
        self.evaluate_delivery_width(self.measure_ordered_collection_width(patch))
    }

    pub(in crate::live) fn projected_field_deltas(
        &self,
        change: &BridgeChangeSummary,
    ) -> Vec<ProjectionFieldDelta> {
        change
            .field_deltas()
            .iter()
            .filter(|delta| {
                self.descriptor
                    .relevance_contract()
                    .projected_fields()
                    .iter()
                    .any(|field| field.matches(delta))
            })
            .map(|delta| ProjectionFieldDelta {
                field: delta.field_key().clone(),
                old_value: delta.old_value().map(ToOwned::to_owned),
                new_value: delta.new_value().map(ToOwned::to_owned),
            })
            .collect()
    }

    pub(in crate::live) fn ordering_field_deltas(
        &self,
        change: &BridgeChangeSummary,
    ) -> Vec<OrderingFieldDelta> {
        change
            .field_deltas()
            .iter()
            .filter(|delta| {
                self.descriptor
                    .relevance_contract()
                    .ordering_fields()
                    .iter()
                    .any(|field| field.matches(delta))
            })
            .map(|delta| OrderingFieldDelta {
                field: delta.field_key().clone(),
                old_value: delta.old_value().map(ToOwned::to_owned),
                new_value: delta.new_value().map(ToOwned::to_owned),
            })
            .collect()
    }

    pub(in crate::live) fn measure_ordered_collection_width(
        &self,
        patch: &OrderedCollectionPatch,
    ) -> usize {
        patch.projected_field_deltas().len() + 1
    }
}
