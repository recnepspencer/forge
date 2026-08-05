use std::num::NonZeroUsize;

use bank_server::BankApplicationQueryDenial;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryControls,
    WorthQueryOperationAuthorizationDenialKind,
};

use super::fixture::{ordinary_read_world, OWNER, STRANGER};
use crate::support::request_scope;

#[test]
fn installed_account_activity_is_a_real_ordered_bank_query() {
    let fixture = ordinary_read_world("installed-account-activity", 0);
    let owner = fixture.authenticate(OWNER);
    let request = request_scope();
    let result = fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&owner)
        .execute(current_controls(&request))
        .expect("account owner should execute the installed query");

    assert_eq!(result.rows().len(), 1);
    let activity = &result.rows()[0];
    assert_eq!(activity.account(), fixture.personal_account);
    assert_eq!(
        activity
            .entries()
            .iter()
            .map(|item| item.account_sequence().get())
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(
        activity
            .entries()
            .iter()
            .map(|item| item.amount().minor_units())
            .collect::<Vec<_>>(),
        vec![10_000, -2_500]
    );

    let receipt = result.receipt();
    let requirements = receipt.graph_read_plan().requirements().counters();
    assert!(requirements.reverse_adjacency_count() >= 2);
    assert!(requirements.directional_adjacency_count() >= 1);
    assert!(requirements.ordering_support_count() >= 1);
    assert!(receipt.ordering_comparison_count() >= 1);
    assert!(receipt.adjacency_list_read_count() >= 1);
    assert_eq!(receipt.fallback_count(), 0);
    assert_eq!(receipt.per_result_neighbor_lookup_count(), 0);
    assert!(receipt.basis_released());
    let terminal = receipt.read_completion();
    assert_eq!(terminal.basis_identity(), receipt.basis_identity());
    assert!(terminal.basis_release().released());
    assert_eq!(terminal.release().released_reservation_count(), 1);
}

#[test]
fn installed_account_activity_denies_a_mapped_stranger_before_plan_authority() {
    let fixture = ordinary_read_world("installed-account-activity-stranger", 0);
    let stranger = fixture.authenticate(STRANGER);
    let request = request_scope();
    let outcome = fixture
        .world
        .runtime
        .account_activity(fixture.personal_account)
        .as_principal(&stranger)
        .execute(current_controls(&request));
    let denial = match outcome {
        Err(denial) => denial,
        Ok(_) => panic!("mapped stranger received account query authority"),
    };

    assert!(matches!(
        denial,
        BankApplicationQueryDenial::Admission(error)
            if error.kind()
                == WorthQueryApplicationQueryAdmissionDenialKind::Authorization(
                    WorthQueryOperationAuthorizationDenialKind::PermissionDenied
                )
    ));
}

fn current_controls(
    request: &WorthQueryRequestScope,
) -> WorthQueryApplicationQueryControls<'_, bank_domain::schema::BankSchema> {
    WorthQueryApplicationQueryControls::current_one_shot(
        NonZeroUsize::new(64).expect("result ceiling is nonzero"),
        NonZeroUsize::new(100_000).expect("work ceiling is nonzero"),
        request,
    )
}
