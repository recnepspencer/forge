use worth_ui::facade::observation::{UiChangeClassificationOutcome, UiObservationFamily};
use worth_ui::facade::rebind::{UiProducedFactFamily, UiQueryChangedFactKind};
use worth_ui_query_binding::{
    WorthUiCollectionChangeKind, WorthUiOperationLiveRefreshOutcome,
    WorthUiOperationLiveRefreshRequest, WorthUiQueryWorkspaceExt,
};
use worth_ui_test_support::WorthUiActiveSessionCertificationExt;

use crate::query_replacement_lifecycle::{
    mixed_real_lifecycle::query_patch::update_measurement,
    reset_workspace::installed_workspace_without_collection_entity_lookup,
    scenario::{
        application, installed_workspace_with_measurement_authority, FIRST_VIEW, SECOND_VIEW,
    },
    support::{admit_active_resource, close_retirement},
};

#[test]
fn real_incremental_and_reset_consequences_retain_query_owned_meaning() {
    prove_query_consequence(false);
    prove_query_consequence(true);
}

fn prove_query_consequence(reset_expected: bool) {
    let label = if reset_expected {
        "phase-312-tt07-reset"
    } else {
        "phase-312-tt07-incremental"
    };
    let (mut workspace, measurement) = if reset_expected {
        installed_workspace_without_collection_entity_lookup(label)
    } else {
        installed_workspace_with_measurement_authority(label)
    };
    let installed = workspace
        .worth_ui()
        .expect("Worth UI Query domain is installed");
    let first = installed.live_measurement_view(FIRST_VIEW).unwrap();
    let second = installed.live_measurement_view(SECOND_VIEW).unwrap();
    let mut session = application(first.clone(), second, &mut workspace)
        .launch()
        .expect("exact Query-backed application launches");
    let reference = admit_active_resource(&mut session, &first, &mut workspace);
    update_measurement(&measurement, &mut workspace);
    let consequence = match session
        .refresh_query_change(WorthUiOperationLiveRefreshRequest::new(
            &reference,
            &mut workspace,
        ))
        .expect("exact installed resource refreshes")
    {
        WorthUiOperationLiveRefreshOutcome::Applied(consequence) => consequence,
        WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("changed Query value must issue a consequence")
        }
    };
    match (reset_expected, consequence.kind()) {
        (false, WorthUiCollectionChangeKind::Incremental(_)) => {}
        (true, WorthUiCollectionChangeKind::Reset(reset)) => {
            assert!(reset.fresh_execution_required());
            assert_eq!(reset.maximum_replacement_rows(), 1);
        }
        _ => panic!("Query owner issued the wrong consequence posture"),
    }

    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_query(consequence).unwrap();
    let admitted = turn.seal().unwrap();
    assert_eq!(admitted.summary().families(), &[UiObservationFamily::Query]);
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("Query consequence must classify as changed"),
    };
    let fact = changed
        .facts()
        .iter()
        .find(|fact| fact.family() == UiProducedFactFamily::Query)
        .and_then(|fact| fact.query())
        .expect("UI classification retains typed Query meaning");
    match (reset_expected, fact.kind()) {
        (false, UiQueryChangedFactKind::Incremental(incremental)) => assert!(
            incremental.graph_effects()
                + incremental.measurement_effects()
                + incremental.allocation_effects()
                > 0
        ),
        (true, UiQueryChangedFactKind::Reset(reset)) => {
            assert!(reset.fresh_execution_required());
            assert_eq!(reset.maximum_replacement_rows(), 1);
        }
        _ => panic!("UI fact changed the Query-owned consequence posture"),
    }
    close_retirement(
        session.shutdown().into_operation_live_retirement(),
        &mut workspace,
    );
}
