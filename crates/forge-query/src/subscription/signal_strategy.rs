use crate::identity::hash_parts;

use super::bridge_family::BridgeSubscriptionDeclarationFamilyKind;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionSignalStrategyRequestKind {
    ExactDetailSignals,
    CollectionMembershipSignals,
}

impl QuerySubscriptionSignalStrategyRequestKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactDetailSignals => "exact_detail_signals",
            Self::CollectionMembershipSignals => "collection_membership_signals",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSignalStrategyRequest {
    request_kind: QuerySubscriptionSignalStrategyRequestKind,
    digest: String,
}

impl QuerySubscriptionSignalStrategyRequest {
    pub(super) fn for_bridge_family(family: &BridgeSubscriptionDeclarationFamilyKind) -> Self {
        let request_kind = match family {
            BridgeSubscriptionDeclarationFamilyKind::DetailExact => {
                QuerySubscriptionSignalStrategyRequestKind::ExactDetailSignals
            }
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership => {
                QuerySubscriptionSignalStrategyRequestKind::CollectionMembershipSignals
            }
        };
        let digest = hash_parts(&[
            "query_subscription_signal_strategy_request_v1".to_string(),
            request_kind.as_str().to_string(),
        ]);
        Self {
            request_kind,
            digest,
        }
    }

    pub fn request_kind(&self) -> &QuerySubscriptionSignalStrategyRequestKind {
        &self.request_kind
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
