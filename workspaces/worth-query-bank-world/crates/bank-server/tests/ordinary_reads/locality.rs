use bank_domain::model::ReadOutcome;
use bank_server::{queries, BankReadControls, BankReadDenial};

use super::fixture::{ordinary_read_world, over_budget_discovery_world, OWNER};
use crate::support::request_scope;

#[test]
fn discovery_and_account_reads_are_bounded_by_the_touched_neighborhood() {
    let baseline = ordinary_read_world("read-local-baseline", 0);
    let expanded = ordinary_read_world("read-local-expanded", 32);
    let baseline_owner = baseline.authenticate(OWNER);
    let expanded_owner = expanded.authenticate(OWNER);

    let baseline_accounts = delivered(
        baseline
            .world
            .runtime
            .query(queries::accounts())
            .as_principal(&baseline_owner)
            .controls(controls(8))
            .execute(),
    );
    let expanded_accounts = delivered(
        expanded
            .world
            .runtime
            .query(queries::accounts())
            .as_principal(&expanded_owner)
            .controls(controls(8))
            .execute(),
    );

    assert_eq!(baseline_accounts.output(), expanded_accounts.output());
    assert_eq!(
        baseline_accounts.metadata().work(),
        expanded_accounts.metadata().work()
    );
    assert_eq!(
        expanded_accounts.metadata().work().reconstructive_scans(),
        0
    );
    assert_eq!(expanded_accounts.output().len(), 2);
    let account_ids = expanded_accounts
        .output()
        .iter()
        .map(|account| account.id())
        .collect::<Vec<_>>();
    let mut expected_ids = vec![expanded.personal_account, expanded.business_account];
    expected_ids.sort();
    assert_eq!(account_ids, expected_ids);

    let summary = delivered(
        expanded
            .world
            .runtime
            .query(queries::account_summary(expanded.personal_account))
            .as_principal(&expanded_owner)
            .controls(controls(1))
            .execute(),
    );
    assert_eq!(summary.output().current_balance().minor_units(), 7_500);
    assert_eq!(summary.output().available_balance().minor_units(), 7_500);
    assert_eq!(summary.metadata().work().reconstructive_scans(), 0);
}

#[test]
fn activity_limit_is_enforced_and_reported_by_the_public_result() {
    let fixture = ordinary_read_world("read-activity-limit", 0);
    let owner = fixture.authenticate(OWNER);
    let activity = delivered(
        fixture
            .world
            .runtime
            .query(queries::account_activity(fixture.personal_account))
            .as_principal(&owner)
            .controls(controls(1))
            .execute(),
    );

    assert_eq!(activity.output().len(), 1);
    assert_eq!(activity.metadata().result_count(), 1);
    assert!(activity.metadata().truncated());
    assert_eq!(activity.metadata().work().reconstructive_scans(), 0);
}

#[test]
fn installed_projection_budget_denies_before_unbounded_discovery_work() {
    let fixture = over_budget_discovery_world("read-work-budget", 160);
    let actor = fixture.authenticate();
    let outcome = fixture
        .world
        .runtime
        .query(queries::accounts())
        .as_principal(&actor)
        .controls(controls(1_024))
        .execute();

    assert!(matches!(
        outcome,
        ReadOutcome::Denied(BankReadDenial::ProjectionWorkBudgetExceeded)
    ));
}

fn controls(maximum_results: usize) -> BankReadControls {
    BankReadControls::current(request_scope(), maximum_results).unwrap()
}

fn delivered<T, D>(outcome: ReadOutcome<T, D>) -> T {
    match outcome {
        ReadOutcome::Delivered(result) => result,
        _ => panic!("expected a delivered read"),
    }
}
