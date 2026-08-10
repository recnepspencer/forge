pub(in crate::tests::phase1_api) const OBSERVER_SOURCE: &str = concat!(
    include_str!("../../../data/graph/runtime/observer.rs"),
    include_str!("../../../data/graph/runtime/observer/branches.rs"),
    include_str!("../../../data/graph/runtime/observer/explanation.rs"),
    include_str!("../../../data/graph/runtime/observer/lineage.rs"),
    include_str!("../../../data/graph/runtime/observer/materialization.rs"),
    include_str!("../../../data/graph/runtime/observer/replay.rs"),
    include_str!("../../../data/graph/runtime/observer/summary.rs"),
);
