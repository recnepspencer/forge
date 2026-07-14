use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::{domain, read};

use super::{declare_candidate_promotion, declare_candidate_search};

#[test]
fn hadwiger_candidate_search_uses_the_ordinary_read_journey() {
    let mut workspace = candidate_workspace("hadwiger-reference-read");
    let outcome = declare_candidate_search()
        .expect("Hadwiger candidate search should declare")
        .using(read::current())
        .run(&mut workspace);
    let completion = outcome
        .completed()
        .expect("candidate search should complete");

    assert_eq!(completion.journey_counters().planning_attempt_count(), 1);
    assert_eq!(
        completion
            .journey_counters()
            .lower_runtime_execution_completed_count(),
        1
    );
}

#[test]
fn hadwiger_contribution_lowers_through_query_owned_workflow() {
    let mut workspace = candidate_workspace("hadwiger-reference-workflow");
    let label = domain::WorthQuerySessionLabel::scoped_strs("hadwiger", ["candidate-17"])
        .expect("candidate label should admit");
    let declaration = declare_candidate_promotion(label.clone(), "candidate-17")
        .expect("Hadwiger contribution should declare");
    let context = domain::preview(&workspace, label).expect("preview context should admit");
    let outcome = declaration.using(context).run(&mut workspace);
    let completion = outcome
        .completed()
        .expect("Hadwiger workflow should complete");

    assert_eq!(
        completion.workflow().aftermath().closeout_kind(),
        domain::WorthQueryPreviewCloseoutKind::Promoted
    );
    assert_eq!(
        completion
            .workflow()
            .counters()
            .lower_runtime_execution_completed_count(),
        1
    );
}

fn candidate_workspace(name: &str) -> worth_query::facade::runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should build")
        .aspect("colorability.lower_bound", "colorability.lower_bound")
        .expect("colorability aspect should build");
    in_memory_test_runtime()
        .with_schema(schema)
        .workspace(name)
        .expect("Hadwiger reference workspace should build")
}
