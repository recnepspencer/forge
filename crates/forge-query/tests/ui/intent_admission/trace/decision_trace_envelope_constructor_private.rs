use forge_query::facade::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionFamily,
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryIntentDecisionTraceEnvelopeKind,
    ForgeQueryIntentDecisionTraceRow,
};

fn main() {
    let _forged = ForgeQueryIntentDecisionTraceEnvelope {
        kind: ForgeQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution,
        family: ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        rows: Vec::<ForgeQueryIntentDecisionTraceRow>::new(),
        trace_digest: String::new(),
    };
}
