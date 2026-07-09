use worth_query::facade::runtime::WorthQueryIntentViolationDecision;

fn main() {
    let _ = WorthQueryIntentViolationDecision {
        family: todo!(),
        entrypoint: todo!(),
        stage: "stage",
        message: String::new(),
        request_digest: String::new(),
        eligibility_digest: String::new(),
        decision_digest: String::new(),
    };
}
