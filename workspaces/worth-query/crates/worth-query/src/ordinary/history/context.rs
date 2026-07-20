use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::runtime::WorthQueryWorkspace;

/// Sealed evidence naming the exact runtime snapshot a historical query may
/// observe. Construct it through [`at`]; its fields cannot be fabricated by a
/// consumer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryHistoricalContext {
    workspace_name: String,
    snapshot_identity: WorthQuerySnapshotIdentity,
}

impl WorthQueryHistoricalContext {
    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn snapshot_identity(&self) -> &WorthQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub(crate) fn basis_label(&self) -> String {
        self.snapshot_identity
            .evidence_identity()
            .as_str()
            .to_string()
    }
}

/// Capture the runtime's currently retained snapshot as an explicit
/// historical basis. The captured identity is checked again at execution.
pub fn at(workspace: &WorthQueryWorkspace) -> WorthQueryHistoricalContext {
    WorthQueryHistoricalContext {
        workspace_name: workspace.name().to_string(),
        snapshot_identity: workspace.snapshot_identity(),
    }
}
