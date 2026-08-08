//! Public-consumer aftermath noninterference across protected-fact twins.

use bank_domain::schema::AccountStatus;
use bank_server::BankMutationCommitOutcome;
use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding;
use worth_query_host::facade::publication::application_aftermath::{
    WorthQueryPublishedAftermathPosture, WorthQueryPublishedApplicationAftermath,
};

use super::freeze_account::fixture::{freeze_world_with_protected_foreign_status, FreezeFixture};
use crate::support::request_scope;

#[test]
fn paired_protected_account_detail_worlds_publish_equal_aftermath_surfaces() {
    let left = freeze_world_with_protected_foreign_status(
        "r848-left",
        AccountStatus::Open,
        AccountStatus::Open,
    );
    let right = freeze_world_with_protected_foreign_status(
        "r848-right",
        AccountStatus::Open,
        AccountStatus::Closed,
    );
    assert_ne!(
        left.protected_foreign_status, right.protected_foreign_status,
        "paired worlds must genuinely differ in the protected fact"
    );

    let left = publish_freeze_outcome(&left, 31);
    let right = publish_freeze_outcome(&right, 32);

    assert_eq!(left, right);
    assert_eq!(
        left.committed.posture(),
        Some(WorthQueryPublishedAftermathPosture::Reversible)
    );
}

#[derive(Debug, Eq, PartialEq)]
struct AftermathPublicationObservation {
    committed: WorthQueryPublishedApplicationAftermath,
}

fn publish_freeze_outcome(fixture: &FreezeFixture, key: u8) -> AftermathPublicationObservation {
    let specialist = fixture.authenticate_specialist();
    let outcome = fixture
        .world
        .runtime
        .freeze_estate_account(
            &specialist,
            fixture.action(fixture.estate_account),
            WorthQueryApplicationIdempotencyBinding::new([key; 32], [key.wrapping_add(1); 32]),
            &request_scope(),
        )
        .expect("freeze must admit under both protected-fact twins");
    let BankMutationCommitOutcome::Committed(receipt) = outcome else {
        panic!("freeze must commit: {outcome:?}");
    };

    AftermathPublicationObservation {
        committed: receipt.aftermath().clone(),
    }
}
