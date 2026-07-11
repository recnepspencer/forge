use forge_foundational::performance_api::performance_public_surface_inventory;
use forge_store_security::StoreCurrentKeyScopeWitness;

fn require_key_scope_witness(_: StoreCurrentKeyScopeWitness) {}

fn main() {
    let foundational_surface = performance_public_surface_inventory()[0];
    require_key_scope_witness(foundational_surface);
}
