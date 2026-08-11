use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectRunCleanupFailure, WorthQueryDirectRunCleanupReceipt,
    WorthQueryDirectRunTerminal,
};

fn finish(
    terminal: WorthQueryDirectRunTerminal,
) -> Result<WorthQueryDirectRunCleanupReceipt, WorthQueryDirectRunCleanupFailure> {
    match terminal.cleanup() {
        Ok(receipt) => {
            let inspection = receipt.inspection();
            let _ = inspection.run_identity();
            let _ = inspection.logical_run_identity();
            let _ = inspection.terminal();
            let _ = inspection.disposition();
            let _ = inspection.provider_session_identity();
            let _ = inspection.resource_plan_identity();
            let _ = inspection.capacity_scope();
            let _ = inspection.released_reservation_count();
            let _ = inspection.resources_released();
            let _ = inspection.provider_work();
            let _ = inspection.counters();
            Ok(receipt)
        }
        Err(failure) => failure.retry(),
    }
}

fn restore(failure: WorthQueryDirectRunCleanupFailure) -> WorthQueryDirectRunTerminal {
    failure.into_terminal()
}

fn main() {}
