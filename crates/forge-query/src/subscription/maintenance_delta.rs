use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_posture::ActiveSubscriptionDeliveryPosture;
use super::delivery_dimensions::MaintenanceDeltaWidth;
use super::evidence_identities::lifecycle_maintenance_delta_identity;

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
    affected_scope: String,
    width: MaintenanceDeltaWidth,
    maintenance_delta_identity: ForgeQueryEvidenceIdentity,
    maintenance_delta_digest: String,
}

impl QuerySubscriptionMaintenanceDelta {
    pub fn admitted(
        kind: QuerySubscriptionMaintenanceDeltaKind,
        active_lane_digest: ActiveSubscriptionLaneDigest,
        affected_scope: impl Into<String>,
        width: MaintenanceDeltaWidth,
    ) -> Self {
        let affected_scope = affected_scope.into();
        let maintenance_delta_identity = lifecycle_maintenance_delta_identity(
            kind,
            active_lane_digest.evidence_identity(),
            &affected_scope,
            width.get(),
        );
        let maintenance_delta_digest = maintenance_delta_identity.as_str().to_string();
        Self {
            kind,
            active_lane_digest,
            affected_scope,
            width,
            maintenance_delta_identity,
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
        self.affected_scope.as_str()
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.maintenance_delta_identity
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
    lowering_report_identity: ForgeQueryEvidenceIdentity,
    lowering_report_digest: String,
}

impl QueryMaintenanceDeltaLoweringReport {
    pub(super) fn new(delta: &QuerySubscriptionMaintenanceDelta) -> Self {
        let lowering_report_identity = ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
            "query_maintenance_delta_lowering_report_v1",
        )
        .field_evidence_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("maintenance_delta"),
            delta.evidence_identity(),
        )
        .field_shape(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("kind"),
            delta.kind().as_str(),
        )
        .field_usize(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("width"),
            delta.width() as usize,
        )
        .seal();
        let lowering_report_digest = lowering_report_identity.as_str().to_string();
        Self {
            maintenance_delta_digest: delta.maintenance_delta_digest().to_string(),
            lowering_report_identity,
            lowering_report_digest,
        }
    }

    pub fn maintenance_delta_digest(&self) -> &str {
        &self.maintenance_delta_digest
    }

    pub fn lowering_report_digest(&self) -> &str {
        &self.lowering_report_digest
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.lowering_report_identity
    }
}
