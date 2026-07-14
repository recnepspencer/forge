use forge_store_layout_indexes::LiveMaintenanceRequest;

fn attempt_to_mint_lower_mutation_proof(request: LiveMaintenanceRequest) {
    let _ = request.prove_wal_before_data();
}

fn main() {}
