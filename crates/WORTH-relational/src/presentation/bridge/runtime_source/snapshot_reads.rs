use worth_runtime_bridge::facade::{
    RelationalBridgeSourceError, SnapshotReadSource, TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::snapshot_authority::resolve_snapshot_version;
use super::RuntimeBridgeRelationalSource;
use crate::presentation::bridge::snapshot_reading::RuntimePublicationSnapshotReader;

impl SnapshotReadSource for RuntimeBridgeRelationalSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        let version_id = resolve_snapshot_version(&self.runtime, identity)?;
        Ok(Box::new(RuntimePublicationSnapshotReader::new(
            std::sync::Arc::clone(&self.runtime),
            identity.clone(),
            version_id,
        )))
    }
}
