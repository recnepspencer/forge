use std::sync::Arc;

use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::{
    BridgeSnapshotReadError, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::identities::parse_bridge_record_identity;
use super::snapshot_values::{
    export_entity_aspect_snapshot_bytes, export_relation_aspect_snapshot_bytes,
};

#[derive(Debug, Clone)]
pub(crate) struct RuntimePublicationSnapshotReader {
    runtime: Arc<RelationalRuntime>,
    snapshot_identity: TruthSnapshotIdentity,
    version_id: VersionId,
}

impl RuntimePublicationSnapshotReader {
    pub(crate) fn new(
        runtime: Arc<RelationalRuntime>,
        snapshot_identity: TruthSnapshotIdentity,
        version_id: VersionId,
    ) -> Self {
        Self {
            runtime,
            snapshot_identity,
            version_id,
        }
    }
}

impl TruthSnapshotReader for RuntimePublicationSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.snapshot_identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        let read_truth = self.runtime.read_truth();
        let mut records = Vec::with_capacity(request.reads().len());
        for read in request.reads() {
            let record_ref = parse_bridge_record_identity(read.entity_identity())
                .map_err(|error| BridgeSnapshotReadError::new(error.to_string()))?;
            let aspect_bytes = match record_ref {
                crate::transactions::data::RecordRef::Entity(entity_id) => {
                    let record = read_truth
                        .unmasked_entity_record_at_version(entity_id, self.version_id)
                        .ok_or_else(|| {
                        BridgeSnapshotReadError::new(format!(
                            "relational bridge snapshot reader could not find entity `{}` in authoritative snapshot `{}`",
                            read.entity_identity(),
                            self.snapshot_identity.as_str()
                        ))
                    })?;
                    export_entity_aspect_snapshot_bytes(&record, read.aspect_key()).ok_or_else(|| {
                        BridgeSnapshotReadError::new(format!(
                            "relational bridge snapshot reader could not resolve aspect `{}` on entity `{}` in authoritative snapshot `{}`",
                            read.aspect_key().as_str(),
                            read.entity_identity(),
                            self.snapshot_identity.as_str()
                        ))
                    })?
                }
                crate::transactions::data::RecordRef::Relation(relation_id) => {
                    let record = read_truth
                        .unmasked_relation_record_at_version(relation_id, self.version_id)
                        .ok_or_else(|| {
                        BridgeSnapshotReadError::new(format!(
                            "relational bridge snapshot reader could not find relation `{}` in authoritative snapshot `{}`",
                            read.entity_identity(),
                            self.snapshot_identity.as_str()
                        ))
                    })?;
                    export_relation_aspect_snapshot_bytes(&record, read.aspect_key()).ok_or_else(|| {
                        BridgeSnapshotReadError::new(format!(
                            "relational bridge snapshot reader could not resolve aspect `{}` on relation `{}` in authoritative snapshot `{}`",
                            read.aspect_key().as_str(),
                            read.entity_identity(),
                            self.snapshot_identity.as_str()
                        ))
                    })?
                }
            };
            records.push(SnapshotReadRecord::new(read.request_key(), aspect_bytes));
        }

        Ok(SnapshotReadPacketResult::new(
            self.snapshot_identity.clone(),
            records,
        ))
    }
}
