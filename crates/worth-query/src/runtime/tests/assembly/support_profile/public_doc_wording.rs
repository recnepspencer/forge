const WORKSPACE_OVERVIEW_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/foundations/workspace-overview.md"
));

const INTENT_ADMISSION_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/execution/intent-admission.md"
));

const GRAPH_AUTHORING_PLAN_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/worth-query/runtime-generic-graph-authoring-plan.md"
));

#[test]
fn workspace_overview_uses_post_deletion_runtime_wording() {
    assert!(
        WORKSPACE_OVERVIEW_DOC.contains("`workspace.public_mutation_surface_report()`"),
        "workspace overview must point callers at the surviving mutation-surface contract"
    );
    assert!(
        !WORKSPACE_OVERVIEW_DOC.contains("Compatibility entry points still exist"),
        "workspace overview must not teach a deleted compatibility-entrypoint story"
    );
}

#[test]
fn graph_authoring_plan_names_deleted_and_lower_level_residue_honestly() {
    assert!(
        GRAPH_AUTHORING_PLAN_DOC.contains("deleted builder-shaped mutation seams"),
        "graph authoring plan must name deleted builder-shaped seams explicitly"
    );
    assert!(
        !GRAPH_AUTHORING_PLAN_DOC.contains("compatibility or deprecated mutation seams"),
        "graph authoring plan must not preserve the weaker compatibility/deprecation framing"
    );
}

#[test]
fn intent_admission_doc_teaches_the_runtime_floor_with_final_public_names() {
    for required in [
        "`runtime.intent(declaration)`",
        "`runtime.next_effect_write_intent(...)`",
        "`runtime.write_intent(command)`",
        "`runtime.write_batch_intent(commands)`",
        "`workspace.read_family_intent(&family)`",
        "`workspace.read_family_in_basis_context_intent(&family, &context)`",
        "`workspace.read_live_intent(&view)`",
        "`workspace.materialize_intent(&view)`",
        "`workspace.inspect_intent(target)`",
        "`workspace.inspect_derived_intent(&view)`",
        "`runtime.probe_existing_intent(request)`",
        "`workspace.probe_existing_intent(request)`",
        "completion.consume_projection(read::project_facts()...)",
        ".consume_projection(read::project_facts().entity_identities())",
        "`decision_trace_envelope()`",
        "`consumer_inspection()`",
    ] {
        assert!(
            INTENT_ADMISSION_DOC.contains(required),
            "intent admission doc must include {required}"
        );
    }
    assert!(INTENT_ADMISSION_DOC.contains("The ordinary path is `.execute()`"));
    assert!(INTENT_ADMISSION_DOC.contains("`.review()?.admit()?.execute()`"));
}

#[test]
fn intent_admission_doc_names_deferred_neighbors_honestly() {
    assert!(
        INTENT_ADMISSION_DOC.contains("Coverage is family-specific"),
        "intent admission doc must make support admission family-specific"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("Check the support matrix"),
        "intent admission doc must route support questions to the support matrix"
    );
    assert!(
        INTENT_ADMISSION_DOC
            .contains("Store-backed replay, durable restart, and neighboring materialization"),
        "intent admission doc must name the deferred capability neighbors"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("remain deferred where the support matrix says so"),
        "intent admission doc must preserve the deferred support posture"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("it is not ordinary execution DX"),
        "intent admission doc must keep certification outside ordinary execution"
    );
}
