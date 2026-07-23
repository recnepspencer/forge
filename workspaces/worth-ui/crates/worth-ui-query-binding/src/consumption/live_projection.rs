use worth_query::facade::domain::WorthQueryInstalledDomainProjectionOutcome;

use crate::{WorthUiDomainEntry, WorthUiQueryViewDefinition};

use super::WorthUiInstalledProjectionTransfer;

/// Projection authority produced by one exact Query-owned live resource.
pub struct WorthUiQueryLiveProjectionOutcome {
    transfer: WorthUiInstalledProjectionTransfer,
}

impl WorthUiQueryLiveProjectionOutcome {
    pub(crate) fn from_installed(
        definition: WorthUiQueryViewDefinition,
        installed: WorthQueryInstalledDomainProjectionOutcome<WorthUiDomainEntry>,
    ) -> Self {
        Self {
            transfer: WorthUiInstalledProjectionTransfer::from_installed(definition, installed),
        }
    }

    pub(crate) fn into_transfer(self) -> WorthUiInstalledProjectionTransfer {
        self.transfer
    }
}
