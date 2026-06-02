use topology::facade::{
    build_derived_fallback_report, build_derived_invalidation_report,
    build_derived_read_diagnostics, build_derived_rebuild_report,
};

fn main() {
    let _ = build_derived_read_diagnostics;
    let _ = build_derived_invalidation_report;
    let _ = build_derived_rebuild_report;
    let _ = build_derived_fallback_report;
}
