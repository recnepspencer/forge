use worth_query::facade::{domain, runtime};

use crate::{WorthUiDomainEntry, WorthUiQueryLiveResource, WorthUiQueryViewDefinition};

#[must_use = "failed close outcomes retain the Query live resource for retry"]
pub enum WorthUiQueryLiveCloseOutcome {
    Closed(Box<domain::WorthQueryInstalledDomainLiveCloseReceipt>),
    RuntimeStopped(WorthUiQueryLiveRuntimeCloseStop),
    AuthorityStopped(WorthUiQueryLiveAuthorityCloseStop),
}

pub struct WorthUiQueryLiveRuntimeCloseStop {
    definition: WorthUiQueryViewDefinition,
    query_stop: Box<domain::WorthQueryInstalledDomainLiveCloseStop<WorthUiDomainEntry>>,
}

impl WorthUiQueryLiveRuntimeCloseStop {
    pub fn error(&self) -> &runtime::WorthQueryRuntimeError {
        self.query_stop.error()
    }

    pub fn into_resource(self) -> WorthUiQueryLiveResource {
        WorthUiQueryLiveResource::new(self.definition, self.query_stop.into_handle())
    }
}

pub struct WorthUiQueryLiveAuthorityCloseStop {
    resource: WorthUiQueryLiveResource,
    drift: Box<domain::WorthQueryInstalledDomainExecutionDrift>,
}

impl WorthUiQueryLiveAuthorityCloseStop {
    pub fn drift(&self) -> &domain::WorthQueryInstalledDomainExecutionDrift {
        &self.drift
    }

    pub fn into_resource(self) -> WorthUiQueryLiveResource {
        self.resource
    }
}

impl WorthUiQueryLiveCloseOutcome {
    pub(crate) fn from_query(
        definition: WorthUiQueryViewDefinition,
        outcome: domain::WorthQueryInstalledDomainLiveCloseOutcome<WorthUiDomainEntry>,
    ) -> Self {
        match outcome {
            domain::WorthQueryInstalledDomainLiveCloseOutcome::Closed(receipt) => {
                Self::Closed(Box::new(receipt))
            }
            domain::WorthQueryInstalledDomainLiveCloseOutcome::RuntimeStopped(query_stop) => {
                Self::RuntimeStopped(WorthUiQueryLiveRuntimeCloseStop {
                    definition,
                    query_stop: Box::new(query_stop),
                })
            }
            domain::WorthQueryInstalledDomainLiveCloseOutcome::AuthorityStopped(
                query_handle,
                drift,
            ) => Self::AuthorityStopped(WorthUiQueryLiveAuthorityCloseStop {
                resource: WorthUiQueryLiveResource::new(definition, query_handle),
                drift: Box::new(drift),
            }),
        }
    }
}
