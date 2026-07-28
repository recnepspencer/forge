use crate::data::graph::SignalGraph;

use super::{
    SignalAuthorizationDecision, SignalAuthorizationDependencyCardinality,
    SignalAuthorizationObservation, SignalAuthorizationPathContract, SignalAuthorizationPathEffect,
    SignalAuthorizationPathObservation, SignalAuthorizationPolicyDefinition,
    SignalAuthorizationPolicyIdentity,
};

#[test]
fn installed_policy_evaluates_allow_and_retains_exact_authority() {
    let mut graph = SignalGraph::new();
    let graph_capability = graph.installed_graph_capability();
    let policy = graph
        .install_authorization_policy(&graph_capability, policy_definition([7; 32]))
        .unwrap();
    let evidence = graph
        .evaluate_authorization(&policy, observation([3; 32], true, false))
        .unwrap();

    assert_eq!(evidence.decision(), SignalAuthorizationDecision::Allowed);
    assert_eq!(evidence.dependency_identity(), &[3; 32]);
    assert!(policy.retains(&evidence));
    assert_eq!(evidence.counters().paths_evaluated, 2);
    assert_eq!(evidence.counters().entities_depended_on, 4);
}

#[test]
fn deny_precedence_and_non_exhaustive_observations_fail_closed() {
    let mut graph = SignalGraph::new();
    let graph_capability = graph.installed_graph_capability();
    let policy = graph
        .install_authorization_policy(&graph_capability, policy_definition([9; 32]))
        .unwrap();
    let denied = graph
        .evaluate_authorization(&policy, observation([4; 32], true, true))
        .unwrap();
    assert_eq!(denied.decision(), SignalAuthorizationDecision::Denied);

    let non_exhaustive = SignalAuthorizationObservation::new(
        [5; 32],
        [
            path(SignalAuthorizationPathEffect::Allow, true, false),
            path(SignalAuthorizationPathEffect::Deny, false, true),
        ],
    );
    assert!(graph
        .evaluate_authorization(&policy, non_exhaustive)
        .is_err());
}

fn policy_definition(identity: [u8; 32]) -> SignalAuthorizationPolicyDefinition {
    SignalAuthorizationPolicyDefinition::new(
        SignalAuthorizationPolicyIdentity::new(identity),
        [
            SignalAuthorizationPathContract::new(SignalAuthorizationPathEffect::Allow),
            SignalAuthorizationPathContract::new(SignalAuthorizationPathEffect::Deny),
        ],
    )
}

fn observation(identity: [u8; 32], allow: bool, deny: bool) -> SignalAuthorizationObservation {
    SignalAuthorizationObservation::new(
        identity,
        [
            path(SignalAuthorizationPathEffect::Allow, allow, true),
            path(SignalAuthorizationPathEffect::Deny, deny, true),
        ],
    )
}

fn path(
    effect: SignalAuthorizationPathEffect,
    matched: bool,
    exhaustive: bool,
) -> SignalAuthorizationPathObservation {
    SignalAuthorizationPathObservation::new(
        effect,
        matched,
        exhaustive,
        SignalAuthorizationDependencyCardinality {
            entities: 2,
            relations: 1,
            adjacency_lists: 1,
            fields: 1,
        },
    )
}
