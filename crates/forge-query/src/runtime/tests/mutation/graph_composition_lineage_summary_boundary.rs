use super::super::support::*;

#[test]
fn compose_graph_without_lineage_steps_fails_closed_on_lineage_summary() {
    let mut workspace = ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([ForgeQueryCollection::new(
            "Vertex",
            [
                crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                crate::memory_workspace::ForgeQueryAspect::new("kind.value", "kind.value"),
            ],
        )])
        .build()
        .expect("runtime should build")
        .workspace("topology.graph-composition-no-lineage-summary")
        .expect("workspace should open");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view(
            "topology.graph-composition-no-lineage-summary-vertices",
            |q| {
                q.from("Vertex")
                    .select(["identity.id", "kind.value"])
                    .order_by("identity.id")
                    .schema_basis("topology-graph-composition-no-lineage-summary-vertices")
            },
        )
        .expect("vertex live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            graph.insert_entity("vertex-only", "Vertex", |vertex| {
                vertex
                    .aspect("identity.id", "vertex-only")
                    .aspect("kind.value", "plain")
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
