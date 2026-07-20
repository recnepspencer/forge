use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_query::facade::{domain, foundation};

use super::conditional_node_contract::{conditional_node_result, dependency, ManualRefresh};
use super::installed_operation_fixture::{
    conditional_installation, conditional_workspace_with, GeometryDomain, ReadExecutionInput,
    ReadFamily, ReadVertex,
};

#[test]
fn fresh_bound_capabilities_mint_distinct_signal_attempt_identities() {
    let dependency = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    let node = conditional_node_result(
        "distinct-direct-attempts",
        dependency,
        domain::WorthQueryConditionalEvaluationCondition::on_demand(),
        domain::WorthQueryConditionalTrigger::on_demand::<ManualRefresh>(),
        domain::WorthQueryMaintenancePosture::OnDemandOnly,
    )
    .unwrap();
    let mut installation = conditional_installation(&node);
    installation.providers =
        worth_runtime_bridge::facade::BridgeConditionalProviderSet::new().trigger(RequestedTrigger);
    let versions = Arc::new(AtomicU64::new(0));
    let mut workspace = conditional_workspace_with(
        "distinct-direct-attempts",
        node,
        installation,
        AdvancingCompute(Arc::clone(&versions)),
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();

    let first = bind(&workspace, &installed)
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap();
    let second = bind(&workspace, &installed)
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap();

    assert_eq!(
        first.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
    assert_eq!(
        second.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
    assert_ne!(
        first.conditional_provenance()[0].signal_identity(),
        second.conditional_provenance()[0].signal_identity(),
        "two consumed bound capabilities are two Signal evaluation attempts"
    );
}

fn bind<'a>(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    installed: &domain::WorthQueryInstalledDomainHandle<GeometryDomain>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(installed, ReadVertex)
        .unwrap()
}

struct RequestedTrigger;

impl worth_runtime_bridge::facade::BridgeConditionalTriggerProvider for RequestedTrigger {
    fn requested(&self) -> bool {
        true
    }
}

struct AdvancingCompute(Arc<AtomicU64>);

impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>
    for AdvancingCompute
{
    fn compute(
        &self,
        _context: &domain::WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        let version = self.0.fetch_add(1, Ordering::SeqCst) + 1;
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                version,
            )]),
        ))
    }
}

fn observation_basis() -> foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone()
}
