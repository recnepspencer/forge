use worth_foundational::facade::CanonicalDigestId;
use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;
use worth_query_declaration::{worth_query_application_schema, worth_query_entity};

use super::{
    BridgeAuthorizationDependencyCardinality, BridgeAuthorizationInstallationRequest,
    BridgeAuthorizationObservation, BridgeAuthorizationPathContract, BridgeAuthorizationPathEffect,
    BridgeAuthorizationPathObservation, BridgeAuthorizationRuntime,
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
fn installed_correspondence_retains_signal_decision_and_exact_dependency_identity() {
    let mut runtime = BridgeAuthorizationRuntime::new();
    let paths = paths();
    let identity = runtime
        .install(BridgeAuthorizationInstallationRequest::new(
            &CanonicalDigestId::new([9; 32]),
            binding_identity(),
            "view_account",
            "Account",
            "account_membership",
            paths.clone(),
        ))
        .unwrap();
    assert_eq!(identity.bytes(), &[9; 32]);
    let evidence = runtime
        .evaluate(BridgeAuthorizationObservation::new(
            identity,
            [8; 32],
            [
                observation(paths[0], true, true),
                observation(paths[1], false, true),
            ],
        ))
        .unwrap();

    assert!(runtime.retains(&evidence));
    assert_eq!(evidence.dependency_identity(), &[8; 32]);
    assert_eq!(
        evidence.decision(),
        worth_signal::facade::SignalAuthorizationDecision::Allowed
    );
}

#[test]
fn correspondence_rejects_reordered_or_non_exhaustive_policy_facts() {
    let mut runtime = BridgeAuthorizationRuntime::new();
    let paths = paths();
    let identity = runtime
        .install(BridgeAuthorizationInstallationRequest::new(
            &CanonicalDigestId::new([10; 32]),
            binding_identity(),
            "approve_payment",
            "PaymentIntent",
            "distinct_approver",
            paths.clone(),
        ))
        .unwrap();
    let reordered = runtime.evaluate(BridgeAuthorizationObservation::new(
        identity,
        [3; 32],
        [
            observation(paths[1], false, true),
            observation(paths[0], true, true),
        ],
    ));
    assert!(reordered.is_err());
    let incomplete = runtime.evaluate(BridgeAuthorizationObservation::new(
        identity,
        [3; 32],
        [
            observation(paths[0], true, false),
            observation(paths[1], false, true),
        ],
    ));
    assert!(incomplete.is_err());
}

fn paths() -> Vec<BridgeAuthorizationPathContract> {
    vec![
        BridgeAuthorizationPathContract::new([1; 32], BridgeAuthorizationPathEffect::Allow),
        BridgeAuthorizationPathContract::new([2; 32], BridgeAuthorizationPathEffect::Deny),
    ]
}

fn observation(
    contract: BridgeAuthorizationPathContract,
    matched: bool,
    exhaustive: bool,
) -> BridgeAuthorizationPathObservation {
    BridgeAuthorizationPathObservation::new(
        *contract.identity(),
        contract.effect(),
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
