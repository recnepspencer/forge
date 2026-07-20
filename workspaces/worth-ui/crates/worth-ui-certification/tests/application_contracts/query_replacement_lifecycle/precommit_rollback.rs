use worth_query::facade::domain;
use worth_ui::facade::app::{
    WorthUiApplicationCutoverDenial, WorthUiVirtualizedPlanSummaryRequest, WorthUiVisibleRange,
};
use worth_ui::facade::query_binding::{WorthUiQueryLiveOpenOutcome, WorthUiQueryWorkspaceExt};
use worth_ui_test_support::{
    with_activation_precommit_interruption, WorthUiActivationPrecommitStage,
};

use super::scenario::{
    application, installed_workspace, submission, FIRST_VIEW, NEXT_COMPONENT, SECOND_VIEW,
};
use super::support::{
    activation_boundary, admit_active_resource, admit_candidate_resource, close, open_resource,
    prepare_catalog,
};

#[test]
fn every_fallible_precommit_stage_reaps_only_the_real_candidate_query_resource() {
    let mut workspace = installed_workspace("query-precommit-matrix");
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let first = installed
        .live_measurement_view(FIRST_VIEW)
        .expect("predecessor view");
    let second = installed
        .live_measurement_view(SECOND_VIEW)
        .expect("candidate view");
    let mut session = application(first.clone(), second.clone())
        .launch()
        .expect("Query application launch");
    admit_active_resource(&mut session, &first, &mut workspace);

    for stage in WorthUiActivationPrecommitStage::ALL {
        assert_query_rollback_at(stage, &mut session, &second, &mut workspace);
    }

    assert!(matches!(
        first
            .open_using(domain::current(), &mut workspace)
            .expect("predecessor installed authority remains exact"),
        WorthUiQueryLiveOpenOutcome::Stopped(_)
    ));
    let _ = session.shutdown();
    close(open_resource(&first, &mut workspace), &mut workspace);
}

fn assert_query_rollback_at(
    stage: WorthUiActivationPrecommitStage,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    second: &worth_ui::facade::query_binding::WorthUiInstalledLiveQueryView,
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
        .into_execution()
        .unwrap_or_else(|_| panic!("predecessor turn remains executable"))
        .execute_virtualized_data_frame(prior_target)
        .expect("the exact predecessor Query target remains executable");

    close(open_resource(second, workspace), workspace);
}
