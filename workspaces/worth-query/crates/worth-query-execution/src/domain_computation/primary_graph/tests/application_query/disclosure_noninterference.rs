use std::collections::BTreeSet;
use std::num::NonZeroUsize;
use std::time::{Duration, UNIX_EPOCH};

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_query::ApplicationQueryParameterSet;
use worth_query_installation::facade::TypedApplicationValue;

use super::super::fixture::{
    admit_touch_account_capability, installed_capability_world_with_label, live_scope,
    status_parameter, AccountIdentity, AuthorizationWorld, CapabilityDisclosure,
    GovernedAccountOmissionQuery, GovernedAccountOmissionResult, GovernedHiddenOrderingQuery,
    ResultRulePredicateQuery,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationDisclosed, WorthQueryApplicationDisclosureDecisionFact,
    WorthQueryApplicationDisclosureOutcome, WorthQueryApplicationDisclosureReceiptPosture,
    WorthQueryApplicationProjectionDenialKind, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryControls,
    WorthQueryApplicationQueryOmissionPosture, WorthQueryPrincipalResolutionMode,
};

#[derive(Debug, Eq, PartialEq)]
struct ConsumerObservation {
    rows: Vec<GovernedAccountOmissionResult>,
    disclosure_posture: WorthQueryApplicationDisclosureReceiptPosture,
    classification: Option<String>,
    disclosed: Vec<AspectValue>,
    omitted: Vec<AspectValue>,
    decisions: Vec<WorthQueryApplicationDisclosureDecisionFact>,
    disclosure_decision_count: usize,
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
    assert_eq!(left.disclosure_decision_count, 5);
    assert_eq!(
        left.decisions
            .iter()
            .filter(|decision| {
                decision.required_disclosure()
                    == &CapabilityDisclosure::PrivateLabel.into_foundational_value()
            })
            .count(),
        4
    );
    assert_eq!(
        left.decisions
            .iter()
            .filter(|decision| {
                decision.outcome() == WorthQueryApplicationDisclosureOutcome::Omitted
            })
            .count(),
        4
    );
    let slots = left
        .decisions
        .iter()
        .map(|decision| *decision.slot())
        .collect::<BTreeSet<_>>();
    assert_eq!(slots.len(), left.decisions.len());
    assert!(left
        .decisions
        .windows(2)
        .all(|decisions| decisions[0].slot() < decisions[1].slot()));
}

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

#[test]
fn hidden_ordering_material_is_consumed_before_domain_projection() {
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
        .application_query(GovernedHiddenOrderingQuery::reference())
        .unwrap();
    let capability = admit_touch_account_capability(&world, &principal, &request).unwrap();
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_governed_application_query(
            &query,
            &access,
            capability,
            ApplicationQueryParameterSet::<GovernedHiddenOrderingQuery>::new(),
            WorthQueryApplicationQueryControls::current_one_shot(
                NonZeroUsize::new(1).unwrap(),
                NonZeroUsize::new(512).unwrap(),
                &request,
            ),
        )
        .unwrap();
    let result = world
        .application
        .execute_application_query_one_shot(plan)
        .unwrap();
    let WorthQueryApplicationDisclosed::Disclosed(activities) = result.rows()[0].activities()
    else {
        panic!("the activity collection must be disclosed");
    };
    let identities = activities
        .iter()
        .map(|activity| match activity.identity() {
            WorthQueryApplicationDisclosed::Disclosed(identity) => identity.as_str(),
            WorthQueryApplicationDisclosed::Omitted(_) => panic!("identity must be disclosed"),
        })
        .collect::<Vec<_>>();
    assert_eq!(identities, ["activity-primary", "activity-secondary"]);
    for activity in activities {
        assert!(matches!(
            activity.sequence(),
            WorthQueryApplicationDisclosed::Omitted(_)
        ));
        assert_eq!(
            activity.required_sequence_denial(),
            WorthQueryApplicationProjectionDenialKind::FieldOmitted
        );
    }
    assert_eq!(result.receipt().ordering_comparison_count(), 1);
    assert_eq!(result.receipt().projected_field_count(), 4);
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
    let capability_session = capability.graph_work_session_identity();
    let capability_run = capability.graph_work_managed_run_identity();
    let capability_branch = capability.graph_work_branch().clone();
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
    assert_ne!(plan.graph_work_session_identity(), capability_session);
    assert_ne!(plan.graph_work_managed_run_identity(), capability_run);
    assert_eq!(plan.graph_work_branch(), &capability_branch);
    assert!(plan.graph_work_capability_identity().is_some());
    assert!(plan.graph_work_decision_fact_count() >= 3);
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
        decisions: disclosure.decisions().to_vec(),
        disclosure_decision_count: disclosure.disclosure_decision_count(),
        capability_identity_present: disclosure.capability_authority_identity().is_some(),
        decision_identity_present: disclosure.decision_identity().is_some(),
        decision_fact_count: disclosure.authorization_decision_fact_count(),
        omission_posture: result.receipt().omission_posture(),
        result_count: result.receipt().result_count(),
        projected_field_count: result.receipt().projected_field_count(),
        adjacency_list_read_count: result.receipt().adjacency_list_read_count(),
        edge_scan_count: result.receipt().edge_scan_count(),
    }
}
