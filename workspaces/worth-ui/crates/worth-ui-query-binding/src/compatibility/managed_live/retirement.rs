use std::collections::VecDeque;

use worth_query::facade::{domain, runtime};

use super::{
    WorthUiQueryLiveAuthorityCloseStop, WorthUiQueryLiveCloseOutcome, WorthUiQueryLiveResource,
    WorthUiQueryLiveRuntimeCloseStop,
};

#[must_use = "retired Query resources must be explicitly closed or deliberately abandoned"]
pub struct WorthUiQueryLiveRetirement {
    resources: VecDeque<WorthUiQueryLiveResource>,
    closed_receipts: Vec<domain::WorthQueryInstalledDomainLiveCloseReceipt>,
}

pub enum WorthUiQueryLiveRetirementCloseOutcome {
    Closed(WorthUiQueryLiveRetirementCloseReceipt),
    RuntimeStopped(WorthUiQueryLiveRetirementRuntimeStop),
    AuthorityStopped(WorthUiQueryLiveRetirementAuthorityStop),
}

pub struct WorthUiQueryLiveRetirementCloseReceipt {
    query_close_receipts: Vec<domain::WorthQueryInstalledDomainLiveCloseReceipt>,
}

pub struct WorthUiQueryLiveRetirementRuntimeStop {
    query_close_receipts: Vec<domain::WorthQueryInstalledDomainLiveCloseReceipt>,
    stop: WorthUiQueryLiveRuntimeCloseStop,
    remaining: VecDeque<WorthUiQueryLiveResource>,
}

pub struct WorthUiQueryLiveRetirementAuthorityStop {
    query_close_receipts: Vec<domain::WorthQueryInstalledDomainLiveCloseReceipt>,
    stop: WorthUiQueryLiveAuthorityCloseStop,
    remaining: VecDeque<WorthUiQueryLiveResource>,
}

impl WorthUiQueryLiveRetirement {
    pub(crate) fn new(resources: Vec<WorthUiQueryLiveResource>) -> Self {
        Self {
            resources: resources.into(),
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
    ) -> WorthUiQueryLiveRetirementCloseOutcome {
        while let Some(resource) = self.resources.pop_front() {
            match resource.close(workspace) {
                WorthUiQueryLiveCloseOutcome::Closed(receipt) => {
                    self.closed_receipts.push(*receipt)
                }
                WorthUiQueryLiveCloseOutcome::RuntimeStopped(stop) => {
                    return WorthUiQueryLiveRetirementCloseOutcome::RuntimeStopped(
                        WorthUiQueryLiveRetirementRuntimeStop {
                            query_close_receipts: self.closed_receipts,
                            stop,
                            remaining: self.resources,
                        },
                    );
                }
                WorthUiQueryLiveCloseOutcome::AuthorityStopped(stop) => {
                    return WorthUiQueryLiveRetirementCloseOutcome::AuthorityStopped(
                        WorthUiQueryLiveRetirementAuthorityStop {
                            query_close_receipts: self.closed_receipts,
                            stop,
                            remaining: self.resources,
                        },
                    );
                }
            }
        }
        WorthUiQueryLiveRetirementCloseOutcome::Closed(WorthUiQueryLiveRetirementCloseReceipt {
            query_close_receipts: self.closed_receipts,
        })
    }
}

impl WorthUiQueryLiveRetirementCloseReceipt {
    pub fn closed_resource_count(&self) -> usize {
        self.query_close_receipts.len()
    }

    pub fn query_close_receipts(
        &self,
    ) -> impl ExactSizeIterator<Item = &domain::WorthQueryInstalledDomainLiveCloseReceipt> {
        self.query_close_receipts.iter()
    }
}

impl WorthUiQueryLiveRetirementRuntimeStop {
    pub fn closed_resource_count(&self) -> usize {
        self.query_close_receipts.len()
    }

    pub fn error(&self) -> &runtime::WorthQueryRuntimeError {
        self.stop.error()
    }

    pub fn into_retirement(self) -> WorthUiQueryLiveRetirement {
        let mut resources = self.remaining;
        resources.push_front(self.stop.into_resource());
        WorthUiQueryLiveRetirement {
            resources,
            closed_receipts: self.query_close_receipts,
        }
    }
}

impl WorthUiQueryLiveRetirementAuthorityStop {
    pub fn closed_resource_count(&self) -> usize {
        self.query_close_receipts.len()
    }

    pub fn drift(&self) -> &worth_query::facade::domain::WorthQueryInstalledDomainExecutionDrift {
        self.stop.drift()
    }

    pub fn into_retirement(self) -> WorthUiQueryLiveRetirement {
        let mut resources = self.remaining;
        resources.push_front(self.stop.into_resource());
        WorthUiQueryLiveRetirement {
            resources,
            closed_receipts: self.query_close_receipts,
        }
    }
}
