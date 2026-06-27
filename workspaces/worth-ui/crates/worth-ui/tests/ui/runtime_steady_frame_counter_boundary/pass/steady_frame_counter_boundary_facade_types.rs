use worth_ui::facade::{
    WorthUiFrameReportMaterializationBoundary, WorthUiSteadyFrameCounterBoundary,
    WorthUiSteadyFrameDiagnosticPolicy, WorthUiSteadyFrameReportPlanner,
};

fn main() {
    let _builder = WorthUiSteadyFrameCounterBoundary::for_active_plan(42)
        .minimal_diagnostics()
        .with_capture_richness(worth_ui::facade::WorthUiCounterCaptureRichness::Standard);
    let _policy = WorthUiSteadyFrameDiagnosticPolicy::Minimal;
    let plan = WorthUiSteadyFrameReportPlanner::support_report();
    let _boundary = WorthUiFrameReportMaterializationBoundary::SupportExpansion;
    let _ = plan;
}
