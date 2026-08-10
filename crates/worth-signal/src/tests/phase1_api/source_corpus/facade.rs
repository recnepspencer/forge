pub(in crate::tests::phase1_api) const FACADE_SOURCE: &str = concat!(
    include_str!("../../../facade.rs"),
    include_str!("../../../facade/adapters.rs"),
    include_str!("../../../facade/advanced.rs"),
    include_str!("../../../facade/core.rs"),
    include_str!("../../../facade/diagnostics.rs"),
    include_str!("../../../facade/history.rs"),
    include_str!("../../../facade/integration.rs"),
    include_str!("../../../facade/runtime.rs"),
    include_str!("../../../facade/schema.rs"),
    include_str!("../../../facade/specialist.rs"),
);
