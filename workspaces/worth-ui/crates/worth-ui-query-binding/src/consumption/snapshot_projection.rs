use worth_query::facade::domain::WorthQueryInstalledDomainProjectionOutcome;

use crate::{WorthUiDomainEntry, WorthUiQueryViewDefinition};

use super::WorthUiInstalledProjectionTransfer;

/// Projection authority produced by a one-shot installed snapshot read.
pub struct WorthUiQuerySnapshotProjectionOutcome {
    transfer: WorthUiInstalledProjectionTransfer,
}

impl WorthUiQuerySnapshotProjectionOutcome {
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
