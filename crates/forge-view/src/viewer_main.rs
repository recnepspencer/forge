//! Forge Trace Viewer — native desktop binary entry point.
//!
//! Usage: `cargo run -p forge-view --bin forge-trace-viewer [traces_dir]`
//!
//! Opens a native window that watches the trace directory and
//! live-reloads whenever new JSON trace files appear.

use std::path::PathBuf;

fn main() -> eframe::Result {
    let trace_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("traces"));

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Forge Trace Viewer")
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Forge Trace Viewer",
        options,
        Box::new(move |cc| {
            Ok(Box::new(forge_view::trace::viewer::TraceViewerApp::new(cc, trace_dir)))
        }),
    )
}
