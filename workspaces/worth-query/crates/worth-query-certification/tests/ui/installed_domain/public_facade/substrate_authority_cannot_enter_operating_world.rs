use worth_query::facade::{foundation, runtime};

fn raw_query_basis_cannot_enter(
    workspace: &runtime::WorthQueryWorkspace,
    basis: foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness>,
) {
    let _ = workspace.observe_operating_world(basis);
}

fn signal_graph_cannot_enter(
    workspace: &runtime::WorthQueryWorkspace,
    graph: worth_signal::facade::SignalGraph,
) {
    let _ = workspace.observe_operating_world(graph);
}

fn runtime_bridge_cannot_enter(
    workspace: &runtime::WorthQueryWorkspace,
    bridge: worth_runtime_bridge::facade::RuntimeBridge,
) {
    let _ = workspace.observe_operating_world(bridge);
}

fn relational_identity_cannot_enter(
    workspace: &runtime::WorthQueryWorkspace,
    identity: worth_relational::facade::grouped_truth::RelationalRowIdentity,
) {
    let _ = workspace.observe_operating_world(identity);
}

fn main() {}
