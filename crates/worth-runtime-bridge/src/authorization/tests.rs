use worth_foundational::facade::CanonicalDigestId;
use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;
use worth_query_declaration::{worth_query_application_schema, worth_query_entity};

use super::{
    BridgeAuthorizationClauseContract, BridgeAuthorizationClauseObservation,
    BridgeAuthorizationDependencyCardinality, BridgeAuthorizationInstallationRequest,
    BridgeAuthorizationObservation, BridgeAuthorizationRequirementContract,
    BridgeAuthorizationRequirementObservation, BridgeAuthorizationRuleContract,
    BridgeAuthorizationRuleEffect, BridgeAuthorizationRuleObservation, BridgeAuthorizationRuntime,
};

worth_query_application_schema! {
    schema BridgeAuthorizationSchema {
        owner: bridge_authorization_test,
        version: (1, 0),
        members: |schema| {
            schema.entity(BridgePrincipal::reference())
        }
    }
}

worth_query_entity!(BridgePrincipal in BridgeAuthorizationSchema);

#[test]
fn installed_correspondence_retains_nested_signal_decision_and_dependency_identity() {
    let mut runtime = BridgeAuthorizationRuntime::new();
    let rules = rules();
    let identity = runtime
        .install(BridgeAuthorizationInstallationRequest::new(
            &CanonicalDigestId::new([9; 32]),
            binding_identity(),
            "view_account",
            "Account",
            "account_membership",
            rules.clone(),
        ))
        .unwrap();
    let evidence = runtime
        .evaluate(observation(identity, &rules, [8; 32], true, false))
        .unwrap();

    assert!(runtime.retains(&evidence));
    assert_eq!(evidence.dependency_identity(), &[8; 32]);
    assert_eq!(
        evidence.decision(),
        worth_signal::facade::SignalAuthorizationDecision::Allowed
    );
    assert_eq!(evidence.counters().requirements_evaluated, 2);
}

#[test]
fn correspondence_rejects_clause_reordering_and_non_exhaustive_facts() {
    let mut runtime = BridgeAuthorizationRuntime::new();
    let rules = rules();
    let identity = runtime
        .install(BridgeAuthorizationInstallationRequest::new(
            &CanonicalDigestId::new([10; 32]),
            binding_identity(),
            "approve_payment",
            "PaymentIntent",
            "distinct_approver",
            rules.clone(),
        ))
        .unwrap();

    let mut reordered = observation(identity, &rules, [3; 32], true, false);
    reordered.rules[0].requirements[0].clauses.swap(0, 1);
    assert!(runtime.evaluate(reordered).is_err());

    let incomplete = observation(identity, &rules, [4; 32], true, false);
    let BridgeAuthorizationObservation {
        correspondence,
        dependency_identity,
        mut rules,
    } = incomplete;
    rules[0].requirements[0].clauses[0] =
        clause_observation(*rules[0].requirements[0].clauses[0].identity(), true, false);
    assert!(runtime
        .evaluate(BridgeAuthorizationObservation {
            correspondence,
            dependency_identity,
            rules,
        })
        .is_err());
}

fn rules() -> Vec<BridgeAuthorizationRuleContract> {
    vec![
        BridgeAuthorizationRuleContract::all(
            BridgeAuthorizationRuleEffect::Required,
            [BridgeAuthorizationRequirementContract::any([
                BridgeAuthorizationClauseContract::new([1; 32]),
                BridgeAuthorizationClauseContract::new([2; 32]),
            ])],
        ),
        BridgeAuthorizationRuleContract::all(
            BridgeAuthorizationRuleEffect::Prohibited,
            [BridgeAuthorizationRequirementContract::any([
                BridgeAuthorizationClauseContract::new([3; 32]),
            ])],
        ),
    ]
}

fn observation(
    identity: super::BridgeAuthorizationCorrespondenceIdentity,
    rules: &[BridgeAuthorizationRuleContract],
    dependency_identity: [u8; 32],
    allowed: bool,
    prohibited: bool,
) -> BridgeAuthorizationObservation {
    BridgeAuthorizationObservation::new(
        identity,
        dependency_identity,
        [
            rule_observation(
                &rules[0],
                [requirement_observation(
                    &rules[0].requirements()[0],
                    [(allowed, true), (false, true)],
                )],
            ),
            rule_observation(
                &rules[1],
                [requirement_observation(
                    &rules[1].requirements()[0],
                    [(prohibited, true)],
                )],
            ),
        ],
    )
}

fn rule_observation(
    contract: &BridgeAuthorizationRuleContract,
    requirements: impl IntoIterator<Item = BridgeAuthorizationRequirementObservation>,
) -> BridgeAuthorizationRuleObservation {
    BridgeAuthorizationRuleObservation::all(contract.effect(), requirements)
}

fn requirement_observation<const N: usize>(
    contract: &BridgeAuthorizationRequirementContract,
    states: [(bool, bool); N],
) -> BridgeAuthorizationRequirementObservation {
    BridgeAuthorizationRequirementObservation::any(contract.clauses().iter().zip(states).map(
        |(clause, (matched, exhaustive))| {
            clause_observation(*clause.identity(), matched, exhaustive)
        },
    ))
}

fn clause_observation(
    identity: [u8; 32],
    matched: bool,
    exhaustive: bool,
) -> BridgeAuthorizationClauseObservation {
    BridgeAuthorizationClauseObservation::new(
        identity,
        matched,
        exhaustive,
        BridgeAuthorizationDependencyCardinality {
            entities: 2,
            relations: 1,
            adjacency_lists: 1,
            fields: 1,
        },
    )
}

fn binding_identity() -> ApplicationSchemaBindingIdentity {
    ApplicationSchemaBindingIdentity::from_installed_parts(
        1,
        1,
        CanonicalDigestId::new([11; 32]),
        CanonicalDigestId::new([12; 32]),
    )
}
