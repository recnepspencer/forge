use std::num::NonZeroUsize;
use std::time::{Duration, UNIX_EPOCH};

use worth_query_declaration::facade::application_query::ApplicationQueryParameterSet;

use super::super::super::fixture::{
    admit_touch_account_capability, installed_capability_world_with_label, live_scope,
    status_parameter, AccountIdentity, ResultRulePredicateQuery,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryAdmissionDenialKind,
    WorthQueryApplicationQueryControls, WorthQueryPrincipalResolutionMode,
};

#[test]
fn result_disclosure_rule_cannot_open_a_predicate_read() {
    let mut world = installed_capability_world_with_label("private");
    world.application.script_authorization_time([
        UNIX_EPOCH + Duration::from_secs(100),
        UNIX_EPOCH + Duration::from_secs(100),
    ]);
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
    let account = world
        .application
        .resolve_entity(
            AccountIdentity::reference(),
            "account-1".to_owned(),
            &request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let query = world
        .application
        .installed_schema()
        .application_query(ResultRulePredicateQuery::reference())
        .unwrap();
    let capability = admit_touch_account_capability(&world, &principal, &request).unwrap();
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let denial = match world.application.admit_governed_application_query(
        &query,
        &access,
        capability,
        ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_owned()),
        WorthQueryApplicationQueryControls::current_one_shot(
            NonZeroUsize::new(1).unwrap(),
            NonZeroUsize::new(256).unwrap(),
            &request,
        ),
    ) {
        Ok(_) => panic!("result disclosure must not counterfeit internal authority"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationQueryAdmissionDenialKind::DisclosureContractInvalid
    );
}
