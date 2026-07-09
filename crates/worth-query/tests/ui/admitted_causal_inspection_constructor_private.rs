use worth_query::facade::{
    AdmittedCausalInspection, CausalDecisionTraceIndex, CausalInspectionAdmissionCounters,
    CausalInspectionAdmissionDecision, CausalInspectionAdmissionReceipt,
    CausalInspectionAdmissionSubject,
};

fn main() {
    let subject: CausalInspectionAdmissionSubject = todo!();
    let decision: CausalInspectionAdmissionDecision = todo!();
    let decision_trace: CausalDecisionTraceIndex = todo!();
    let receipt: CausalInspectionAdmissionReceipt = todo!();
    let counters: CausalInspectionAdmissionCounters = todo!();

    let _ = AdmittedCausalInspection {
        subject,
        decision,
        decision_trace,
        receipt,
        counters,
        admitted_inspection_digest: "worthd-admission".to_string(),
    };
}
