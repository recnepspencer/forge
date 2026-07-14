use worth_store_layout_indexes::evolution::migration::{
    LayoutBindingWitness, LayoutMigrationPlan,
};

fn worth(plan: &LayoutMigrationPlan, binding: LayoutBindingWitness) {
    let _ = plan.interruption_state_at(binding);
}

fn main() {}
