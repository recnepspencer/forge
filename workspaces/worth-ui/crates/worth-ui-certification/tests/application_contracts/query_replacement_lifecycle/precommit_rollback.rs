use worth_ui::facade::app::{
    WorthUiApplicationCutoverDenial, WorthUiVirtualizedPlanSummaryRequest, WorthUiVisibleRange,
};
use worth_ui_query_binding::{
    WorthUiInstalledLiveQueryView, WorthUiInstalledQueryBindingReference, WorthUiQueryViewShape,
    WorthUiQueryWorkspaceExt,
};
use worth_ui_test_support::{
    with_activation_precommit_interruption, WorthUiActivationPrecommitStage,
};

use super::scenario::{
    installed_workspace, mixed_live_snapshot_application, submission, FIRST_VIEW, NEXT_COMPONENT,
    SECOND_VIEW, SNAPSHOT_VIEW,
};
use super::settled_snapshot_preservation::{admit_active_settlement, settle_snapshot};
use super::support::{
    activation_boundary, admit_active_resource, admit_candidate_resource,
    assert_active_operation_live_resource, close, close_retirement, open_resource, prepare_catalog,
};

#[test]
fn every_fallible_precommit_stage_reaps_candidate_live_state_and_preserves_exact_snapshot() {
    let mut workspace = installed_workspace("query-precommit-matrix");
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let first = installed
        .live_measurement_view(FIRST_VIEW)
        .expect("predecessor view");
    let second = installed
        .live_measurement_view(SECOND_VIEW)
        .expect("candidate view");
    let snapshot = installed
        .measurement_view(SNAPSHOT_VIEW)
        .expect("snapshot view");
    let snapshot_identity = snapshot.definition().identity().clone();
    let app =
        mixed_live_snapshot_application(first.clone(), second.clone(), snapshot, &mut workspace);
    let snapshot_reference = app
        .resolve_query_view(&snapshot_identity, WorthUiQueryViewShape::Collection)
        .expect("application retains the exact snapshot reference");
    let mut session = app.launch().expect("Query application launch");
    admit_active_resource(&mut session, &first, &mut workspace);
    let initial_snapshot = admit_active_settlement(
        &mut session,
        settle_snapshot(&snapshot_reference, &mut workspace),
        false,
    );
    assert_eq!(initial_snapshot.source_generation().unwrap().as_u64(), 1);

    for (index, stage) in WorthUiActivationPrecommitStage::ALL.into_iter().enumerate() {
        assert_query_rollback_at(
            stage,
            &mut session,
            &second,
            &snapshot_reference,
            (index + 2) as u64,
            &mut workspace,
        );
    }

    assert_active_operation_live_resource(&session);
    close_retirement(
        session.shutdown().into_operation_live_retirement(),
        &mut workspace,
    );
    close(open_resource(&first, &mut workspace), &mut workspace);
}

fn assert_query_rollback_at(
    stage: WorthUiActivationPrecommitStage,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    second: &WorthUiInstalledLiveQueryView,
    snapshot_reference: &WorthUiInstalledQueryBindingReference,
    expected_snapshot_generation: u64,
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
) {
    let label = format!("query-precommit-{}", stage.label().replace(' ', "-"));
    let prior_target = session
        .inspect_virtualized_plan(WorthUiVirtualizedPlanSummaryRequest::first_view())
        .expect("predecessor Query summary")
        .target(WorthUiVisibleRange::rows(0, 1).expect("one visible row"));
    let mut candidate = prepare_catalog(
        session,
        submission(
            &label,
            NEXT_COMPONENT,
            &[SECOND_VIEW],
            session.capabilities(),
        ),
    );
    admit_candidate_resource(&mut candidate.0, second, workspace);
    let lowered = session
        .lower_prepared_replacement(*candidate.0)
        .expect("candidate lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("candidate stages");
    let boundary = activation_boundary(session);
    let prior_generation = session.generation_identity().clone();
    let prior_runtime = session.inspect_runtime();

    let (cutover, observed) = with_activation_precommit_interruption(stage, || {
        session.activate_prepared_replacement(pending, candidate.1, boundary, None)
    });
    assert_eq!(observed, Some(stage), "{} was not reached", stage.label());
    let denial = match cutover {
        Ok(_) => panic!("the armed precommit stage must deny publication"),
        Err(denial) => denial,
    };
    assert!(
        matches!(denial, WorthUiApplicationCutoverDenial::Activation(_)),
        "{} must deny inside the production activation transaction",
        stage.label()
    );
    drop(denial);

    assert_eq!(session.generation_identity(), &prior_generation);
    let after = session.inspect_runtime();
    assert_eq!(after.artifact_digest(), prior_runtime.artifact_digest());
    assert_eq!(
        after.active_plan_digest(),
        prior_runtime.active_plan_digest()
    );
    assert_eq!(after.snapshot_digest(), prior_runtime.snapshot_digest());
    session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("predecessor turn remains executable"))
        .execute_virtualized_data_frame(prior_target)
        .expect("the exact predecessor Query target remains executable");

    let refreshed_snapshot = admit_active_settlement(
        session,
        settle_snapshot(snapshot_reference, workspace),
        true,
    );
    assert_eq!(
        refreshed_snapshot.source_generation().unwrap().as_u64(),
        expected_snapshot_generation,
        "{} must leave the exact predecessor settlement refreshable",
        stage.label()
    );

    close(open_resource(second, workspace), workspace);
}
