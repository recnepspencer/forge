use super::*;

#[test]
fn live_consumer_fanout_keeps_each_delivery_free_of_canonical_work() {
    const CONSUMER_COUNT: usize = 32;

    let fixture = ordinary_read_world("live-consumer-canonical-scale", 0);
    let owner = fixture.authenticate(OWNER);
    let teller = fixture.authenticate(TELLER);
    let mut leases = (0..CONSUMER_COUNT)
        .map(|_| {
            fixture
                .world
                .runtime
                .account_activity(fixture.personal_account)
                .as_principal(&owner)
                .subscribe(live_controls())
                .expect("each bounded authenticated live consumer should open")
        })
        .collect::<Vec<_>>();

    commit_deposit(
        &fixture,
        &teller,
        fixture.personal_account,
        "live-consumer-canonical-scale-deposit",
    );

    let mut expected_phases = None;
    let mut expected_authorization = None;
    for lease in &mut leases {
        let BankAccountActivityLiveOutcome::Delivered(update) = lease.poll() else {
            panic!("each retained consumer must receive the matching commit")
        };
        let receipt = update.receipt();
        let phases = receipt.canonical_work();
        let authorization = receipt.authorization_work();
        assert_phase_posture(phases);
        assert_eq!(authorization.requirement_count(), 1);
        assert_eq!(authorization.canonical_work(), Default::default());
        assert_eq!(receipt.fallback_count(), 0);
        assert_eq!(*expected_phases.get_or_insert(phases), phases);
        assert_eq!(
            *expected_authorization.get_or_insert(authorization),
            authorization
        );
    }
    for lease in leases {
        assert!(matches!(
            lease.close(),
            worth_query_host::facade::primary_graph::WorthQueryApplicationLiveCloseOutcome::Completed(_)
        ));
    }
}
