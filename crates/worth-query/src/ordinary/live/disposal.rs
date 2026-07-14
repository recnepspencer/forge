use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::runtime::{WorthQueryRuntimeError, WorthQueryWorkspace};

use super::WorthQueryManagedLiveHandle;

#[derive(Debug)]
#[must_use = "failed close outcomes retain the managed handle for retry"]
pub enum WorthQueryManagedLiveCloseOutcome {
    Closed(WorthQueryManagedLiveCloseReceipt),
    Stopped(WorthQueryManagedLiveCloseStop),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryManagedLiveCloseReceipt {
    resource_name: String,
    closeout_identity: WorthQueryEvidenceIdentity,
    lane_terminal: bool,
}

impl WorthQueryManagedLiveCloseReceipt {
    pub fn resource_name(&self) -> &str {
        &self.resource_name
    }

    pub fn closeout_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.closeout_identity
    }

    pub fn lane_terminal(&self) -> bool {
        self.lane_terminal
    }
}

#[derive(Debug)]
pub struct WorthQueryManagedLiveCloseStop {
    handle: WorthQueryManagedLiveHandle,
    error: WorthQueryRuntimeError,
}

impl WorthQueryManagedLiveCloseStop {
    pub fn error(&self) -> &WorthQueryRuntimeError {
        &self.error
    }

    pub fn into_handle(self) -> WorthQueryManagedLiveHandle {
        self.handle
    }
}

impl WorthQueryManagedLiveHandle {
    pub fn close(
        mut self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryManagedLiveCloseOutcome {
        match workspace.close_managed_live_view(self.view(), self.workspace_capability()) {
            Ok(closeout) => {
                let receipt = WorthQueryManagedLiveCloseReceipt {
                    resource_name: self.name().to_string(),
                    closeout_identity: closeout.evidence_identity().clone(),
                    lane_terminal: closeout.lane_terminal(),
                };
                self.disarm();
                WorthQueryManagedLiveCloseOutcome::Closed(receipt)
            }
            Err(error) => {
                WorthQueryManagedLiveCloseOutcome::Stopped(WorthQueryManagedLiveCloseStop {
                    handle: self,
                    error,
                })
            }
        }
    }
}
