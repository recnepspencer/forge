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
        let version_id = self
            .runtime
            .with_runtime(|runtime| resolve_snapshot_version(runtime, identity))?;
        let reader = match &self.partition {
            Some(partition) => RuntimePublicationSnapshotReader::for_partition_authority(
                self.runtime.clone(),
                identity.clone(),
                version_id,
                partition.relational,
            ),
            None => RuntimePublicationSnapshotReader::from_authority(
                self.runtime.clone(),
                identity.clone(),
                version_id,
            ),
        };
        Ok(Box::new(reader))
    }
}
