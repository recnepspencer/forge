use crate::identity::data::VersionId;
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
    runtime: super::runtime_source::runtime_authority::RelationalBridgeRuntimeAuthority,
    snapshot_identity: TruthSnapshotIdentity,
    version_id: VersionId,
    partition: Option<crate::identity::data::PartitionId>,
}

impl RuntimePublicationSnapshotReader {
    #[cfg(test)]
    pub(crate) fn new(
        runtime: std::sync::Arc<crate::logic::runtime::RelationalRuntime>,
        snapshot_identity: TruthSnapshotIdentity,
        version_id: VersionId,
    ) -> Self {
        Self::from_authority(
            super::runtime_source::runtime_authority::RelationalBridgeRuntimeAuthority::immutable(
                runtime,
            ),
            snapshot_identity,
            version_id,
        )
    }

    pub(super) fn from_authority(
        runtime: super::runtime_source::runtime_authority::RelationalBridgeRuntimeAuthority,
        snapshot_identity: TruthSnapshotIdentity,
        version_id: VersionId,
    ) -> Self {
        Self {
            runtime,
            snapshot_identity,
            version_id,
            partition: None,
        }
    }

    pub(super) fn for_partition_authority(
        runtime: super::runtime_source::runtime_authority::RelationalBridgeRuntimeAuthority,
        snapshot_identity: TruthSnapshotIdentity,
        version_id: VersionId,
        partition: crate::identity::data::PartitionId,
    ) -> Self {
        Self {
            runtime,
            snapshot_identity,
            version_id,
            partition: Some(partition),
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
        self.runtime
            .with_runtime(|runtime| read_packet(runtime, request, self.version_id, self.partition))
            .map(|records| SnapshotReadPacketResult::new(self.snapshot_identity.clone(), records))
    }
}

fn read_packet(
    runtime: &crate::logic::runtime::RelationalRuntime,
    request: &SnapshotReadPacket,
    version_id: VersionId,
    partition: Option<crate::identity::data::PartitionId>,
) -> Result<Vec<SnapshotReadRecord>, BridgeSnapshotReadError> {
    let read_truth = runtime.read_truth();
    let mut records = Vec::with_capacity(request.reads().len());
    for read in request.reads() {
        let identity_parts = read.relational_record_identity_parts().ok_or_else(|| {
            BridgeSnapshotReadError::new(
                "relational bridge snapshot reader requires typed record identity parts",
            )
        })?;
        if partition.is_some_and(|bound| bound.as_u32() != identity_parts.partition_id()) {
            return Err(BridgeSnapshotReadError::new(
                "relational bridge snapshot read is outside the source partition authority",
            ));
        }
        let record_ref = record_ref_from_identity_parts(identity_parts)
            .map_err(|error| BridgeSnapshotReadError::new(error.to_string()))?;
        let aspect_value = match record_ref {
            crate::transactions::data::RecordRef::Entity(entity_id) => {
                let record_label = format!(
                    "entity:{}:{}:{}",
                    entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
                );
                let record = read_truth
                        .authoritative_entity_record_at_version(entity_id, version_id)
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
                    relation_id.partition_id.0, relation_id.local_slot.0, relation_id.generation.0
                );
                let record = read_truth
                        .authoritative_relation_record_at_version(relation_id, version_id)
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
    Ok(records)
}
