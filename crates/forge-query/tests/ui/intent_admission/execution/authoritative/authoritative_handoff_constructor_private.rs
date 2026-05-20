use forge_query::facade::runtime::ForgeQueryAuthoritativeIntentExecutionHandoff;

fn main() {
    let _ = ForgeQueryAuthoritativeIntentExecutionHandoff {
        declaration: todo!(),
        request_digest: String::new(),
        eligibility_digest: String::new(),
        decision_digest: String::new(),
        handoff_digest: String::new(),
    };
}
