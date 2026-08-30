use std::path::{Path, PathBuf};

use super::{
    FreshRootArtifactRow, RootSliceScenario, RootWireDenial, RootWireIdentity, RootWireRole,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OfflineRootObservationConnectorRequest {
    isolated_store_root: PathBuf,
    external_report_path: PathBuf,
    wire: RootWireIdentity,
}

impl OfflineRootObservationConnectorRequest {
    pub(crate) fn new(
        scenario: &RootSliceScenario,
        row: &FreshRootArtifactRow,
        expected_run: [u8; 32],
        wire: RootWireIdentity,
    ) -> Result<Self, RootWireDenial> {
        wire.require_binding(
            RootWireRole::OfflineVerifier,
            scenario.identity(),
            expected_run,
        )?;
        Ok(Self {
            isolated_store_root: row.root().to_path_buf(),
            external_report_path: scenario.reports().offline().to_path_buf(),
            wire,
        })
    }

    pub(crate) fn isolated_store_root(&self) -> &Path {
        &self.isolated_store_root
    }
    pub(crate) fn external_report_path(&self) -> &Path {
        &self.external_report_path
    }
    pub(crate) const fn wire(&self) -> &RootWireIdentity {
        &self.wire
    }
}
