use forge_store_layout_indexes::access_planning;
use forge_store_physical_format::PhysicalReference;

fn main() {
    let family = todo!();
    let catalog = todo!();
    let raw_reference: PhysicalReference = todo!();
    let _ = access_planning()
        .admit_btree_publication_materialization(family, catalog, raw_reference);
}
