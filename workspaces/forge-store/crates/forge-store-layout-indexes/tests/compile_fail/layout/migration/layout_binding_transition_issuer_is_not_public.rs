use forge_store_layout_indexes::evolution::migration::{LayoutBindingWitness, LayoutVersion};

fn forge(source: &LayoutBindingWitness, target: LayoutVersion) -> LayoutBindingWitness {
    LayoutBindingWitness::issue_transition(source, target)
}

fn main() {}
