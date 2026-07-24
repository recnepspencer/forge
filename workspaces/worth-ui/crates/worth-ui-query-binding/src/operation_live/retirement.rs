use std::collections::VecDeque;

use worth_query::facade::runtime;

use crate::{
    WorthUiOperationLiveCloseOutcome, WorthUiOperationLiveCloseReceipt,
    WorthUiOperationLiveCloseStop, WorthUiOperationLiveResource,
};

#[must_use = "retired operation-live resources must be closed or deliberately abandoned"]
pub struct WorthUiOperationLiveRetirement {
    resources: VecDeque<WorthUiOperationLiveResource>,
    closed_receipts: Vec<WorthUiOperationLiveCloseReceipt>,
}

impl std::fmt::Debug for WorthUiOperationLiveRetirement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiOperationLiveRetirement")
            .field("remaining_resource_count", &self.resources.len())
            .field("closed_resource_count", &self.closed_receipts.len())
            .finish()
    }
}

pub enum WorthUiOperationLiveRetirementCloseOutcome {
    Closed(WorthUiOperationLiveRetirementCloseReceipt),
    Stopped(Box<WorthUiOperationLiveRetirementStop>),
}

pub struct WorthUiOperationLiveRetirementCloseReceipt {
    closed_receipts: Vec<WorthUiOperationLiveCloseReceipt>,
}

pub struct WorthUiOperationLiveRetirementStop {
    closed_receipts: Vec<WorthUiOperationLiveCloseReceipt>,
    stopped: WorthUiOperationLiveCloseStop,
    remaining: VecDeque<WorthUiOperationLiveResource>,
}

impl WorthUiOperationLiveRetirement {
    pub(crate) fn new(resources: Vec<WorthUiOperationLiveResource>) -> Self {
        Self {
            resources: resources.into(),
            closed_receipts: Vec::new(),
        }
    }

    pub(crate) fn with_resource_capacity(capacity: usize) -> Self {
        Self {
            resources: VecDeque::with_capacity(capacity),
            closed_receipts: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn close(
        mut self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> WorthUiOperationLiveRetirementCloseOutcome {
        while let Some(resource) = self.resources.pop_front() {
            match resource.close(workspace) {
                WorthUiOperationLiveCloseOutcome::Closed(receipt) => {
                    self.closed_receipts.push(receipt);
                }
                WorthUiOperationLiveCloseOutcome::Stopped(stop) => {
                    return WorthUiOperationLiveRetirementCloseOutcome::Stopped(Box::new(
                        WorthUiOperationLiveRetirementStop {
                            closed_receipts: self.closed_receipts,
                            stopped: *stop,
                            remaining: self.resources,
                        },
                    ));
                }
            }
        }
        WorthUiOperationLiveRetirementCloseOutcome::Closed(
            WorthUiOperationLiveRetirementCloseReceipt {
                closed_receipts: self.closed_receipts,
            },
        )
    }
}

impl Extend<WorthUiOperationLiveResource> for WorthUiOperationLiveRetirement {
    fn extend<T: IntoIterator<Item = WorthUiOperationLiveResource>>(&mut self, resources: T) {
        self.resources.extend(resources);
    }
}

impl WorthUiOperationLiveRetirementCloseReceipt {
    pub fn closed_resource_count(&self) -> usize {
        self.closed_receipts.len()
    }

    pub fn query_close_receipts(
        &self,
    ) -> impl ExactSizeIterator<Item = &WorthUiOperationLiveCloseReceipt> {
        self.closed_receipts.iter()
    }
}

impl WorthUiOperationLiveRetirementStop {
    pub fn closed_resource_count(&self) -> usize {
        self.closed_receipts.len()
    }

    pub fn query_error(&self) -> &runtime::WorthQueryRuntimeError {
        self.stopped.query_error()
    }

    pub const fn counters(&self) -> runtime::WorthQuerySharedLeaseReleaseCounters {
        self.stopped.counters()
    }

    pub fn into_retirement(self) -> WorthUiOperationLiveRetirement {
        let mut resources = self.remaining;
        resources.push_front(self.stopped.into_resource());
        WorthUiOperationLiveRetirement {
            resources,
            closed_receipts: self.closed_receipts,
        }
    }
}
