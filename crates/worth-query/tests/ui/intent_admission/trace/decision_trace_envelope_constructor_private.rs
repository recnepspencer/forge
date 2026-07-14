use worth_query::facade::runtime::{WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionFamily, WorthQueryIntentDecisionTraceEnvelope, WorthQueryIntentDecisionTraceEnvelopeKind, WorthQueryIntentDecisionTraceRow};

fn main() {
    let _worthd = WorthQueryIntentDecisionTraceEnvelope {
        kind: WorthQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution,
        family: WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        rows: Vec::<WorthQueryIntentDecisionTraceRow>::new(),
        trace_digest: String::new(),
    };
}
