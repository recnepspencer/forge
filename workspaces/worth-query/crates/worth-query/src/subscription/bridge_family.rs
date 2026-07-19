use super::family::QuerySubscriptionFamily;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BridgeSubscriptionDeclarationFamilyKind {
    DetailExact,
    CollectionMembership,
}

impl BridgeSubscriptionDeclarationFamilyKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailExact => "detail_exact",
            Self::CollectionMembership => "collection_membership",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryToBridgeSubscriptionFamilyMap {
    query_family: QuerySubscriptionFamily,
    bridge_family: BridgeSubscriptionDeclarationFamilyKind,
}

impl QueryToBridgeSubscriptionFamilyMap {
    pub(super) fn for_query_family(query_family: &QuerySubscriptionFamily) -> Self {
        let bridge_family = match query_family {
            QuerySubscriptionFamily::DetailExact
            | QuerySubscriptionFamily::InspectorDetailExact => {
                BridgeSubscriptionDeclarationFamilyKind::DetailExact
            }
            QuerySubscriptionFamily::CollectionMembership
            | QuerySubscriptionFamily::GroupedCollectionMembership
            | QuerySubscriptionFamily::BoundedMaterialization => {
                BridgeSubscriptionDeclarationFamilyKind::CollectionMembership
            }
        };
        Self {
            query_family: query_family.clone(),
            bridge_family,
        }
    }

    pub fn query_family(&self) -> &QuerySubscriptionFamily {
        &self.query_family
    }

    pub fn bridge_family(&self) -> &BridgeSubscriptionDeclarationFamilyKind {
        &self.bridge_family
    }
}
