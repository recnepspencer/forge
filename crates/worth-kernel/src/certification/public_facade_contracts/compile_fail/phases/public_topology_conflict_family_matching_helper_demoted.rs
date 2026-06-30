use worth_kernel::workload_composition::AdmittedTopologyConflictInput;

fn bypass<'a>(admitted: AdmittedTopologyConflictInput<'a>) {
    let _ = admitted
        .touched_closure()
        .matching_conflict_family_identities_for_contract(admitted.routing_contract());
}

fn main() {}
