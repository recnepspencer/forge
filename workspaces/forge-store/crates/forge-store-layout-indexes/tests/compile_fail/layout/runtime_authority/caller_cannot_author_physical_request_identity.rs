use forge_store_budgets::StorePhysicalRequestIdentity;

fn main() {
    let _ = StorePhysicalRequestIdentity::admit(1, 2, 3, 4, b"copied-key");
}
