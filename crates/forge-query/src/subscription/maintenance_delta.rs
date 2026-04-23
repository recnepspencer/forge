use crate::identity::hash_parts;

use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_posture::ActiveSubscriptionDeliveryPosture;
use super::delivery_dimensions::MaintenanceDeltaWidth;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionMaintenanceDeltaKind {
    DetailFieldDelta,
    InspectorFocusDelta,
    CollectionMembershipDelta,
    CollectionOrderDelta,
    GroupedMembershipDelta,
    BoundedMaterializationScopeDelta,
    ContinuationDelta,
    GapNoticeDelta,
}

impl QuerySubscriptionMaintenanceDeltaKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailFieldDelta => "detail_field_delta",
            Self::InspectorFocusDelta => "inspector_focus_delta",
            Self::CollectionMembershipDelta => "collection_membership_delta",
            Self::CollectionOrderDelta => "collection_order_delta",
            Self::GroupedMembershipDelta => "grouped_membership_delta",
            Self::BoundedMaterializationScopeDelta => "bounded_materialization_scope_delta",
            Self::ContinuationDelta => "continuation_delta",
            Self::GapNoticeDelta => "gap_notice_delta",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionMaintenanceDelta {
    kind: QuerySubscriptionMaintenanceDeltaKind,
    active_lane_digest: ActiveSubscriptionLaneDigest,
    affected_scope_digest: String,
    width: MaintenanceDeltaWidth,
    maintenance_delta_digest: String,
}

impl QuerySubscriptionMaintenanceDelta {
    pub fn admitted(
        kind: QuerySubscriptionMaintenanceDeltaKind,
        active_lane_digest: ActiveSubscriptionLaneDigest,
        affected_scope: impl Into<String>,
        width: MaintenanceDeltaWidth,
    ) -> Self {
        let affected_scope_digest = hash_parts(&[
            "query_subscription_affected_scope_v1".to_string(),
            format!("scope:{}", affected_scope.into()),
        ]);
        let maintenance_delta_digest = hash_parts(&[
            "query_subscription_maintenance_delta_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("lane:{}", active_lane_digest.as_str()),
            format!("scope:{}", affected_scope_digest),
            format!("width:{}", width.get()),
        ]);
        Self {
            kind,
            active_lane_digest,
            affected_scope_digest,
            width,
            maintenance_delta_digest,
        }
    }

    pub fn kind(&self) -> QuerySubscriptionMaintenanceDeltaKind {
        self.kind
    }

    pub fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn affected_scope_digest(&self) -> &str {
        &self.affected_scope_digest
    }

    pub fn width(&self) -> u64 {
        self.width.get()
    }

    pub fn maintenance_delta_digest(&self) -> &str {
        &self.maintenance_delta_digest
    }

    pub(super) fn delivery_posture(&self) -> ActiveSubscriptionDeliveryPosture {
        match self.kind {
            QuerySubscriptionMaintenanceDeltaKind::InspectorFocusDelta => {
                ActiveSubscriptionDeliveryPosture::FocusedInspectorPatch
            }
            QuerySubscriptionMaintenanceDeltaKind::GroupedMembershipDelta => {
                ActiveSubscriptionDeliveryPosture::GroupedPatch
            }
            QuerySubscriptionMaintenanceDeltaKind::BoundedMaterializationScopeDelta => {
                ActiveSubscriptionDeliveryPosture::BoundedMaterializationPatch
            }
            _ => ActiveSubscriptionDeliveryPosture::QueryShapedPatch,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryMaintenanceDeltaLoweringReport {
    maintenance_delta_digest: String,
    lowering_report_digest: String,
}

impl QueryMaintenanceDeltaLoweringReport {
    pub(super) fn new(delta: &QuerySubscriptionMaintenanceDelta) -> Self {
        let lowering_report_digest = hash_parts(&[
            "query_maintenance_delta_lowering_report_v1".to_string(),
            format!("delta:{}", delta.maintenance_delta_digest()),
            format!("kind:{}", delta.kind().as_str()),
            format!("width:{}", delta.width()),
        ]);
        Self {
            maintenance_delta_digest: delta.maintenance_delta_digest().to_string(),
            lowering_report_digest,
        }
    }

    pub fn maintenance_delta_digest(&self) -> &str {
        &self.maintenance_delta_digest
    }

    pub fn lowering_report_digest(&self) -> &str {
        &self.lowering_report_digest
    }
}
