use super::*;
use crate::facade::{PlannedChangeStreamWindow, StreamWindowDeliveryResult};
use crate::routing::canonicalization::digest_string;

mod certification_bundle;
mod execution;
mod native_windows;
pub(in crate::harness::adapter::adapter_impl) mod terminal_report_export;

#[cfg(test)]
mod typed_certification_tests;

pub(super) use execution::{execute_stream_request, StreamHarnessExecution};
pub(super) use native_windows::{NativeStreamCommitWindow, StreamHarnessTarget};

fn routing_digest(result: &StreamWindowDeliveryResult) -> String {
    digest_string(
        "stream-routing-digest",
        &result
            .route_results()
            .iter()
            .map(|entry| entry.result_summary().route_identity().as_str())
            .collect::<Vec<_>>()
            .join("|"),
    )
    .to_string()
}

fn pressure_report(
    window: &PlannedChangeStreamWindow,
) -> crate::stream::BackpressureDecisionRecord {
    crate::stream::BackpressureDecisionRecord::classify(window)
}
