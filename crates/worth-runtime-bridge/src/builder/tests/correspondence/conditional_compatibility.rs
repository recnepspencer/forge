use std::sync::Arc;

use worth_foundational::facade::CanonicalMismatchKind;
use worth_query_installation::facade::{
    WorthQueryConditionalNodeLocation, WorthQueryPortableConditionalDimension,
};
use worth_signal::facade::{Aspect, NodeEvaluationResult, PartitionToken, SignalGraph};

use super::real_query_dependencies::{
    conditional_node, conditional_node_always_eligible, dependency, freshly_installed_dependency,
};
use super::{exact_mapping, registration, runtime, BridgeAspectRegistrationId};
use crate::facade::{
    BridgeConditionalComputeProvider, BridgeConditionalContinuityMismatch,
    BridgeConditionalExecutionAffinityMismatch, BridgeConditionalInstallationRequest,
    BridgeConditionalProviderSemantics, BridgeConditionalProviderSet,
    BridgeInstalledConditionalLowering, BridgeOwnedSignalRuntime,
    BridgeSignalAspectTargetDeclaration,
};

mod certification;
mod provider_semantics;
mod retained_decision;

pub(super) struct Compute(pub(super) u64);

impl BridgeConditionalProviderSemantics for Compute {
    type SemanticContract = u64;

    fn semantic_contract(&self) -> Self::SemanticContract {
        self.0
    }
}

impl BridgeConditionalComputeProvider for Compute {
    fn compute(&self, _context: &mut dyn std::any::Any) -> Result<NodeEvaluationResult, String> {
        Ok(NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                self.0,
            )]),
        ))
    }
}

pub(super) fn install(
    declaration: worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    partition: &str,
) -> (
    BridgeOwnedSignalRuntime,
    Arc<BridgeInstalledConditionalLowering>,
) {
    install_with(
        declaration,
        partition,
        BridgeConditionalProviderSet::new().compute(Compute(1)),
    )
}

fn install_with(
    declaration: worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    partition: &str,
    providers: BridgeConditionalProviderSet,
) -> (
    BridgeOwnedSignalRuntime,
    Arc<BridgeInstalledConditionalLowering>,
) {
    install_with_target_partitions(declaration, &[partition], providers)
}

fn install_with_target_partitions(
    declaration: worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    partitions: &[&str],
    providers: BridgeConditionalProviderSet,
) -> (
    BridgeOwnedSignalRuntime,
    Arc<BridgeInstalledConditionalLowering>,
) {
    let (mut owner, request) = installation_fixture(declaration, partitions, providers);
    let lowering = owner
        .install(request)
        .expect("owner-bound conditional lowering installs");
    (owner, lowering)
}

fn installation_fixture(
    declaration: worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    partitions: &[&str],
    providers: BridgeConditionalProviderSet,
) -> (
    BridgeOwnedSignalRuntime,
    BridgeConditionalInstallationRequest,
) {
    installation_fixture_with_baseline(declaration, partitions, providers, &[])
}

fn installation_fixture_with_baseline(
    declaration: worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    partitions: &[&str],
    providers: BridgeConditionalProviderSet,
    baseline_labels: &[&str],
) -> (
    BridgeOwnedSignalRuntime,
    BridgeConditionalInstallationRequest,
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let targets = partitions
        .iter()
        .enumerate()
        .map(|(index, partition)| {
            let worth_proof::TransitionOutcome::Success(node_capability) =
                graph.admit_installed_node(node)
            else {
                panic!("fresh Signal node admits");
            };
            if partitions.len() == 1 {
                BridgeSignalAspectTargetDeclaration::allocate(
                    BridgeAspectRegistrationId::admit_bridge_owned("profile-name"),
                    PartitionToken::new(*partition),
                    node_capability,
                )
            } else {
                let worth_proof::TransitionOutcome::Success(aspect_capability) =
                    graph.admit_installed_aspect(node, Aspect::new(index as u8))
                else {
                    panic!("fresh Signal aspect admits");
                };
                BridgeSignalAspectTargetDeclaration::exact(
                    BridgeAspectRegistrationId::admit_bridge_owned("profile-name"),
                    PartitionToken::new(*partition),
                    node_capability,
                    aspect_capability,
                )
                .expect("node and aspect capabilities share the graph owner")
            }
        })
        .collect();
    let request_dependency = if baseline_labels.is_empty() {
        freshly_installed_dependency("query:one")
    } else {
        dependency("query:one")
    };
    let request_registration = registration(request_dependency, targets);
    let baseline = baseline_labels
        .iter()
        .map(|label| {
            let baseline_node = graph.node().build();
            let worth_proof::TransitionOutcome::Success(node_capability) =
                graph.admit_installed_node(baseline_node)
            else {
                panic!("baseline Signal node admits");
            };
            registration(
                dependency(label),
                vec![BridgeSignalAspectTargetDeclaration::allocate(
                    BridgeAspectRegistrationId::admit_bridge_owned("profile-name"),
                    PartitionToken::new("bridge-baseline"),
                    node_capability,
                )],
            )
        })
        .collect();
    let owner = BridgeOwnedSignalRuntime::new(runtime(exact_mapping(), baseline), graph)
        .expect("Bridge owns the fresh Signal runtime");
    (
        owner,
        BridgeConditionalInstallationRequest {
            declaration,
            location: WorthQueryConditionalNodeLocation::operation("query:one")
                .expect("valid operation location"),
            registrations: vec![request_registration],
            providers,
        },
    )
}
