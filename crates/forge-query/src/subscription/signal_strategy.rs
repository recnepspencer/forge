use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::bridge_family::BridgeSubscriptionDeclarationFamilyKind;
use super::evidence_identities::signal_strategy_request_identity;

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
    evidence_identity: ForgeQueryEvidenceIdentity,
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
        let evidence_identity = signal_strategy_request_identity(&request_kind);
        Self {
            request_kind,
            evidence_identity,
        }
    }

    pub fn request_kind(&self) -> &QuerySubscriptionSignalStrategyRequestKind {
        &self.request_kind
    }

    pub fn digest(&self) -> &str {
        self.evidence_identity.as_str()
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.evidence_identity
    }
}
