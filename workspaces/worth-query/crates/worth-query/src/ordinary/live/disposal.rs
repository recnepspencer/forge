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
    disposal_work: WorthQueryManagedLiveDisposalWork,
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

    pub fn disposal_work(&self) -> &WorthQueryManagedLiveDisposalWork {
        &self.disposal_work
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryManagedLiveDisposalWork {
    consumer_attachment_close_count: u64,
    active_lane_close_count: u64,
    lifecycle_closeout_count: u64,
    budget_consumption_width: u64,
    budget_remaining_width: u64,
}

impl WorthQueryManagedLiveDisposalWork {
    fn from_closeout(closeout: &crate::subscription::SubscriptionLifecycleCloseout) -> Self {
        let counters = closeout.counters();
        Self {
            consumer_attachment_close_count: counters.consumer_attachment_close_count(),
            active_lane_close_count: counters.active_lane_close_count(),
            lifecycle_closeout_count: counters.subscription_lifecycle_closeout_count(),
            budget_consumption_width: counters.subscription_budget_consumption_width(),
            budget_remaining_width: counters.subscription_budget_remaining_width(),
        }
    }

    pub fn consumer_attachment_close_count(&self) -> u64 {
        self.consumer_attachment_close_count
    }

    pub fn active_lane_close_count(&self) -> u64 {
        self.active_lane_close_count
    }

    pub fn lifecycle_closeout_count(&self) -> u64 {
        self.lifecycle_closeout_count
    }

    pub fn budget_consumption_width(&self) -> u64 {
        self.budget_consumption_width
    }

    pub fn budget_remaining_width(&self) -> u64 {
        self.budget_remaining_width
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
                    disposal_work: WorthQueryManagedLiveDisposalWork::from_closeout(&closeout),
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
