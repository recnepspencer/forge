use worth_query::facade::runtime::WorthQueryEffectTriggeredIntentExecutionHandoff;

fn main() {
    let _ = WorthQueryEffectTriggeredIntentExecutionHandoff {
        declaration: todo!(),
        request_digest: String::new(),
        eligibility_digest: String::new(),
        decision_digest: String::new(),
        handoff_digest: String::new(),
    };
}
