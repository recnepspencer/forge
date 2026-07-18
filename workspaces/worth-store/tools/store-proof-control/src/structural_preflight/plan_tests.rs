use std::path::Path;

use worth_store_test_support::structural_preflight::{
    StructuralPredicate, StructuralPreflightProfile, StructuralPreflightRequest,
};

#[test]
fn complete_preflight_plan_binds_tools_sources_and_dependency_flow() {
    let forge_root = super::forge_root(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let request = StructuralPreflightRequest::new(
        StructuralPreflightProfile::Complete,
        vec![
            StructuralPredicate::Boundary,
            StructuralPredicate::AgentContext,
            StructuralPredicate::Dependency,
        ],
    )
    .unwrap();

    let plan = super::plan::build(&forge_root, request).unwrap();
    let boundary = predicate(&plan.predicates, StructuralPredicate::Boundary);
    let agent_context = predicate(&plan.predicates, StructuralPredicate::AgentContext);
    let dependency = predicate(&plan.predicates, StructuralPredicate::Dependency);
    let boundary_tool = boundary.tool.as_ref().unwrap();
    let agent_context_tool = agent_context.tool.as_ref().unwrap();
    let boundary_tool_is_locked_offline = required_cargo_posture(&boundary_tool.arguments);
    let agent_context_tool_is_locked_offline =
        required_cargo_posture(&agent_context_tool.arguments);
    let boundary_tool_binds_rustc = boundary_tool
        .supporting_tools
        .iter()
        .any(|tool| tool.purpose == "rustc");
    let all_input_scopes_are_bound = plan
        .predicates
        .iter()
        .flat_map(|predicate| &predicate.input_scopes)
        .all(|scope| !scope.input_identity.is_empty());
    let dependency_scope_binds_store_lockfile = dependency.input_scopes.iter().any(|scope| {
        scope
            .source_paths
            .iter()
            .any(|path| path.ends_with("Cargo.lock"))
    });
    let dependency_is_metadata_owned = dependency.tool.is_none();

    assert!(boundary_tool_is_locked_offline);
    assert!(agent_context_tool_is_locked_offline);
    assert!(boundary_tool_binds_rustc);
    assert!(all_input_scopes_are_bound);
    assert!(dependency_scope_binds_store_lockfile);
    assert!(dependency_is_metadata_owned);
}

fn predicate(
    predicates: &[worth_store_test_support::structural_preflight::StructuralPredicatePlan],
    expected: StructuralPredicate,
) -> &worth_store_test_support::structural_preflight::StructuralPredicatePlan {
    predicates
        .iter()
        .find(|predicate| predicate.predicate == expected)
        .unwrap()
}

fn required_cargo_posture(arguments: &[String]) -> bool {
    ["--offline", "--locked"]
        .iter()
        .all(|required| arguments.iter().any(|argument| argument == required))
}
