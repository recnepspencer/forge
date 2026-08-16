use worth_signal::facade::InstalledSignalConditionDecision;

use super::{exact_mapping, runtime, Compute};
use crate::builder::tests::correspondence::semantic_dependencies::{
    freshly_installed_dependency, temporal_contract,
};
use crate::facade::{
    BridgeConditionalLocation, BridgeConditionalProviderSemantics, BridgeConditionalProviderSet,
    BridgeConditionalResolverContext, BridgeConditionalWakeProvider,
    BridgeOwnedConditionalInstallationRequest, BridgeOwnedSignalRuntime,
};

struct Wake;

impl BridgeConditionalProviderSemantics for Wake {
    type SemanticContract = &'static str;

    fn semantic_contract(&self) -> Self::SemanticContract {
        "bridge-test-temporal-wake"
    }
}

impl BridgeConditionalWakeProvider for Wake {
    fn resolve(
        &self,
        _context: BridgeConditionalResolverContext,
    ) -> Result<InstalledSignalConditionDecision, String> {
        Ok(InstalledSignalConditionDecision::Eligible)
    }
}

#[test]
fn bridge_allocates_conditional_signal_topology_from_semantic_dependencies() {
    let mut owner =
        BridgeOwnedSignalRuntime::with_owned_signal_graph(runtime(exact_mapping(), Vec::new()))
            .expect("Bridge owns the fresh Signal graph");

    let lowering = owner
        .install_owned_conditional(BridgeOwnedConditionalInstallationRequest {
            contract: temporal_contract("query:one"),
            location: BridgeConditionalLocation::operation("query:one"),
            dependencies: vec![freshly_installed_dependency("query:one")],
            providers: BridgeConditionalProviderSet::new()
                .wake(Wake)
                .compute(Compute(7)),
        })
        .expect("semantic dependencies lower without caller-owned Signal capabilities");

    assert_eq!(lowering.location().node_identity(), "query:one");
    assert_eq!(lowering.correspondence_count(), 1);
    assert_eq!(lowering.counters().signal_targets_joined, 1);
    assert_eq!(lowering.counters().signal_contract_installations, 1);
}

#[test]
fn bridge_rejects_owned_dependencies_from_another_conditional_node() {
    let mut owner =
        BridgeOwnedSignalRuntime::with_owned_signal_graph(runtime(exact_mapping(), Vec::new()))
            .expect("Bridge owns the fresh Signal graph");

    let result = owner.install_owned_conditional(BridgeOwnedConditionalInstallationRequest {
        contract: temporal_contract("query:first"),
        location: BridgeConditionalLocation::operation("query:first"),
        dependencies: vec![freshly_installed_dependency("query:second")],
        providers: BridgeConditionalProviderSet::new()
            .wake(Wake)
            .compute(Compute(7)),
    });
    let Err(denial) = result else {
        panic!("foreign dependency identity must fail before topology admission");
    };

    assert_eq!(
        denial.kind(),
        crate::facade::BridgeConditionalDenialKind::DeclarationCorrespondenceMismatch
    );
}
