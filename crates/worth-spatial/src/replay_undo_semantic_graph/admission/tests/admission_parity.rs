use crate::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    prepare_spatial_replay_semantic_graph_request,
};

use super::fixtures::{
    boolean_event_ledger_fixture, boolean_event_ledger_request, projection_receipt_fixture,
    projection_receipt_request,
};

#[test]
fn spatial_replay_admission_is_stable_for_same_boolean_event_ledger_authority() {
    let fixture = boolean_event_ledger_fixture();

    let first_prepared =
        prepare_spatial_replay_semantic_graph_request(boolean_event_ledger_request(&fixture))
            .expect("prepared request");
    let second_prepared =
        prepare_spatial_replay_semantic_graph_request(boolean_event_ledger_request(&fixture))
            .expect("prepared request");
    let first = admit_prepared_spatial_replay_semantic_graph_input(
        &fixture.family_catalog,
        &first_prepared,
    )
    .expect("admitted input");
    let second = admit_prepared_spatial_replay_semantic_graph_input(
        &fixture.family_catalog,
        &second_prepared,
    )
    .expect("admitted input");

    assert_eq!(first.family_identity(), second.family_identity());
    assert_eq!(
        first.semantic_graph_identity(),
        second.semantic_graph_identity()
    );
}

#[test]
fn spatial_replay_admission_is_stable_for_same_projection_receipt_authority() {
    let fixture = projection_receipt_fixture();

    let first_prepared =
        prepare_spatial_replay_semantic_graph_request(projection_receipt_request(&fixture))
            .expect("prepared request");
    let second_prepared =
        prepare_spatial_replay_semantic_graph_request(projection_receipt_request(&fixture))
            .expect("prepared request");
    let first = admit_prepared_spatial_replay_semantic_graph_input(
        &fixture.family_catalog,
        &first_prepared,
    )
    .expect("admitted input");
    let second = admit_prepared_spatial_replay_semantic_graph_input(
        &fixture.family_catalog,
        &second_prepared,
    )
    .expect("admitted input");

    assert_eq!(
        first.semantic_graph_identity(),
        second.semantic_graph_identity()
    );
    assert_eq!(first.retained_replay_receipt(), None);
}
