use worth_kernel::workload_composition::BooleanChainIntegrationHandoff;
use worth_topo::facade::PlanarBooleanLoopValidatorRegistrationPlan;

fn require_boolean_chain(_: &BooleanChainIntegrationHandoff) {}

fn main() {
    let validators = PlanarBooleanLoopValidatorRegistrationPlan::phase_2();
    let local_graph_legality = validators
        .validators()
        .iter()
        .find(|row| row.governs_topology_legality())
        .expect("phase 2 exposes topology-legality validator rows");
    require_boolean_chain(local_graph_legality);
}
