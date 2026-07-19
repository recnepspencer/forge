use super::family::QuerySubscriptionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionLiveGraphAccessPosture {
    IncrementalMaintenancePlanned,
    SnapshotRefreshSupportRequired,
}

impl QuerySubscriptionLiveGraphAccessPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IncrementalMaintenancePlanned => "incremental_maintenance_planned",
            Self::SnapshotRefreshSupportRequired => "snapshot_refresh_support_required",
        }
    }
}

pub(super) fn live_graph_access_posture_for_family(
    family: &QuerySubscriptionFamily,
) -> QuerySubscriptionLiveGraphAccessPosture {
    match family {
        QuerySubscriptionFamily::DetailExact
        | QuerySubscriptionFamily::CollectionMembership
        | QuerySubscriptionFamily::InspectorDetailExact => {
            QuerySubscriptionLiveGraphAccessPosture::IncrementalMaintenancePlanned
        }
        QuerySubscriptionFamily::GroupedCollectionMembership
        | QuerySubscriptionFamily::BoundedMaterialization => {
            QuerySubscriptionLiveGraphAccessPosture::SnapshotRefreshSupportRequired
        }
    }
}
