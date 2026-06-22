use super::super::support::*;

#[test]
fn compose_graph_without_lineage_steps_fails_closed_on_lineage_summary() {
    let mut workspace = stateful_bridge_vertex_runtime()
        .workspace("topology.graph-composition-no-lineage-summary")
        .expect("workspace should open");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view(
            "topology.graph-composition-no-lineage-summary-vertices",
            |q| {
                q.from("Vertex")
                    .select([
                        crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                        crate::authoring::AspectFieldKey::new("kind", "value").unwrap(),
                    ])
                    .order_by(crate::authoring::AspectFieldKey::new("identity", "id").unwrap())
                    .schema_basis("topology-graph-composition-no-lineage-summary-vertices")
            },
        )
        .expect("vertex live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            graph.insert_entity("vertex-only", "Vertex", |vertex| {
                vertex
                    .aspect(
                        test_aspect_touch("identity.id"),
                        test_string_aspect_value("vertex-only"),
                    )
                    .aspect(
                        test_aspect_touch("kind.value"),
                        test_string_aspect_value("plain"),
                    )
            })?;
            Ok(())
        })
        .expect("simple composition should execute");

    assert!(receipt.graph_composition_lineage_summary().is_none());
    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert!(inspection.graph_composition_lineage_summary().is_none());
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}
