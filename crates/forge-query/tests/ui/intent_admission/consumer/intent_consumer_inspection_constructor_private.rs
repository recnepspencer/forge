use forge_query::facade::{
    ForgeQueryIntentConsumerInspection, ForgeQueryIntentConsumerOutcomeClass,
};

fn main() {
    let _forged = ForgeQueryIntentConsumerInspection {
        intent_name: "",
        outcome_class: ForgeQueryIntentConsumerOutcomeClass::Admitted,
        decision_trace_envelope: None,
        execution_provenance: None,
        fallback_stage: "admitted-decision",
        fallback_cause: "admitted_for_execution",
        fallback_detail: "",
    };
}
