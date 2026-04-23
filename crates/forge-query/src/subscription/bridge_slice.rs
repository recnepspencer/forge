use super::slice::{QuerySubscriptionSliceIntent, QuerySubscriptionSliceKind};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BridgeSubscriptionSliceKind {
    ProjectedField,
    Membership,
    Ordering,
    Grouping,
    RelationScope,
    ViewMetadata,
}

impl BridgeSubscriptionSliceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectedField => "projected_field",
            Self::Membership => "membership",
            Self::Ordering => "ordering",
            Self::Grouping => "grouping",
            Self::RelationScope => "relation_scope",
            Self::ViewMetadata => "view_metadata",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryToBridgeSliceMap {
    bridge_slices: Vec<BridgeSubscriptionSliceKind>,
}

impl QueryToBridgeSliceMap {
    pub(super) fn from_slice_intent(slice_intent: &QuerySubscriptionSliceIntent) -> Self {
        let bridge_slices = slice_intent
            .parts()
            .iter()
            .map(|part| match part.kind() {
                QuerySubscriptionSliceKind::AuthorizedProjection => {
                    BridgeSubscriptionSliceKind::ProjectedField
                }
                QuerySubscriptionSliceKind::Membership => BridgeSubscriptionSliceKind::Membership,
                QuerySubscriptionSliceKind::Ordering => BridgeSubscriptionSliceKind::Ordering,
                QuerySubscriptionSliceKind::Grouping => BridgeSubscriptionSliceKind::Grouping,
                QuerySubscriptionSliceKind::RelationScope => {
                    BridgeSubscriptionSliceKind::RelationScope
                }
                QuerySubscriptionSliceKind::ViewShapeMetadata => {
                    BridgeSubscriptionSliceKind::ViewMetadata
                }
            })
            .collect();
        Self { bridge_slices }
    }

    pub fn bridge_slices(&self) -> &[BridgeSubscriptionSliceKind] {
        &self.bridge_slices
    }
}
