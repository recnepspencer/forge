use worth_query_host::facade::admission::application_query::WorthQueryApplicationQueryLane;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationLiveCloseOutcome, WorthQueryApplicationLiveControls,
    WorthQueryApplicationQueryBasisPosture, WorthQueryApplicationQueryResumeControls,
};

use super::support::{
    activity_request, activity_world, approve_first, assert_resources_released, controls, items,
    take_first_requested,
};
use crate::{BankEstateEmergencyAccessActivityLiveOutcome, BankReadControls};

#[test]
fn exact_activity_meaning_and_identity_survive_every_public_lane() {
    let mut world = activity_world("estate-emergency-activity-lane-parity");
    let one_shot = ready(&world, controls(8))
        .execute_with_approved_elevation(&world.approved)
        .expect("the exact approved activity view should execute");
    let historical = ready(&world, controls(8))
        .admit_historical_with_approved_elevation(&world.approved, |admitted| admitted.execute())
        .expect("historical activity should retain the approval-commit meaning");
    let preview_session = world
        .fixture
        .runtime
        .open_preview(&super::super::fixture::request_scope())
        .unwrap();
    let preview = ready(&world, controls(8))
        .admit_preview_with_approved_elevation(&world.approved, &preview_session, |admitted| {
            admitted.execute()
        })
        .expect("preview activity should preserve current meaning");
    let expected = items(one_shot.rows());
    let identity = one_shot.receipt().query_identity();
    assert!(one_shot.receipt().inspect().terminal_resources_released());
    assert!(historical.receipt().inspect().terminal_resources_released());
    assert!(preview.receipt().inspect().terminal_resources_released());
    assert_eq!(
        one_shot.receipt().lane(),
        WorthQueryApplicationQueryLane::OneShot
    );
    assert_eq!(
        one_shot.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Current
    );
    let paged = collect_pages(&world, identity);
    assert_eq!(
        expected.len(),
        2,
        "the query must traverse a real many relation"
    );
    assert_eq!(one_shot.rows().len(), 1);
    assert_eq!(one_shot.rows()[0].estate(), super::super::fixture::ESTATE);
    assert_eq!(historical.rows(), one_shot.rows());
    assert_eq!(preview.rows(), one_shot.rows());
    assert_eq!(paged, expected);
    assert_eq!(historical.receipt().query_identity(), identity);
    assert_eq!(preview.receipt().query_identity(), identity);
    assert_eq!(
        historical.receipt().lane(),
        WorthQueryApplicationQueryLane::Historical
    );
    assert_eq!(
        historical.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Historical
    );
    assert_eq!(
        preview.receipt().lane(),
        WorthQueryApplicationQueryLane::Preview
    );
    assert_eq!(
        preview.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Preview
    );
    assert!(preview_session.discard().unwrap().discarded());

    let live_controls = WorthQueryApplicationLiveControls::bounded(
        super::super::fixture::request_scope(),
        4,
        8,
        2_048,
    )
    .unwrap();
    let first_requested = take_first_requested(&mut world);
    let mut live = ready(&world, controls(8))
        .subscribe_with_approved_elevation(&world.approved, live_controls)
        .expect("the same exact elevation should open the activity live lane");
    approve_first(&world, first_requested);
    let BankEstateEmergencyAccessActivityLiveOutcome::Delivered(update) = live.poll() else {
        panic!("a real matching approval effect should deliver through publication");
    };
    let current = ready(&world, controls(8))
        .execute_with_approved_elevation(&world.approved)
        .expect("current one-shot should remain lawful after the matching cause");
    let delivered = items(update.rows());
    let current_items = items(current.rows());
    assert_eq!(update.rows().len(), 1);
    assert_eq!(update.rows()[0].estate(), super::super::fixture::ESTATE);
    assert_eq!(
        delivered.len(),
        1,
        "one live cause projects its exact child"
    );
    assert_eq!(
        current_items
            .iter()
            .find(|item| item.access() == delivered[0].access()),
        delivered.first(),
        "live and current one-shot must assign identical meaning to the changed child"
    );
    assert_eq!(update.receipt().query_identity(), identity);
    assert!(update.receipt().inspect().terminal_resources_released());
    assert_eq!(
        update.receipt().lane(),
        WorthQueryApplicationQueryLane::Live
    );
    assert_eq!(
        update.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Current
    );
    let WorthQueryApplicationLiveCloseOutcome::Completed(completion) = live.close() else {
        panic!("the live lane must release its opening graph-read session");
    };
    assert_eq!(completion.release().released_reservation_count(), 1);
    assert_resources_released(&world);
}

fn collect_pages(
    world: &super::support::ActivityWorld,
    expected_identity: &worth_query_host::facade::domain::WorthQueryInstalledApplicationQueryIdentity,
) -> Vec<bank_domain::queries::EstateEmergencyAccessActivityItem> {
    let first = ready(world, controls(1))
        .page_with_approved_elevation(&world.approved)
        .expect("the first activity page should execute");
    assert_continuation_receipt(
        first.receipt(),
        expected_identity,
        WorthQueryApplicationQueryBasisPosture::Current,
    );
    assert_page_scope(first.rows());
    let mut collected = items(first.rows());
    let (_, mut continuation) = first.into_parts();
    while let Some(next) = continuation {
        let page = ready(world, controls(1))
            .resume_with_approved_elevation(
                &world.approved,
                next,
                WorthQueryApplicationQueryResumeControls::new(
                    std::num::NonZeroUsize::new(1).unwrap(),
                    std::num::NonZeroUsize::new(20_000).unwrap(),
                    &super::super::fixture::request_scope(),
                ),
            )
            .expect("every activity continuation should readmit and execute");
        assert_continuation_receipt(
            page.receipt(),
            expected_identity,
            WorthQueryApplicationQueryBasisPosture::Pinned,
        );
        assert_page_scope(page.rows());
        collected.extend(items(page.rows()));
        (_, continuation) = page.into_parts();
    }
    collected
}

fn assert_page_scope(rows: &[bank_domain::queries::EstateEmergencyAccessActivity]) {
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].estate(), super::super::fixture::ESTATE);
}

fn assert_continuation_receipt(
    receipt: &worth_query_host::facade::publication::domain_computation::WorthQueryApplicationQueryPublicationReceipt,
    expected_identity: &worth_query_host::facade::domain::WorthQueryInstalledApplicationQueryIdentity,
    expected_basis: WorthQueryApplicationQueryBasisPosture,
) {
    assert!(receipt.inspect().terminal_resources_released());
    assert_eq!(receipt.query_identity(), expected_identity);
    assert_eq!(receipt.lane(), WorthQueryApplicationQueryLane::Continuation);
    assert_eq!(receipt.basis_posture(), expected_basis);
}

fn ready<'a>(
    world: &'a super::support::ActivityWorld,
    controls: BankReadControls,
) -> crate::BankReadyQuery<'a, 'a, bank_domain::queries::EstateEmergencyAccessActivityRequest> {
    world
        .fixture
        .runtime
        .query(activity_request())
        .as_principal(&world.requester)
        .controls(controls)
}
