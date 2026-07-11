use forge_store_layout_indexes::S8BootstrapLayoutCatalog;

fn main() {
    let _ = S8BootstrapLayoutCatalog {
        identity: todo!(),
        discovery_layout: todo!(),
        root_entry_count: 1,
        segment_count: 1,
        page_slot_count: 1,
        extent_count: 1,
        allocation_class_count: 1,
        free_space_count: 1,
    };
}
