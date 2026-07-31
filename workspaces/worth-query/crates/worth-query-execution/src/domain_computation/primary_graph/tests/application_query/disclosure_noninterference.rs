use std::num::NonZeroUsize;
use std::time::{Duration, UNIX_EPOCH};

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_query::ApplicationQueryParameterSet;
use worth_query_installation::facade::TypedApplicationValue;

use super::super::fixture::{
    admit_touch_account_capability, installed_capability_world_with_label, live_scope,
    AccountIdentity, AuthorizationWorld, CapabilityDisclosure, GovernedAccountOmissionQuery,
    GovernedAccountOmissionResult,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationDisclosed, WorthQueryApplicationDisclosureReceiptPosture,
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryControls,
    WorthQueryApplicationQueryOmissionPosture, WorthQueryPrincipalResolutionMode,
};

#[derive(Debug, Eq, PartialEq)]
struct ConsumerObservation {
    rows: Vec<GovernedAccountOmissionResult>,
    disclosure_posture: WorthQueryApplicationDisclosureReceiptPosture,
    classification: Option<String>,
    disclosed: Vec<AspectValue>,
    omitted: Vec<AspectValue>,
    capability_identity_present: bool,
    decision_identity_present: bool,
    decision_fact_count: usize,
    omission_posture: WorthQueryApplicationQueryOmissionPosture,
    result_count: usize,
    projected_field_count: usize,
    adjacency_list_read_count: usize,
    edge_scan_count: usize,
}

#[test]
fn protected_label_difference_is_absent_from_every_one_shot_observable() {
    let left = observe("private-left");
    let right = observe("private-right");

    assert_eq!(left, right);
    assert_eq!(left.rows.len(), 1);
    assert_eq!(left.projected_field_count, 1);
    assert_eq!(left.adjacency_list_read_count, 0);
    assert_eq!(left.edge_scan_count, 0);
    assert!(matches!(
        left.rows[0].status(),
        WorthQueryApplicationDisclosed::Disclosed(status) if status == "open"
    ));
    let WorthQueryApplicationDisclosed::Omitted(omission) = left.rows[0].label() else {
        panic!("the protected label must be omitted before application projection");
    };
    assert_eq!(omission.classification(), "account-omission");
    assert_eq!(
        omission.required_disclosure(),
        &CapabilityDisclosure::PrivateLabel.into_foundational_value()
    );
    assert!(matches!(
        left.rows[0].activities(),
        WorthQueryApplicationDisclosed::Omitted(_)
    ));
}

fn observe(label: &str) -> ConsumerObservation {
    let mut world = installed_capability_world_with_label(label);
    world.application.script_authorization_time([
        UNIX_EPOCH + Duration::from_secs(100),
        UNIX_EPOCH + Duration::from_secs(100),
    ]);
    execute(&world)
}

fn execute(world: &AuthorizationWorld) -> ConsumerObservation {
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
        .application_query(GovernedAccountOmissionQuery::reference())
        .unwrap();
    let capability = admit_touch_account_capability(world, &principal, &request).unwrap();
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_governed_application_query(
            &query,
            &access,
            capability,
            ApplicationQueryParameterSet::<GovernedAccountOmissionQuery>::new(),
            WorthQueryApplicationQueryControls::current_one_shot(
                NonZeroUsize::new(1).unwrap(),
                NonZeroUsize::new(256).unwrap(),
                &request,
            ),
        )
        .unwrap();
    let result = world
        .application
        .execute_application_query_one_shot(plan)
        .unwrap();
    let disclosure = result.receipt().disclosure();
    ConsumerObservation {
        rows: result.rows().to_vec(),
        disclosure_posture: disclosure.posture(),
        classification: disclosure.classification().map(str::to_owned),
        disclosed: disclosure.disclosed().to_vec(),
        omitted: disclosure.omitted().to_vec(),
        capability_identity_present: disclosure.capability_authority_identity().is_some(),
        decision_identity_present: disclosure.decision_identity().is_some(),
        decision_fact_count: disclosure.decision_fact_count(),
        omission_posture: result.receipt().omission_posture(),
        result_count: result.receipt().result_count(),
        projected_field_count: result.receipt().projected_field_count(),
        adjacency_list_read_count: result.receipt().adjacency_list_read_count(),
        edge_scan_count: result.receipt().edge_scan_count(),
    }
}
