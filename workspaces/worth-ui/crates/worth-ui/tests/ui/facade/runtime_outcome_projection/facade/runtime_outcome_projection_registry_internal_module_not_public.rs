use worth_ui::capability::registry::runtime_outcome_projection;

fn main() {
    let _ = core::any::type_name::<
        runtime_outcome_projection::RuntimeOutcomeProjectionRegistry,
    >();
}
