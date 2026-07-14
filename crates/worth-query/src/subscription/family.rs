#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionFamily {
    DetailExact,
    CollectionMembership,
    BoundedMaterialization,
    GroupedCollectionMembership,
    InspectorDetailExact,
}

impl QuerySubscriptionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailExact => "detail_exact",
            Self::CollectionMembership => "collection_membership",
            Self::BoundedMaterialization => "bounded_materialization",
            Self::GroupedCollectionMembership => "grouped_collection_membership",
            Self::InspectorDetailExact => "inspector_detail_exact",
        }
    }
}
