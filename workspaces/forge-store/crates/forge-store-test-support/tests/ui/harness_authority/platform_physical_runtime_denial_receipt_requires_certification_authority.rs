use forge_store_physical_format::{PlatformPhysicalAppendReport, PlatformPhysicalRuntimeReceipt};

fn main() {
    let report: PlatformPhysicalAppendReport = todo!();
    let _ = PlatformPhysicalRuntimeReceipt::from_append_hidden_scan_denial(report);
}
