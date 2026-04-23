use crate::identity::hash_parts;

use super::family::QuerySubscriptionFamily;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionDeliveryIntent {
    ExactDetailReplacement,
    OrderedMembershipDelta,
    GroupedMembershipDelta,
    InspectorFocusedDetailReplacement,
    BoundedMaterializationMembershipDelta,
}

impl QuerySubscriptionDeliveryIntent {
    pub(super) fn for_family(family: &QuerySubscriptionFamily) -> Self {
        match family {
            QuerySubscriptionFamily::DetailExact => Self::ExactDetailReplacement,
            QuerySubscriptionFamily::CollectionMembership => Self::OrderedMembershipDelta,
            QuerySubscriptionFamily::GroupedCollectionMembership => Self::GroupedMembershipDelta,
            QuerySubscriptionFamily::InspectorDetailExact => {
                Self::InspectorFocusedDetailReplacement
            }
            QuerySubscriptionFamily::BoundedMaterialization => {
                Self::BoundedMaterializationMembershipDelta
            }
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactDetailReplacement => "exact_detail_replacement",
            Self::OrderedMembershipDelta => "ordered_membership_delta",
            Self::GroupedMembershipDelta => "grouped_membership_delta",
            Self::InspectorFocusedDetailReplacement => "inspector_focused_detail_replacement",
            Self::BoundedMaterializationMembershipDelta => {
                "bounded_materialization_membership_delta"
            }
        }
    }

    pub fn digest(&self) -> String {
        hash_parts(&[
            "query_subscription_delivery_intent_v1".to_string(),
            self.as_str().to_string(),
        ])
    }
}
