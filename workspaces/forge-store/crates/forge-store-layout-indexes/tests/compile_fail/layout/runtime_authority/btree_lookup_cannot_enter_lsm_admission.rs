use forge_store_layout_indexes::{BaselineLsmLookupAdmission, SelectedBTreeLookup};

fn wrong_machine(selected: SelectedBTreeLookup) {
    let _ = BaselineLsmLookupAdmission::admit(selected);
}
