use topology::facade::{
    topology_grouped_operator_neighborhood, topology_operator_continuation_target,
    topology_operator_contribution_workflow, topology_operator_signal_workflow,
    TopologyOperatorWorkflowHandleExt,
};

fn main() {
    let _ = topology_grouped_operator_neighborhood;
    let _ = topology_operator_continuation_target;
    let _ = topology_operator_contribution_workflow;
    let _ = topology_operator_signal_workflow;
    let _ = std::any::type_name::<dyn TopologyOperatorWorkflowHandleExt>();
}
