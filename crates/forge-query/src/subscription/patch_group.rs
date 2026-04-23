use crate::identity::hash_parts;

use super::maintenance_delta::QuerySubscriptionMaintenanceDeltaKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryPatchGroupKind {
    DetailFieldPatchGroup,
    InspectorFocusedPatchGroup,
    CollectionMembershipPatchGroup,
    CollectionOrderPatchGroup,
    GroupedMembershipPatchGroup,
    BoundedMaterializationScopePatchGroup,
    ContinuationPatchGroup,
    DeliveryGapPatchGroup,
}

impl QueryPatchGroupKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailFieldPatchGroup => "detail_field_patch_group",
            Self::InspectorFocusedPatchGroup => "inspector_focused_patch_group",
            Self::CollectionMembershipPatchGroup => "collection_membership_patch_group",
            Self::CollectionOrderPatchGroup => "collection_order_patch_group",
            Self::GroupedMembershipPatchGroup => "grouped_membership_patch_group",
            Self::BoundedMaterializationScopePatchGroup => {
                "bounded_materialization_scope_patch_group"
            }
            Self::ContinuationPatchGroup => "continuation_patch_group",
            Self::DeliveryGapPatchGroup => "delivery_gap_patch_group",
        }
    }

    pub(super) fn from_delta_kind(kind: QuerySubscriptionMaintenanceDeltaKind) -> Self {
        match kind {
            QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta => Self::DetailFieldPatchGroup,
            QuerySubscriptionMaintenanceDeltaKind::InspectorFocusDelta => {
                Self::InspectorFocusedPatchGroup
            }
            QuerySubscriptionMaintenanceDeltaKind::CollectionMembershipDelta => {
                Self::CollectionMembershipPatchGroup
            }
            QuerySubscriptionMaintenanceDeltaKind::CollectionOrderDelta => {
                Self::CollectionOrderPatchGroup
            }
            QuerySubscriptionMaintenanceDeltaKind::GroupedMembershipDelta => {
                Self::GroupedMembershipPatchGroup
            }
            QuerySubscriptionMaintenanceDeltaKind::BoundedMaterializationScopeDelta => {
                Self::BoundedMaterializationScopePatchGroup
            }
            QuerySubscriptionMaintenanceDeltaKind::ContinuationDelta => {
                Self::ContinuationPatchGroup
            }
            QuerySubscriptionMaintenanceDeltaKind::GapNoticeDelta => Self::DeliveryGapPatchGroup,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPatchGroup {
    kind: QueryPatchGroupKind,
    width: u64,
    patch_group_digest: String,
}

impl QueryPatchGroup {
    pub(super) fn new(kind: QueryPatchGroupKind, source_digest: &str, width: u64) -> Self {
        let patch_group_digest = hash_parts(&[
            "query_patch_group_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("source:{}", source_digest),
            format!("width:{}", width),
        ]);
        Self {
            kind,
            width,
            patch_group_digest,
        }
    }

    pub fn kind(&self) -> QueryPatchGroupKind {
        self.kind
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn patch_group_digest(&self) -> &str {
        &self.patch_group_digest
    }
}
