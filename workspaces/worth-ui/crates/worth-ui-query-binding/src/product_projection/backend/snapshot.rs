use worth_query::facade::{foundation, runtime};
use worth_runtime_bridge::facade::{RelationalBridgeSnapshotIdentityParts, TruthSnapshotIdentity};

use super::state::SharedSourceState;

pub(super) struct WorthUiScalarProjectionSnapshotIdentity {
    state: SharedSourceState,
}

impl WorthUiScalarProjectionSnapshotIdentity {
    pub(super) fn new(state: SharedSourceState) -> Self {
        Self { state }
    }
}

impl runtime::WorthQueryRuntimeSnapshotIdentityAdapter for WorthUiScalarProjectionSnapshotIdentity {
    fn current_snapshot_identity(&self) -> foundation::WorthQuerySnapshotIdentity {
        projection_snapshot_identity(self.state.borrow().current_snapshot_version())
    }
}

pub(super) fn projection_snapshot_identity(version: u64) -> foundation::WorthQuerySnapshotIdentity {
    foundation::WorthQuerySnapshotIdentity::from_bridge_snapshot_projection(
        TruthSnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(313, version),
        ),
    )
    .expect("product relational snapshot projection retains typed parts")
}
