use forge_relational::facade::identity::RelationId;
use worth_schema::facade::{WorthRelationKind, WorthTopologyRelationKind};

pub(super) fn seeded_relation_id(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    snapshot: &forge_relational::facade::snapshots::SnapshotHandle,
    kind: WorthTopologyRelationKind,
) -> RelationId {
    runtime
        .read_truth()
        .read_snapshot(snapshot)
        .expect("seeded snapshot should remain readable")
        .relations()
        .iter()
        .find(|record| record.kind.kind_id == WorthRelationKind::Topology(kind).kind_id())
        .map(|record| record.relation_id)
        .expect("seeded topology should contain requested relation kind")
}
