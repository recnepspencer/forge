use worth_store_layout_indexes::SelectedBTreeLookup;

fn scalar_projection(selected: SelectedBTreeLookup) -> [u64; 8] {
    selected.request_identity().binding_words()
}

fn main() {}
