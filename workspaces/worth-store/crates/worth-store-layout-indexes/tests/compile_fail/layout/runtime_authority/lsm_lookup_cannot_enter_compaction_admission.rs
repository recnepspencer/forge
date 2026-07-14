use worth_store_layout_indexes::{BaselineLsmCompactionAdmission, SelectedLsmLookup};

fn wrong_operation(selected: SelectedLsmLookup) {
    let _ = BaselineLsmCompactionAdmission::admit(selected);
}

fn main() {}
