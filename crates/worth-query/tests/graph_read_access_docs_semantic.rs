use std::collections::BTreeSet;

use worth_query::facade::runtime::WorthQueryGraphReadAccessRequirementKind;

mod support;

use support::graph_index_inventory::runtime_profiles::default_graph_support_workspace;
use support::graph_read_access::read_surface_declarations::graph_access_family;

const GRAPH_READ_ACCESS_DOC: &str = include_str!("../docs/authoring/graph-read-access-planning.md");
const AI_README: &str = include_str!("../docs/AI_README.md");

#[test]
fn documented_explicit_access_plan_flow_executes_and_receipts_the_same_plan() {
    let mut workspace = default_graph_support_workspace("graph-read-access.docs.semantic-flow");
    let family = graph_access_family(&mut workspace, "docs-semantic-flow");
    let access_plan = workspace
        .read_family_intent(&family)
        .review()
        .expect("documented family should be reviewable")
        .graph_read_access_plan()
        .expect("documented family should expose an admitted access plan");
    let plan_digest = access_plan.digest().to_string();
    let requirement_kinds = access_plan
        .admission()
        .requirement_set()
        .rows()
        .iter()
        .map(|row| row.kind().as_str())
        .collect::<BTreeSet<_>>();

    let result = workspace
        .execute_read_family_with_access_plan(&family, access_plan)
        .expect("documented explicit access-plan flow should execute");
    let consumption = result
        .receipt()
        .graph_read_access_plan_consumption()
        .expect("documented flow should attach access-plan consumption");
    let counters = consumption.execution_counters();

    assert_eq!(consumption.plan_digest(), plan_digest);
    assert_eq!(counters.executor_entry_count(), 1);
    assert_eq!(counters.per_result_neighbor_lookup_count(), 0);
    assert!(counters.materialized_row_count() > 0);
    assert_requirement_kinds_are_documented(&requirement_kinds);
    assert!(GRAPH_READ_ACCESS_DOC.contains("execute_read_family_with_access_plan"));
    assert!(GRAPH_READ_ACCESS_DOC.contains("graph_read_access_plan_consumption"));
    assert!(!GRAPH_READ_ACCESS_DOC.contains(".unwrap()"));
}

fn assert_requirement_kinds_are_documented(requirement_kinds: &BTreeSet<&str>) {
    assert!(requirement_kinds
        .contains(WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency.as_str()));
    assert!(requirement_kinds
        .contains(WorthQueryGraphReadAccessRequirementKind::TraversalWorkset.as_str()));
    for requirement_kind in requirement_kinds {
        assert!(
            GRAPH_READ_ACCESS_DOC.contains(requirement_kind),
            "graph read access docs should list requirement `{requirement_kind}`"
        );
        assert!(
            AI_README.contains(requirement_kind),
            "AI_README should orient agents to requirement `{requirement_kind}`"
        );
    }
}
