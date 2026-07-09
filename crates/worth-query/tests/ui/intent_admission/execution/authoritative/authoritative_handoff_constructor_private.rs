use worth_query::facade::runtime::WorthQueryAuthoritativeIntentExecutionHandoff;

fn main() {
    let _ = WorthQueryAuthoritativeIntentExecutionHandoff {
        declaration: todo!(),
        request_digest: String::new(),
        eligibility_digest: String::new(),
        decision_digest: String::new(),
        handoff_digest: String::new(),
    };
}
