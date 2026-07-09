use std::sync::Arc;

use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use worth_runtime_bridge::facade::{
    BridgeSnapshotReadError, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::identities::record_ref_from_identity_parts;
use super::snapshot_values::{
    export_entity_aspect_snapshot_value, export_relation_aspect_snapshot_value,
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
            let record_ref = read
                .relational_record_identity_parts()
                .ok_or_else(|| {
                    BridgeSnapshotReadError::new(
                        "relational bridge snapshot reader requires typed record identity parts",
                    )
                })
                .and_then(|parts| {
                    record_ref_from_identity_parts(parts)
                        .map_err(|error| BridgeSnapshotReadError::new(error.to_string()))
                })?;
            let aspect_value = match record_ref {
                crate::transactions::data::RecordRef::Entity(entity_id) => {
                    let record_label = format!(
                        "entity:{}:{}:{}",
                        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
                    );
                    let record = read_truth
                        .authoritative_entity_record_at_version(entity_id, self.version_id)
                        .ok_or_else(|| {
                        BridgeSnapshotReadError::new(format!(
                            "relational bridge snapshot reader could not find entity `{}` in authoritative snapshot",
                            record_label
                        ))
                    })?;
                    export_entity_aspect_snapshot_value(&record, read.aspect_key()).ok_or_else(|| {
                        BridgeSnapshotReadError::new(format!(
                            "relational bridge snapshot reader could not resolve aspect `{}` on entity `{}` in authoritative snapshot",
                            read.aspect_key().as_str(),
                            record_label
                        ))
                    })?
                }
                crate::transactions::data::RecordRef::Relation(relation_id) => {
                    let record_label = format!(
                        "relation:{}:{}:{}",
                        relation_id.partition_id.0,
                        relation_id.local_slot.0,
                        relation_id.generation.0
                    );
                    let record = read_truth
                        .authoritative_relation_record_at_version(relation_id, self.version_id)
                        .ok_or_else(|| {
                        BridgeSnapshotReadError::new(format!(
                            "relational bridge snapshot reader could not find relation `{}` in authoritative snapshot",
                            record_label
                        ))
                    })?;
                    export_relation_aspect_snapshot_value(&record, read.aspect_key()).ok_or_else(|| {
                        BridgeSnapshotReadError::new(format!(
                            "relational bridge snapshot reader could not resolve aspect `{}` on relation `{}` in authoritative snapshot",
                            read.aspect_key().as_str(),
                            record_label
                        ))
                    })?
                }
            };
            records.push(SnapshotReadRecord::for_request(read, aspect_value));
        }

        Ok(SnapshotReadPacketResult::new(
            self.snapshot_identity.clone(),
            records,
        ))
    }
}
