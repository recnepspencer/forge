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
    "/../../_docs/forge-query/runtime-generic-graph-authoring-plan.md"
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
        "`runtime.intent(declaration).execute()`",
        "`runtime.intent(declaration).review()?.admit()?.execute()`",
        "`runtime.next_effect_write_intent(&effect, version, contract).execute()`",
        "`runtime.next_effect_write_intent(&effect, version, contract).review()?.admit()?.execute()`",
        "`runtime.write_intent(command).execute()`",
        "`runtime.write_intent(command).review()?.admit()?.execute()`",
        "`workspace.write_intent(command).execute()`",
        "`workspace.write_intent(command).review()?.admit()?.execute()`",
        "`runtime.write_batch_intent(commands).execute()`",
        "`runtime.write_batch_intent(commands).review()?.admit()?.execute()`",
        "`workspace.write_batch_intent(commands).execute()`",
        "`workspace.write_batch_intent(commands).review()?.admit()?.execute()`",
        "`workspace.read_family_intent(&family).execute()`",
        "`workspace.read_family_intent(&family).review()?.admit()?.execute()`",
        "`workspace.read_family_in_basis_context_intent(&family, &context).execute()`",
        "`workspace.read_family_in_basis_context_intent(&family, &context).review()?.admit()?.execute()`",
        "`workspace.read_live_intent(&view).execute()`",
        "`workspace.read_live_intent(&view).review()?.admit()?.execute()`",
        "`workspace.read(&view)`",
        "`workspace.materialize_intent(&view).execute()`",
        "`workspace.materialize_intent(&view).review()?.admit()?.execute()`",
        "`workspace.inspect_intent(target).execute()`",
        "`workspace.inspect_intent(target).review()?.admit()?.execute()`",
        "`workspace.inspect_derived_intent(&view).execute()`",
        "`workspace.inspect_derived_intent(&view).review()?.admit()?.execute()`",
        "`runtime.probe_existing_intent(request).execute()`",
        "`runtime.probe_existing_intent(request).review()?.admit()?.execute()`",
        "`workspace.probe_existing_intent(request).execute()`",
        "`workspace.probe_existing_intent(request).review()?.admit()?.execute()`",
        "`workspace.materialize(&view)`",
        "`forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)?.admit()?.scope()`",
        "`forge_query_projection_consumption_intent(declaration)?.admit()?.bind_contract()`",
        "`ForgeQueryIntentDecisionTraceEnvelope`",
        "`consumer_inspection()`",
    ] {
        assert!(
            INTENT_ADMISSION_DOC.contains(required),
            "intent admission doc must include {required}"
        );
    }
}

#[test]
fn intent_admission_doc_names_deferred_neighbors_honestly() {
    assert!(
        INTENT_ADMISSION_DOC.contains("remain explicitly deferred"),
        "intent admission doc must state that deferred neighbors are still deferred"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("still support-gated"),
        "intent admission doc must preserve the support-gated posture"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("basis observation"),
        "intent admission doc must name basis observation as a covered family"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("projection consumption"),
        "intent admission doc must name projection consumption as a covered family"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("read-family execution"),
        "intent admission doc must teach read execution as a covered family"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("derived materialization"),
        "intent admission doc must teach derived inspection-materialization as a covered family"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("existing-truth probe routing"),
        "intent admission doc must teach lower-runtime capability routing as a covered family"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("graph-composition verified-existing lanes"),
        "intent admission doc must teach verified-existing work through graph-composition lanes"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("generic and derived inspection"),
        "intent admission doc must teach generic inspection as a covered family"
    );
    assert!(
        INTENT_ADMISSION_DOC.contains("Generic materialization neighbors"),
        "intent admission doc must keep the generic materialization neighbors deferred explicitly"
    );
}
