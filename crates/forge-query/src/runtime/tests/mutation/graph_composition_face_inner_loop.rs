use super::super::support::*;

fn face_loop_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([
            ForgeQueryCollection::new(
                "Loop",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("kind.value", "kind.value"),
                ],
            ),
            ForgeQueryCollection::new(
                "HalfEdge",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("kind.value", "kind.value"),
                ],
            ),
            ForgeQueryCollection::new(
                "FaceLoopRelation",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("face.id", "face.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("loop.id", "loop.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("role.value", "role.value"),
                ],
            ),
            ForgeQueryCollection::new(
                "LoopHalfEdgeRelation",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("loop.id", "loop.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("half_edge.id", "half_edge.id"),
                    crate::memory_workspace::ForgeQueryAspect::new(
                        "position.ordinal",
                        "position.ordinal",
                    ),
                ],
            ),
        ])
        .build()
        .expect("runtime should build")
}

#[test]
fn compose_graph_supports_face_inner_loop_insertion_with_full_resolution_map() {
    let mut workspace = face_loop_runtime()
        .workspace("topology.graph-composition-face-inner-loop")
        .expect("workspace should open");
    let loops: ForgeQueryLiveView<Value> = workspace
        .live_view("topology.face-inner-loop-loops", |q| {
            q.from("Loop")
                .select(["identity.id", "kind.value"])
                .order_by("identity.id")
                .schema_basis("topology-face-inner-loop-loops")
        })
        .expect("loop live view should declare");
    let half_edges: ForgeQueryLiveView<Value> = workspace
        .live_view("topology.face-inner-loop-half-edges", |q| {
            q.from("HalfEdge")
                .select(["identity.id", "kind.value"])
                .order_by("identity.id")
                .schema_basis("topology-face-inner-loop-half-edges")
        })
        .expect("half-edge live view should declare");
    let face_loops: ForgeQueryLiveView<Value> = workspace
        .live_view("topology.face-inner-loop-face-loops", |q| {
            q.from("FaceLoopRelation")
                .select(["face.id", "loop.id", "role.value"])
                .order_by("face.id")
                .schema_basis("topology-face-inner-loop-face-loops")
        })
        .expect("face-loop live view should declare");
    let loop_edges: ForgeQueryLiveView<Value> = workspace
        .live_view("topology.face-inner-loop-loop-edges", |q| {
            q.from("LoopHalfEdgeRelation")
                .select(["loop.id", "half_edge.id", "position.ordinal"])
                .order_by("position.ordinal")
                .schema_basis("topology-face-inner-loop-loop-edges")
        })
        .expect("loop-edge live view should declare");

    let receipt = workspace
        .compose_graph(|graph| {
            let inner_loop = graph.insert_entity("draft-loop", "Loop", |loop_entity| {
                loop_entity
                    .aspect("identity.id", "loop-inner-1")
                    .aspect("kind.value", "inner")
            })?;
            let first_half_edge =
                graph.insert_entity("draft-half-edge-a", "HalfEdge", |half_edge| {
                    half_edge
                        .aspect("identity.id", "he-inner-1")
                        .aspect("kind.value", "half_edge")
                })?;
            let second_half_edge =
                graph.insert_entity("draft-half-edge-b", "HalfEdge", |half_edge| {
                    half_edge
                        .aspect("identity.id", "he-inner-2")
                        .aspect("kind.value", "half_edge")
                })?;
            graph.insert_relation("FaceLoopRelation", |relation| {
                relation
                    .existing_entity_identity("face.id", "face-1")
                    .symbolic_entity_identity("loop.id", &inner_loop)
                    .aspect("role.value", "inner")
            })?;
            graph.insert_relation("LoopHalfEdgeRelation", |relation| {
                relation
                    .symbolic_entity_identity("loop.id", &inner_loop)
                    .symbolic_entity_identity("half_edge.id", &first_half_edge)
                    .aspect("position.ordinal", "0")
            })?;
            graph.insert_relation("LoopHalfEdgeRelation", |relation| {
                relation
                    .symbolic_entity_identity("loop.id", &inner_loop)
                    .symbolic_entity_identity("half_edge.id", &second_half_edge)
                    .aspect("position.ordinal", "1")
            })?;
            Ok(())
        })
        .expect("face inner loop insertion should execute");
    let loop_identity = receipt.write_receipts()[0].deltas()[0]
        .entity_identity
        .clone();
    let first_half_edge_identity = receipt.write_receipts()[1].deltas()[0]
        .entity_identity
        .clone();
    let second_half_edge_identity = receipt.write_receipts()[2].deltas()[0]
        .entity_identity
        .clone();

    let program = receipt
        .graph_composition_program()
        .expect("graph composition receipt should expose program");
    let lifecycle = receipt
        .graph_composition_lifecycle_outcomes()
        .expect("graph composition receipt should expose lifecycle");
    let evidence = receipt
        .graph_composition_evidence()
        .expect("graph composition receipt should expose evidence");
    let resolution_map = receipt.graph_composition_resolution_map();

    assert_eq!(program.component_count(), 6);
    assert_eq!(receipt.graph_composition_breadth().component_count(), 6);
    assert_eq!(
        receipt
            .graph_composition_breadth()
            .symbolic_entity_declaration_count(),
        3
    );
    assert_eq!(
        receipt
            .graph_composition_breadth()
            .symbolic_relation_declaration_count(),
        3
    );
    assert_eq!(
        receipt
            .batch_mutation_evidence()
            .symbolic_resolution_count(),
        5
    );
    assert_eq!(evidence.symbolic_resolution_count(), 5);
    assert_eq!(
        lifecycle.counter_snapshot(),
        "created=6;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(
        evidence.counter_snapshot(),
        "components=6;symbolic_entities=3;symbolic_relations=3;symbolic_resolutions=5;affected_live_views=4;affected_derived_views=0;considered_computed_views=0;created=6;updated_identity_preserved=0;retargeted_identity_preserved=0;retired_current_truth=0;superseded_with_lineage=0;deleted_if_uncommitted=0;denied_before_execution=0"
    );
    assert_eq!(resolution_map.len(), 5);
    assert_graph_composition_resolution_snapshot(
        &resolution_map,
        &vec![
            (
                3,
                Some("loop.id".to_string()),
                "draft-loop".to_string(),
                loop_identity.to_string(),
            ),
            (
                4,
                Some("loop.id".to_string()),
                "draft-loop".to_string(),
                loop_identity.to_string(),
            ),
            (
                4,
                Some("half_edge.id".to_string()),
                "draft-half-edge-a".to_string(),
                first_half_edge_identity.to_string(),
            ),
            (
                5,
                Some("loop.id".to_string()),
                "draft-loop".to_string(),
                loop_identity.to_string(),
            ),
            (
                5,
                Some("half_edge.id".to_string()),
                "draft-half-edge-b".to_string(),
                second_half_edge_identity.to_string(),
            ),
        ],
    );
    assert_eq!(
        program.steps()[0].kind(),
        ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
    );
    assert_eq!(
        program.steps()[3].kind(),
        ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
    );
    let loop_rows = workspace.read(&loops);
    let half_edge_rows = workspace.read(&half_edges);
    let face_loop_rows = workspace.read(&face_loops);
    let loop_edge_rows = workspace.read(&loop_edges);
    assert_eq!(loop_rows.len(), 1);
    assert_eq!(half_edge_rows.len(), 2);
    assert_eq!(
        loop_rows[0].payload["identity"]["id"].as_str(),
        Some("loop-inner-1")
    );
    assert_eq!(
        half_edge_rows[0].payload["identity"]["id"].as_str(),
        Some("he-inner-1")
    );
    assert_eq!(
        half_edge_rows[1].payload["identity"]["id"].as_str(),
        Some("he-inner-2")
    );
    assert_eq!(face_loop_rows.len(), 1);
    assert_eq!(
        face_loop_rows[0].payload["loop"]["id"].as_str(),
        Some(loop_identity.as_str())
    );
    assert_eq!(
        face_loop_rows[0].payload["face"]["id"].as_str(),
        Some("face-1")
    );
    assert_eq!(loop_edge_rows.len(), 2);
    assert_eq!(
        loop_edge_rows[0].payload["half_edge"]["id"].as_str(),
        Some(first_half_edge_identity.as_str())
    );
    assert_eq!(
        loop_edge_rows[1].payload["half_edge"]["id"].as_str(),
        Some(second_half_edge_identity.as_str())
    );

    match workspace.inspect(&receipt).expect("receipt should inspect") {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .graph_composition_evidence()
                    .expect("inspection should expose graph composition evidence")
                    .symbolic_resolution_count(),
                5
            );
            assert_eq!(
                inspection
                    .graph_composition_resolution_map()
                    .entries()
                    .len(),
                5
            );
            assert_graph_composition_resolution_maps_match(
                &inspection.graph_composition_resolution_map(),
                &resolution_map,
            );
            assert_eq!(
                inspection.component_operations()[3]
                    .symbolic_aspect_resolution_evidence()
                    .len(),
                1
            );
            assert_eq!(
                inspection.component_operations()[4]
                    .symbolic_aspect_resolution_evidence()
                    .len(),
                2
            );
            assert_eq!(
                inspection.component_operations()[5]
                    .symbolic_aspect_resolution_evidence()
                    .len(),
                2
            );
            assert_eq!(
                inspection.component_operations()[4].symbolic_aspect_resolution_evidence()[1]
                    .resolved_entity_identity(),
                first_half_edge_identity.as_str()
            );
            assert_eq!(
                inspection.component_operations()[5].symbolic_aspect_resolution_evidence()[1]
                    .resolved_entity_identity(),
                second_half_edge_identity.as_str()
            );
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
}
