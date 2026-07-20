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
        let reader = match &self.partition {
            Some(partition) => RuntimePublicationSnapshotReader::for_partition(
                std::sync::Arc::clone(&self.runtime),
                identity.clone(),
                version_id,
                partition.relational,
            ),
            None => RuntimePublicationSnapshotReader::new(
                std::sync::Arc::clone(&self.runtime),
                identity.clone(),
                version_id,
            ),
        };
        Ok(Box::new(reader))
    }
}
