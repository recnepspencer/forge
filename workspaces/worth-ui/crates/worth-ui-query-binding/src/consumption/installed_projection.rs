use worth_query::facade::domain::{
    WorthQueryInstalledDomainExecutionReceipt, WorthQueryInstalledDomainProjectionOutcome,
};
use worth_query::facade::read::WorthQueryProjectionOutcome;

use crate::{WorthUiDomainEntry, WorthUiQueryViewDefinition};

/// Installed-domain projection transfer envelope. The ordinary projection and
/// the installed execution receipt cannot be separated by Worth UI callers.
pub struct WorthUiQueryProjectionOutcome {
    definition: WorthUiQueryViewDefinition,
    outcome: WorthQueryProjectionOutcome,
    installed_execution: WorthQueryInstalledDomainExecutionReceipt,
}

impl WorthUiQueryProjectionOutcome {
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
