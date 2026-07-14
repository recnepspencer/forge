use worth_store_layout_indexes::evolution::migration::{LayoutBindingWitness, LayoutVersion};

fn worth(source: &LayoutBindingWitness, target: LayoutVersion) -> LayoutBindingWitness {
    LayoutBindingWitness::issue_transition(source, target)
}

fn main() {}
