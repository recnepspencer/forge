use worth_foundational::performance_api::performance_public_surface_inventory;
use worth_store_security::StoreKeyScope;

fn require_key_scope(_: StoreKeyScope) {}

fn main() {
    let foundational_surface = performance_public_surface_inventory()[0];
    require_key_scope(foundational_surface);
}
