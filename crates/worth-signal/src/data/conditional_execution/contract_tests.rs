use worth_proof::TransitionOutcome;

use crate::data::aspect::{Aspect, AspectMask, SignalAspectLoweringOwner};
use crate::data::graph::SignalGraph;

use super::{
    SignalConditionalArtifactReuse, SignalConditionalCondition,
    SignalConditionalContractDefinition, SignalConditionalVersionComparator,
    SignalDeltaThresholdContract, SignalThresholdBoundary, SignalThresholdComparisonDomain,
    SignalThresholdValueFamily,
};

#[test]
fn reinstalling_one_graph_node_preserves_role_bound_provider_identities() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let owner = SignalAspectLoweringOwner::fresh();
    graph.claim_aspect_lowering_owner(&owner).unwrap();
    let definition = SignalConditionalContractDefinition {
        condition: SignalConditionalCondition::RuntimePredicate,
        dependency_aspects: AspectMask::from_aspect(Aspect::new(1)),
        trigger_aspects: AspectMask::from_aspect(Aspect::new(1)),
        dependency_comparator: SignalConditionalVersionComparator::RuntimeResolved,
        output_comparator: SignalConditionalVersionComparator::RuntimeResolved,
        artifact_reuse: SignalConditionalArtifactReuse::RuntimeResolved,
    };

    let TransitionOutcome::Success(first_capability) = graph.admit_installed_node(node) else {
        panic!("fresh node should admit")
    };
    let first = graph
        .install_conditional_contract(&owner, first_capability, definition.clone())
        .unwrap();
    let TransitionOutcome::Success(second_capability) = graph.admit_installed_node(node) else {
        panic!("installed node should remain current")
    };
    let second = graph
        .install_conditional_contract(&owner, second_capability, definition)
        .unwrap();

    assert!(first.compare_semantic_continuity(&second).is_ok());
    assert!(first.compare_execution_affinity(&second).is_ok());
}

#[test]
fn unitful_threshold_meaning_survives_signal_installation_without_numeric_flattening() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let owner = SignalAspectLoweringOwner::fresh();
    graph.claim_aspect_lowering_owner(&owner).unwrap();
    let TransitionOutcome::Success(node_capability) = graph.admit_installed_node(node) else {
        panic!("fresh node should admit")
    };
    let threshold = SignalDeltaThresholdContract::new(
        worth_foundational::facade::AspectValue::Float64(
            worth_foundational::facade::CanonicalF64::from_f64(0.01),
        ),
        "worth.tests.units.millimeters",
        SignalThresholdValueFamily::Float64,
        SignalThresholdComparisonDomain::AbsoluteDifference,
        SignalThresholdBoundary::Inclusive,
    );
    let contract = graph
        .install_conditional_contract(
            &owner,
            node_capability,
            SignalConditionalContractDefinition {
                condition: SignalConditionalCondition::DeltaThreshold(threshold.clone()),
                dependency_aspects: AspectMask::from_aspect(Aspect::new(1)),
                trigger_aspects: AspectMask::from_aspect(Aspect::new(1)),
                dependency_comparator: SignalConditionalVersionComparator::Exact,
                output_comparator: SignalConditionalVersionComparator::Exact,
                artifact_reuse: SignalConditionalArtifactReuse::NotReusable,
            },
        )
        .unwrap();

    assert_eq!(
        contract.semantic_condition(),
        &SignalConditionalCondition::DeltaThreshold(threshold)
    );
    assert!(matches!(
        contract.condition(),
        crate::data::node::EvaluationCondition::Installed(_)
    ));
}
