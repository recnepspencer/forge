use forge_relational::facade::{
    identity::{EntityId, PartitionId},
    payloads::RecordPayload,
    runtime::RelationalRuntime,
    symbols::InternedString,
    transactions::{
        CommitResult, CreateIntent, EntitySpec, MutationIntent, RelationSpec,
        TransactionCommitError, TransactionOptions, WorkerIntentBatch,
    },
};
use serde_json::json;

use crate::data::entities::{WorthEntityKind, WorthTopologyEntityKind};
use crate::data::relations::{WorthRelationKind, WorthTopologyRelationKind};
use crate::data::seed::labels::WorthMinimalTopologyLabels;
use crate::data::seed::types::WorthMinimalTopologySeed;

pub fn create_bootstrap_entities(
    runtime: &mut RelationalRuntime,
    labels: &WorthMinimalTopologyLabels,
) -> Result<CommitResult, TransactionCommitError> {
    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(
        WorkerIntentBatch::new("worth-seed-entities")
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
                labels.model.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
                labels.body.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Lump),
                labels.lump.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Region),
                labels.region.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
                labels.shell.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
                labels.face.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
                labels.outer_loop.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Wire),
                labels.wire.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
                labels.half_edge.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Edge),
                labels.edge.as_str(),
            ))
            .push(create_entity_intent(
                WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
                labels.vertex.as_str(),
            )),
    );
    tx.commit()
}

pub fn create_bootstrap_relations(
    runtime: &mut RelationalRuntime,
    ids: &WorthMinimalTopologySeed,
    labels: &WorthMinimalTopologyLabels,
) -> Result<CommitResult, TransactionCommitError> {
    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(
        WorkerIntentBatch::new("worth-seed-relations")
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody),
                &format!("{}.owns_body", labels.model),
                ids.model,
                ids.body,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump),
                &format!("{}.owns_lump", labels.body),
                ids.body,
                ids.lump,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion),
                &format!("{}.owns_region", labels.lump),
                ids.lump,
                ids.region,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell),
                &format!("{}.owns_shell", labels.region),
                ids.region,
                ids.shell,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace),
                &format!("{}.owns_face", labels.shell),
                ids.shell,
                ids.face,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::FaceOuterLoop),
                &format!("{}.outer_loop", labels.face),
                ids.face,
                ids.outer_loop,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::LoopOwnsHalfEdge),
                &format!("{}.owns_half_edge", labels.outer_loop),
                ids.outer_loop,
                ids.half_edge,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge),
                &format!("{}.owns_half_edge", labels.wire),
                ids.wire,
                ids.half_edge,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext),
                &format!("{}.next", labels.half_edge),
                ids.half_edge,
                ids.half_edge,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgePrev),
                &format!("{}.prev", labels.half_edge),
                ids.half_edge,
                ids.half_edge,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext),
                &format!("{}.radial", labels.half_edge),
                ids.half_edge,
                ids.half_edge,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeUsesEdge),
                &format!("{}.edge", labels.half_edge),
                ids.half_edge,
                ids.edge,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex),
                &format!("{}.start_vertex", labels.half_edge),
                ids.half_edge,
                ids.vertex,
            ))
            .push(create_relation_intent(
                WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex),
                &format!("{}.end_vertex", labels.half_edge),
                ids.half_edge,
                ids.vertex,
            )),
    );
    tx.commit()
}

fn create_entity_intent(kind: WorthEntityKind, label: &str) -> MutationIntent {
    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
        partition_id: PartitionId::main(),
        kind_id: kind.kind_id(),
        client_key: InternedString::Raw(label.to_string()),
        payload: RecordPayload::StructuredJson(json!({ "label": label })),
    }))
}

fn create_relation_intent(
    kind: WorthRelationKind,
    label: &str,
    source: EntityId,
    target: EntityId,
) -> MutationIntent {
    MutationIntent::Create(CreateIntent::Relation(RelationSpec {
        partition_id: PartitionId::main(),
        kind_id: kind.kind_id(),
        client_key: InternedString::Raw(label.to_string()),
        source,
        target,
        payload: None,
    }))
}
