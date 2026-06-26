use super::super::support::*;

#[test]
fn equivalent_touch_fronts_preserve_same_obligation_authority_projection() {
    let mut scalar_runtime = runtime_with_selection_foundation_collection_obligation("Task");
    let scalar_receipt = scalar_runtime
        .write(task_insert_command("front-scalar"))
        .expect("scalar write should execute");
    let scalar_projection = scalar_receipt
        .obligation_dispatch()
        .expect("scalar write should carry obligation dispatch")
        .evidence_projection();

    let mut batch_runtime = runtime_with_selection_foundation_collection_obligation("Task");
    let batch_receipt = batch_runtime
        .write_batch(vec![task_insert_command("front-batch")])
        .expect("batch write should execute");
    let batch_projection = batch_receipt
        .obligation_dispatch()
        .expect("batch write should carry obligation dispatch")
        .evidence_projection();

    let mut graph_runtime = runtime_with_selection_foundation_collection_obligation("Task");
    let (commands, breadth, program) = single_task_graph_program("front-graph");
    let graph_receipt = graph_runtime
        .write_graph_batch(commands, breadth, program)
        .expect("graph write should execute");
    let graph_projection = graph_receipt
        .obligation_dispatch()
        .expect("graph write should carry obligation dispatch")
        .evidence_projection();

    assert_eq!(
        scalar_projection.context_kind(),
        Some(ForgeQueryGraphObligationDispatchContextKind::ScalarMutation)
    );
    assert_eq!(
        batch_projection.context_kind(),
        Some(ForgeQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch)
    );
    assert_eq!(
        graph_projection.context_kind(),
        Some(ForgeQueryGraphObligationDispatchContextKind::GraphComposition)
    );
    assert_eq!(
        dispatch_authority_signature(&scalar_projection),
        dispatch_authority_signature(&batch_projection)
    );
    assert_eq!(
        dispatch_authority_signature(&scalar_projection),
        dispatch_authority_signature(&graph_projection)
    );
}

fn runtime_with_selection_foundation_collection_obligation(collection: &str) -> ForgeQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(
            ForgeQueryGraphObligationRegistration::schema_contract_validator(
                ForgeQueryGraphObligationRuleIdentity::new(
                    "test.graph-obligation-dispatch-equivalence",
                    collection,
                    "v1",
                )
                .unwrap(),
                ForgeQueryGraphTouchSelector::collection(collection).unwrap(),
                ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
            ),
        )
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with selection-foundation graph obligation")
}

fn single_task_graph_program(
    id: &str,
) -> (
    Vec<ForgeQueryWriteCommand>,
    ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram,
) {
    let mut graph = ForgeQueryGraphCompositionBuilder::new();
    graph
        .insert_entity("task", "Task", |entity| {
            entity
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value(id),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Graph task"),
                )
        })
        .unwrap();
    graph.finish().unwrap()
}

fn task_insert_command(id: &str) -> ForgeQueryWriteCommand {
    ForgeQueryWriteCommand::InsertAspects {
        collection: crate::runtime::ForgeQueryMutationTargetCollectionIdentity::new(
            "write-command-declared",
            "Task",
        ),
        aspects: vec![
            ForgeQueryAdmittedAspectValue::new(
                test_aspect_touch("identity.id"),
                test_string_aspect_value(id),
            )
            .unwrap(),
            ForgeQueryAdmittedAspectValue::new(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Projected task"),
            )
            .unwrap(),
        ],
        symbolic_aspect_references: Vec::new(),
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
        symbolic_target_reference: None,
    }
}

fn dispatch_authority_signature(
    projection: &ForgeQueryAuthoritativeMutationObligationDispatchProjection,
) -> Vec<(
    String,
    String,
    String,
    ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationStateAccessPolicy,
)> {
    let mut rows = projection
        .rows()
        .iter()
        .map(|row| {
            (
                row.rule_namespace().to_string(),
                row.rule_name().to_string(),
                row.rule_semantic_version().to_string(),
                row.obligation_kind(),
                row.support_lane(),
                row.state_access_policy(),
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}
