use worth_kernel::workload_composition::{
    admit_topology_conflict_input, AdmittedTopologyConflictInput,
};

fn bypass<'a>(admitted: AdmittedTopologyConflictInput<'a>) {
    let _ = admit_topology_conflict_input(admitted.routing_contract());
}

fn main() {}
