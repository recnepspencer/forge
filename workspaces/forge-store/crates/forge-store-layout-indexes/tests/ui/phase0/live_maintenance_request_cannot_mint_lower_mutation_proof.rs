use forge_store_layout_indexes::S8LiveMaintenanceRequest;

fn attempt_to_mint_lower_mutation_proof(request: S8LiveMaintenanceRequest) {
    let _ = request.prove_wal_before_data();
}

fn main() {}
