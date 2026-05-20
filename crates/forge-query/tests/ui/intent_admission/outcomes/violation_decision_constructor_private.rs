use forge_query::facade::runtime::ForgeQueryIntentViolationDecision;

fn main() {
    let _ = ForgeQueryIntentViolationDecision {
        family: todo!(),
        entrypoint: todo!(),
        stage: "stage",
        message: String::new(),
        request_digest: String::new(),
        eligibility_digest: String::new(),
        decision_digest: String::new(),
    };
}
