const WORKSPACE_OVERVIEW_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/workspace-overview.md"
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
