use super::super::super::support::*;
use super::fixtures::local_identity_read;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::ordinary::comparison::{
    current_and_retained, declare, WorthQueryComparisonChange,
    WorthQueryComparisonCorrespondencePosture, WorthQueryComparisonNextAction,
    WorthQueryComparisonStopSource,
};
use crate::runtime::tests::support::{
    insert_command, stateful_bridge_task_runtime, test_string_aspect_value,
};

#[test]
fn current_and_retained_diff_is_unchanged_on_the_same_truth_basis() {
    let declaration = declare(local_identity_read)
        .expect("comparison declaration should build")
        .diff();
    let mut workspace = read_runtime()
        .workspace("ordinary-comparison-diff")
        .expect("workspace should open");
    let context = current_and_retained(&workspace);
    let outcome = declaration.using(context).run(&mut workspace);
    let completion = outcome.completed().expect("comparison should complete");

    assert_eq!(completion.change(), WorthQueryComparisonChange::Unchanged);
    assert_eq!(completion.left().rows(), completion.right().rows());
    assert_eq!(
        completion
            .right_materialization()
            .resolved_path_class()
            .as_str(),
        "resolved_retained_snapshot_path"
    );
    assert_eq!(
        completion
            .journey_counters()
            .historical_execution_attempt_count(),
        1
    );
}

#[test]
fn structural_ambiguity_remains_advisory_and_exposes_no_partial_completion() {
    let declaration = declare(task_collection_read)
        .expect("comparison declaration should build")
        .correspondence(4);
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-comparison-ambiguous")
        .expect("workspace should open");
    write_task(&mut workspace, "first");
    write_task(&mut workspace, "second");
    let context = current_and_retained(&workspace);
    let outcome = declaration.using(context).run(&mut workspace);

    assert!(outcome.completed().is_none());
    assert!(outcome.stop().is_none());
    let correspondence = outcome
        .correspondence()
        .expect("ambiguity must remain correspondence evidence");
    assert_eq!(
        correspondence.posture(),
        WorthQueryComparisonCorrespondencePosture::Advisory
    );
    let ambiguity = correspondence
        .correspondence()
        .outcome()
        .as_advisory_structural_ambiguous()
        .expect("two derived candidates should be ambiguous");
    assert_eq!(ambiguity.candidate_set().len(), 2);
}

#[test]
fn exact_single_row_identity_can_prove_lineage_without_raw_consumer_ids() {
    let declaration = declare(task_collection_read)
        .expect("comparison declaration should build")
        .lineage();
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-comparison-lineage")
        .expect("workspace should open");
    write_task(&mut workspace, "only");
    let context = current_and_retained(&workspace);
    let outcome = declaration.using(context).run(&mut workspace);
    let correspondence = outcome.correspondence().expect("lineage should resolve");

    assert_eq!(
        correspondence.posture(),
        WorthQueryComparisonCorrespondencePosture::AuthoritativeContinuity
    );
    assert!(correspondence
        .correspondence()
        .outcome()
        .as_lineage_continuity()
        .is_some());
}

#[test]
fn stale_structural_pair_denies_before_either_query_executes() {
    let declaration = declare(task_collection_read)
        .expect("comparison declaration should build")
        .diff();
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("ordinary-comparison-stale")
        .expect("workspace should open");
    let context = current_and_retained(&workspace);
    write_task(&mut workspace, "changed");
    let outcome = declaration.using(context).run(&mut workspace);
    let stop = outcome.stop().expect("stale pair must stop");

    assert_eq!(
        stop.source(),
        WorthQueryComparisonStopSource::StaleBasisPair
    );
    assert_eq!(
        stop.next_action(),
        WorthQueryComparisonNextAction::RefreshBasisPair
    );
    assert_eq!(stop.journey_counters().current_execution_attempt_count(), 0);
    assert_eq!(
        stop.journey_counters().historical_execution_attempt_count(),
        0
    );
}

fn task_collection_read<Output>(
    read: crate::runtime::WorthQueryReadBuilder<Output>,
) -> Result<Output, crate::runtime::WorthQueryReadDenial> {
    read.local_collection(
        "Task",
        manager_schema(),
        |query| {
            query.project(
                AspectFieldSelector::new("identity", "id").expect("identity selector should build"),
            )
        },
        |shape| {
            shape.field(
                AuthoredResultShapeField::new("identity", "id", "id")
                    .expect("identity result field should build"),
            )
        },
    )
}

fn write_task(workspace: &mut crate::runtime::WorthQueryWorkspace, label: &str) {
    workspace
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value(label)),
                ("title.value", test_string_aspect_value(label)),
            ],
        ))
        .expect("task write should advance the snapshot");
}
