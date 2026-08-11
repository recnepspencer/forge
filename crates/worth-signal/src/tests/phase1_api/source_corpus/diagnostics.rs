pub(in crate::tests::phase1_api) const EXECUTION_FLOW_SOURCE: &str =
    include_str!("../../../diagnostics/runtime/execution_flow.rs");
pub(in crate::tests::phase1_api) const HISTORY_SOURCE: &str =
    include_str!("../../../diagnostics/inspection/history.rs");
pub(in crate::tests::phase1_api) const RECORDER_SOURCE: &str = concat!(
    include_str!("../../../diagnostics/runtime/recorder.rs"),
    include_str!("../../../diagnostics/runtime/recorder/artifacts.rs"),
    include_str!("../../../diagnostics/runtime/recorder/branching.rs"),
    include_str!("../../../diagnostics/runtime/recorder/events.rs"),
    include_str!("../../../diagnostics/runtime/recorder/failure.rs"),
    include_str!("../../../diagnostics/runtime/recorder/snapshots.rs"),
);
pub(in crate::tests::phase1_api) const SUMMARY_SOURCE: &str = concat!(
    include_str!("../../../diagnostics/model/summary.rs"),
    include_str!("../../../diagnostics/model/summary/execution.rs"),
    include_str!("../../../diagnostics/model/summary/explanation.rs"),
    include_str!("../../../diagnostics/model/summary/graph.rs"),
    include_str!("../../../diagnostics/model/summary/history.rs"),
    include_str!("../../../diagnostics/model/summary/plan.rs"),
    include_str!("../../../diagnostics/model/summary/temporal.rs"),
);
