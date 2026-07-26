use worth_query::facade::domain;
use worth_ui_query_binding::{
    WorthUiQueryViewShape, WorthUiQueryWorkspaceExt, WorthUiSnapshotConsumerPreparationDenial,
};
use worth_ui_test_support::WorthUiActiveSessionCertificationExt;

use crate::query_consumer_kit_application::file_authored_query_app;
use crate::query_consumer_kit_workspace::{
    installed_measurement_workspace, interactive_borrowed_collection_requirements,
    unsupported_measurement_workspace,
};

#[test]
fn unsupported_consumer_contract_denies_without_mutating_the_active_application() {
    let workspace = unsupported_measurement_workspace("public-unsupported-query-consumer");
    let view = workspace
        .worth_ui()
        .unwrap()
        .measurement_view("inspector.measurements")
        .unwrap();
    let view_identity = view.definition().identity().clone();
    let app = file_authored_query_app(view);
    let reference = app
        .resolve_query_view(&view_identity, WorthUiQueryViewShape::Collection)
        .unwrap();
    let session = app.launch().expect("the prior application launches");
    let prior = session.inspect_runtime();

    let denial = match reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .prepare_snapshot_consumer(interactive_borrowed_collection_requirements())
    {
        Ok(_) => panic!("unsupported Query support cannot mint a consumer contract"),
        Err(denial) => denial,
    };

    assert!(matches!(
        denial,
        WorthUiSnapshotConsumerPreparationDenial::ConsumerContract(
            domain::WorthQueryConsumerProjectionContractDenial::Compatibility(_)
        )
    ));
    assert_eq!(session.inspect_runtime(), prior);
    assert!(session.inspect_query_state_residue().is_clean());
    let _ = session.shutdown();
}

#[test]
fn duplicate_consumer_contract_mint_is_query_owned_and_leaves_active_truth_complete() {
    let workspace = installed_measurement_workspace("public-duplicate-query-consumer");
    let view = workspace
        .worth_ui()
        .unwrap()
        .measurement_view("inspector.measurements")
        .unwrap();
    let view_identity = view.definition().identity().clone();
    let app = file_authored_query_app(view);
    let reference = app
        .resolve_query_view(&view_identity, WorthUiQueryViewShape::Collection)
        .unwrap();
    let session = app.launch().expect("the prior application launches");
    let prior = session.inspect_runtime();
    let bound = reference
        .enter_snapshot_attempt(&workspace)
        .unwrap()
        .bind_snapshot()
        .unwrap();

    let _first = bound.consumer_projection_contract().unwrap();
    let denial = match bound.consumer_projection_contract() {
        Ok(_) => panic!("one bound Query operation cannot mint a second contract"),
        Err(denial) => denial,
    };

    assert!(matches!(
        denial,
        domain::WorthQueryConsumerProjectionContractDenial::AlreadyMinted { .. }
    ));
    assert_eq!(session.inspect_runtime(), prior);
    assert!(session.inspect_query_state_residue().is_clean());
    let _ = session.shutdown();
}
