use worth_query::facade::{
    WorthQueryIntentConsumerInspection, WorthQueryIntentConsumerOutcomeClass,
};

fn main() {
    let _worthd = WorthQueryIntentConsumerInspection {
        intent_name: "",
        outcome_class: WorthQueryIntentConsumerOutcomeClass::Admitted,
        decision_trace_envelope: None,
        execution_provenance: None,
        fallback_stage: "admitted-decision",
        fallback_cause: "admitted_for_execution",
        fallback_detail: "",
    };
}
