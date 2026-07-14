use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::basis_lifecycle;
use worth_query::facade::{comparison, history, inspection, live, read};

use super::ordinary_query::{
    declare_measurement_comparison, declare_measurement_history, declare_measurement_live,
    declare_measurement_read, inspect_measurement_read,
};

#[test]
fn worth_ui_read_and_inspection_use_query_owned_outcomes() {
    let mut workspace = measurement_workspace("worth-ui-reference-read");
    let outcome = declare_measurement_read()
        .expect("Worth UI read declaration should admit")
        .using(read::current())
        .run(&mut workspace);
    let completion = outcome.completed().expect("Worth UI read should complete");
    assert_eq!(completion.journey_counters().planning_attempt_count(), 1);
    assert_eq!(
        completion
            .journey_counters()
            .lower_runtime_execution_completed_count(),
        1
    );

    let basis = basis_lifecycle()
        .historical_snapshot("worth-ui-reference-inspection", true)
        .inspect()
        .expect("inspection basis should admit");
    let inspected = inspect_measurement_read(completion)
        .using(inspection::inspection_basis(basis))
        .run(&workspace);
    let inspected = inspected.settled().expect("Worth UI inspection should settle");
    assert!(inspected.materialization().is_some());
    assert_eq!(inspected.counters().materialization_attempt_count(), 1);
}

#[test]
fn worth_ui_managed_live_owns_activation_delivery_and_disposal() {
    let mut workspace = measurement_workspace("worth-ui-reference-live");
    let opened = declare_measurement_live()
        .expect("Worth UI live declaration should admit")
        .using(live::current())
        .open(&mut workspace);
    let handle = match opened {
        live::WorthQueryLiveOpenOutcome::Opened(completion) => completion.into_handle(),
        live::WorthQueryLiveOpenOutcome::Stopped(stop) => {
            panic!("Worth UI live open stopped: {:?}", stop.source())
        }
    };
    assert_eq!(
        handle
            .observe(&mut workspace)
            .expect("observe")
            .activation_work()
            .declaration_count(),
        1
    );
    match handle.close(&mut workspace) {
        live::WorthQueryManagedLiveCloseOutcome::Closed(receipt) => {
            assert!(receipt.lane_terminal());
            assert_eq!(receipt.disposal_work().lifecycle_closeout_count(), 1);
        }
        live::WorthQueryManagedLiveCloseOutcome::Stopped(stop) => {
            panic!("Worth UI live close stopped: {:?}", stop.error())
        }
    }
}

#[test]
fn worth_ui_history_and_comparison_bind_bases_structurally() {
    let mut historical = measurement_workspace("worth-ui-reference-history");
    let historical_context = history::at(&historical);
    let historical_outcome = declare_measurement_history()
        .expect("Worth UI history declaration should admit")
        .using(historical_context)
        .run(&mut historical);
    assert!(historical_outcome.completed().is_some());

    let mut left = measurement_workspace("worth-ui-reference-left");
    let mut right = measurement_workspace("worth-ui-reference-right");
    let context = comparison::between(
        &left,
        comparison::WorthQuerySessionLabel::scoped_strs("worth-ui", ["left"])
            .expect("left label should admit"),
        &right,
        comparison::WorthQuerySessionLabel::scoped_strs("worth-ui", ["right"])
            .expect("right label should admit"),
    )
    .expect("comparison basis pair should admit");
    let compared = declare_measurement_comparison()
        .expect("Worth UI comparison declaration should admit")
        .using(context)
        .run((&mut left, &mut right));
    assert_eq!(
        compared.completed().expect("comparison should complete").change(),
        comparison::WorthQueryComparisonChange::Unchanged
    );
}

fn measurement_workspace(name: &str) -> worth_query::facade::runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should build")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect should build");
    in_memory_test_runtime()
        .with_schema(schema)
        .workspace(name)
        .expect("Worth UI reference workspace should build")
}
