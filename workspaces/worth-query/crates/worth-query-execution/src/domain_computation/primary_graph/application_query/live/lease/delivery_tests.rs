use std::time::Duration;

use super::super::{
    WorthQueryApplicationLiveCloseOutcome, WorthQueryApplicationLiveControls,
    WorthQueryApplicationLiveOutcome,
};
use crate::domain_computation::primary_graph::tests::{
    fixture::{
        installed_authorization_world, live_account_parameters, live_scope, Account,
        AccountIdentity, AccountSummaryParameters, Activity, LiveAccountActivityCause,
        LiveAccountActivityQuery, LiveAccountActivityResult,
    },
    live_delivery_support::commit_live_activity,
};
use crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionMode;

#[test]
fn committed_live_cause_projects_with_bounded_result_buffer_evidence() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let external = world.authenticate("alice", Duration::from_secs(60), &request);
    let principal = world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = world
        .application
        .installed_schema()
        .application_query(LiveAccountActivityQuery::reference())
        .unwrap();
    let account = world
        .application
        .resolve_entity(
            AccountIdentity::reference(),
            "account-1".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let observer = world.application.result_buffer_observer();
    let mut lease = world
        .application
        .open_application_query_live::<
            LiveAccountActivityQuery,
            AccountSummaryParameters,
            LiveAccountActivityResult,
            _,
            _,
            Account,
            Activity,
            LiveAccountActivityCause,
        >(
            query,
            &principal,
            account,
            live_account_parameters("account-1"),
            WorthQueryApplicationLiveControls::bounded(request.clone(), 4, 16, 2_048).unwrap(),
        )
        .unwrap();
    let committed = commit_live_activity(&world, &principal, &request);

    let WorthQueryApplicationLiveOutcome::Delivered(update) = lease.poll() else {
        panic!("the committed installed live cause must deliver");
    };
    assert_eq!(update.commit_id(), committed.commit_id());
    assert_eq!(update.result().account(), "account-1");
    assert_eq!(
        update.result().activities(),
        &[("activity-primary".to_owned(), 11)]
    );
    let buffer = update
        .receipt()
        .result_buffer()
        .expect("live projection must carry bounded result-buffer evidence");
    assert!(buffer.released());
    assert!(buffer.peak_bytes() > 0);
    assert!(buffer.peak_bytes() <= buffer.limit_bytes());
    assert_eq!(observer.observe().active_buffers(), 0);
    assert_eq!(observer.observe().retained_bytes(), 0);
    assert_eq!(
        lease.close(),
        WorthQueryApplicationLiveCloseOutcome::Completed
    );
}
