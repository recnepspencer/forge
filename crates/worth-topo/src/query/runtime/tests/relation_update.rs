use forge_query::facade::ForgeQueryExistingRelationTarget;
use forge_relational::facade::identity::RelationId;
use worth_schema::facade::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase, WorthTopologyRelationKind,
};

use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_denies_relation_updates_until_identity_preserving_authority_support_exists()
{
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-update-rewire-endpoint",
        &WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-update-rewire-endpoint")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let half_edge_name = "worth.current-head.query-update-rewire-endpoint.wire_open.half_edge.0";
    let target_vertex_name = "worth.current-head.query-update-rewire-endpoint.wire_open.vertex.2";
    let half_edge_identity = query_entity_identity_by_name(&entity_rows, half_edge_name);
    let relation = query_relation_row_by_kind_and_source(
        &relation_rows,
        WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
        &half_edge_identity,
    );
    let target_vertex_identity = query_entity_identity_by_name(&entity_rows, target_vertex_name);
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{:?}", query_relation_id_from_row(relation)),
                relation.identity.clone(),
            )
            .expect("non-empty relation binding identity")
            .in_target_collection("WorthTopologyRelation")
            .expect("target collection"),
        )
        .expect("bind existing relation");

    let error = workspace
        .update_existing(binding, |mutation| {
            mutation
                .aspect(
                    "topology.kind",
                    WorthTopologyRelationKind::HalfEdgeEndsAtVertex.kind_name(),
                )
                .aspect("topology.source_identity", half_edge_identity)
                .aspect("topology.target_identity", target_vertex_identity)
        })
        .expect_err("relation updates must stay fail-closed until the runtime can preserve authoritative relation identity");

    let message = error.to_string();
    assert!(message.contains("does not admit `update_existing` write command yet"));

    let relation_rows_after = workspace.read(assembly.relations());
    let original_relation = query_relation_row_by_kind_and_source(
        &relation_rows_after,
        WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
        &query_entity_identity_by_name(&entity_rows, half_edge_name),
    );
    assert_eq!(
        query_relation_id_from_row(original_relation),
        query_relation_id_from_row(relation)
    );
}

fn query_entity_identity_by_name(
    rows: &[forge_query::facade::ForgeQueryEntity],
    name: &str,
) -> String {
    rows.iter()
        .find(|row| row_matches_name(row, name))
        .map(|row| row.identity.clone())
        .expect("query rows should contain requested persistent name")
}

fn query_relation_row_by_kind_and_source<'a>(
    rows: &'a [forge_query::facade::ForgeQueryEntity],
    kind: WorthTopologyRelationKind,
    source_identity: &str,
) -> &'a forge_query::facade::ForgeQueryEntity {
    rows.iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == kind.kind_name())
                && row
                    .payload
                    .get("topology")
                    .and_then(|value| value.get("source_identity"))
                    .and_then(|value| value.as_str())
                    .is_some_and(|identity| identity == source_identity)
        })
        .expect("query rows should contain requested topology relation")
}

fn query_relation_id_from_row(row: &forge_query::facade::ForgeQueryEntity) -> RelationId {
    serde_json::from_value(row.payload["lineage"]["provenance"].clone())
        .expect("query relation provenance should decode")
}

fn row_matches_name(row: &forge_query::facade::ForgeQueryEntity, name: &str) -> bool {
    row.payload
        .get("naming")
        .and_then(|value| value.get("persistent_name"))
        .and_then(|value| value.as_str())
        .is_some_and(|persistent_name| persistent_name == name)
        || row
            .payload
            .get("topology")
            .and_then(|value| value.get("structure"))
            .and_then(|value| value.as_str())
            .is_some_and(|structure| structure == name)
}
