use worth_query::facade::{domain, live};

use super::WorthUiQueryLiveResource;
use crate::{WorthUiDomainEntry, WorthUiQueryViewDefinition};

#[derive(Debug)]
pub enum WorthUiQueryLiveOpenError {
    Declaration(Box<live::WorthQueryLiveDeclarationStop>),
    InstalledAuthority(
        Box<
            domain::WorthQueryInstalledDomainCapabilityStop<
                domain::WorthQueryInstalledDomainExecutionDrift,
            >,
        >,
    ),
}

#[must_use = "an opened Query live resource must be retained or explicitly closed"]
pub enum WorthUiQueryLiveOpenOutcome {
    Opened(WorthUiQueryLiveResource),
    Stopped(Box<domain::WorthQueryInstalledDomainCapabilityStop<live::WorthQueryLiveOpenStop>>),
}

impl WorthUiQueryLiveOpenOutcome {
    pub(crate) fn from_query(
        definition: WorthUiQueryViewDefinition,
        outcome: domain::WorthQueryInstalledDomainLiveOpenOutcome<WorthUiDomainEntry>,
    ) -> Self {
        match outcome {
            domain::WorthQueryInstalledDomainLiveOpenOutcome::Opened(handle) => {
                Self::Opened(WorthUiQueryLiveResource::new(definition, handle))
            }
            domain::WorthQueryInstalledDomainLiveOpenOutcome::Stopped(stop) => {
                Self::Stopped(Box::new(stop))
            }
        }
    }
}
