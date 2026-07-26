use worth_query::facade::{
    foundation::WorthQueryEntityIdentity,
    runtime::{WorthQueryAspectTouch, WorthQueryAuthoredAspectValue, WorthQueryWorkspace},
};
use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui_query_binding::{
    WorthUiInstalledQueryBindingReference, WorthUiOperationLiveRefreshRequest,
    WorthUiOperationLiveSourceRefreshOutcome,
};
use worth_ui_test_support::WorthUiFrameworkTurnCertificationExt;

pub(super) fn apply_real_live_patch(
    session: &mut WorthUiActiveApplicationSession,
    reference: &WorthUiInstalledQueryBindingReference,
    measurement: &WorthQueryEntityIdentity,
    workspace: &mut WorthQueryWorkspace,
) {
    update_measurement(measurement, workspace);
    let mut refresh = None;
    let completion = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|source| {
                refresh = Some(source.refresh_operation_live(
                    WorthUiOperationLiveRefreshRequest::new(reference, workspace),
                ));
            });
        })
        .expect("no mounted presentation lease is active");
    drop(completion.into_completion());
    let WorthUiOperationLiveSourceRefreshOutcome::Staged(staging) = refresh
        .expect("the Query source runs")
        .expect("the real patch stages one UI consequence")
    else {
        panic!("a semantic Query update must stage one UI consequence")
    };
    assert_eq!(staging.change_order(), 1);
    assert_eq!(staging.counters().patch_operations_visited(), 1);
    assert_eq!(staging.counters().patch_facts_reported(), 1);
    assert_eq!(staging.counters().graph_effects_minted(), 1);
    assert_eq!(staging.counters().measurement_effects_minted(), 2);
    assert_eq!(staging.query_work().operations_materialized(), 1);
    assert_eq!(staging.query_work().full_collection_scans(), 0);
    assert_eq!(staging.query_work().unrelated_consumer_scans(), 0);
}

fn update_measurement(measurement: &WorthQueryEntityIdentity, workspace: &mut WorthQueryWorkspace) {
    workspace
        .update(measurement.clone(), |entity| {
            entity.set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("measurement.value")
                    .expect("static measurement aspect"),
                WorthQueryAuthoredAspectValue::native(
                    worth_foundational::facade::AspectValue::Float32(
                        worth_foundational::facade::CanonicalF32::from_f32(320.0),
                    ),
                ),
            )
        })
        .expect("real Query mutation succeeds");
}
