use worth_foundational::facade::AspectValue;
use worth_runtime_bridge::facade::{
    BridgeSnapshotReadError, RelationalBridgeSourceError, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthSnapshotIdentity,
    TruthSnapshotReader,
};

#[derive(Clone)]
pub(super) struct VersionedFixtureSnapshotSource {
    baseline_version: u64,
    before: AspectValue,
    after: AspectValue,
}

impl VersionedFixtureSnapshotSource {
    pub(super) fn new(baseline_version: u64, before: AspectValue, after: AspectValue) -> Self {
        Self {
            baseline_version,
            before,
            after,
        }
    }
}

impl SnapshotReadSource for VersionedFixtureSnapshotSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        let version = identity
            .relational_snapshot_parts()
            .map(|parts| parts.version_id())
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(
                    "threshold fixture requires a relational snapshot identity",
                )
            })?;
        Ok(Box::new(VersionedFixtureSnapshotReader {
            identity: identity.clone(),
            value: if version <= self.baseline_version {
                self.before.clone()
            } else {
                self.after.clone()
            },
        }))
    }
}

struct VersionedFixtureSnapshotReader {
    identity: TruthSnapshotIdentity,
    value: AspectValue,
}

impl TruthSnapshotReader for VersionedFixtureSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        Ok(SnapshotReadPacketResult::new(
            self.identity.clone(),
            request
                .reads()
                .iter()
                .map(|read| SnapshotReadRecord::for_request(read, self.value.clone()))
                .collect(),
        ))
    }
}
