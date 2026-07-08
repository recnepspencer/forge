use forge_store_layout_indexes::{layout_declarations, S8LayoutReadmissionWitness};

fn main() {
    let family = layout_declarations().seed_family().family();
    let _ = S8LayoutReadmissionWitness::terminal_import(family);
}
