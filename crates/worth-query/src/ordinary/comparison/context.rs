use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::ordinary::history::{at, WorthQueryHistoricalContext};
use crate::runtime::WorthQueryWorkspace;

/// A sealed structural pair of the current runtime basis and one retained
/// historical basis. Both endpoints are captured together, so a consumer
/// cannot substitute a digest or tuple after declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryComparisonContext {
    current_snapshot: WorthQuerySnapshotIdentity,
    retained: WorthQueryHistoricalContext,
}

impl WorthQueryComparisonContext {
    pub fn current_snapshot(&self) -> &WorthQuerySnapshotIdentity {
        &self.current_snapshot
    }

    pub fn retained_snapshot(&self) -> &WorthQuerySnapshotIdentity {
        self.retained.snapshot_identity()
    }

    pub(crate) fn retained_context(&self) -> WorthQueryHistoricalContext {
        self.retained.clone()
    }
}

pub fn current_and_retained(workspace: &WorthQueryWorkspace) -> WorthQueryComparisonContext {
    WorthQueryComparisonContext {
        current_snapshot: workspace.snapshot_identity(),
        retained: at(workspace),
    }
}
