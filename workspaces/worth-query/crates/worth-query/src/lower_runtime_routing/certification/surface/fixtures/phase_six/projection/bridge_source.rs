use worth_foundational::facade::{AspectKey, AspectValue};
use worth_runtime_bridge::facade::{
    GroupedProjectionMemberSource, GroupedProjectionSource, RelationalBridgeRecordIdentityParts,
    TruthSnapshotIdentity,
};

use super::relational_row_identity;

#[derive(Debug)]
pub(super) struct BridgeProjection {
    pub(super) snapshot_identity: TruthSnapshotIdentity,
    pub(super) grouping_aspect: AspectKey,
    pub(super) identity_binding_aspect_key: AspectKey,
    pub(super) grouping_binding_aspect_key: AspectKey,
    pub(super) members: Vec<BridgeProjectionMember>,
}

#[derive(Debug)]
pub(super) struct BridgeProjectionMember {
    row_identity: String,
    identity_value: AspectValue,
    grouping_value: AspectValue,
}

impl BridgeProjectionMember {
    pub(super) fn new(
        row_identity: RelationalBridgeRecordIdentityParts,
        identity_value: &str,
        grouping_value: &str,
    ) -> Self {
        Self {
            row_identity: relational_row_identity(row_identity),
            identity_value: crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(
                identity_value,
            ),
            grouping_value: crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(
                grouping_value,
            ),
        }
    }
}

impl GroupedProjectionMemberSource for BridgeProjectionMember {
    fn row_identity(&self) -> &str {
        &self.row_identity
    }

    fn identity_value(&self) -> &AspectValue {
        &self.identity_value
    }

    fn grouping_value(&self) -> &AspectValue {
        &self.grouping_value
    }
}

impl GroupedProjectionSource for BridgeProjection {
    type Member = BridgeProjectionMember;

    fn basis_snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    fn grouping_aspect_key(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    fn identity_binding_aspect_key(&self) -> &AspectKey {
        &self.identity_binding_aspect_key
    }

    fn grouping_binding_aspect_key(&self) -> &AspectKey {
        &self.grouping_binding_aspect_key
    }

    fn members(&self) -> &[Self::Member] {
        &self.members
    }
}
