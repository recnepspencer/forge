use serde_json::{json, Value};
use worth_store_recovery_runtime::{RecoveryReportEnvelope, RecoveryReportOutcome};

use super::super::c7_crash_campaign::recovery::C8RecoveryEvidence;
use crate::physical_work_evidence::{hex, process_value};

pub(super) fn value(evidence: &C8RecoveryEvidence) -> Value {
    let marker = evidence.marker();
    let report = evidence.report();
    json!({
        "process": process_value(evidence.process()),
        "elapsed_ms": evidence.elapsed().as_millis(),
        "runtime": marker.runtime(),
        "store": hex(&marker.store()),
        "root_generation": marker.root_generation(),
        "report": report_value(report),
    })
}

fn report_value(report: &RecoveryReportEnvelope) -> Value {
    let counters = report.counters();
    json!({
        "outcome": outcome_label(report.outcome()),
        "store": report.store_identity().map(|store| hex(&store)),
        "root_generation": report.root_generation(),
        "denial_cause": report.denial_cause().map(|cause| format!("{cause:?}")),
        "counters": {
            "recovery_effects": counters.recovery_effects(),
            "cleanup_performed": counters.cleanup_performed(),
            "cleanup_deferred": counters.cleanup_deferred(),
            "peak_recovery_bytes": counters.peak_recovery_bytes(),
        },
    })
}

const fn outcome_label(outcome: RecoveryReportOutcome) -> &'static str {
    match outcome {
        RecoveryReportOutcome::Recovered => "recovered",
        RecoveryReportOutcome::Refused => "refused",
        RecoveryReportOutcome::Blocked => "blocked",
        RecoveryReportOutcome::PublicationIndeterminate => "publication-indeterminate",
    }
}
