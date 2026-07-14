use forge_store_layout_indexes::evolution::migration::{
    LayoutBindingWitness, LayoutMigrationPlan,
};

fn forge(plan: &LayoutMigrationPlan, binding: LayoutBindingWitness) {
    let _ = plan.interruption_state_at(binding);
}

fn main() {}
