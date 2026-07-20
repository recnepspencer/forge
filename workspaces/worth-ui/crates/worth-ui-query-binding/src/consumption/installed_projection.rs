use worth_query::facade::domain::{
    WorthQueryInstalledDomainExecutionReceipt, WorthQueryInstalledDomainProjectionOutcome,
};
use worth_query::facade::read::WorthQueryProjectionOutcome;

use crate::{WorthUiDomainEntry, WorthUiQueryViewDefinition};

/// Shared installed-domain transfer held behind lifecycle-specific public
/// projection envelopes.
pub(crate) struct WorthUiInstalledProjectionTransfer {
    definition: WorthUiQueryViewDefinition,
    outcome: WorthQueryProjectionOutcome,
    installed_execution: WorthQueryInstalledDomainExecutionReceipt,
}

impl WorthUiInstalledProjectionTransfer {
    pub(crate) fn from_installed(
        definition: WorthUiQueryViewDefinition,
        installed: WorthQueryInstalledDomainProjectionOutcome<WorthUiDomainEntry>,
    ) -> Self {
        let (outcome, installed_execution) = installed.into_parts();
        Self {
            definition,
            outcome,
            installed_execution,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthUiQueryViewDefinition,
        WorthQueryProjectionOutcome,
        WorthQueryInstalledDomainExecutionReceipt,
    ) {
        (self.definition, self.outcome, self.installed_execution)
    }
}
