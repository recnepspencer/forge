fn main() {
    let _ = forge_store::CompactionCutoverWitness {
        retained_basis_label: String::from("basis-a"),
        compaction_product_id: String::from("product-a"),
    };
}
