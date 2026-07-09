use worth_query::facade::runtime::WorthQueryIntentAdvisoryDecision;

fn main() {
    let _ = WorthQueryIntentAdvisoryDecision {
        family: todo!(),
        entrypoint: todo!(),
        stage: "stage",
        message: String::new(),
        request_digest: String::new(),
        eligibility_digest: String::new(),
        decision_digest: String::new(),
    };
}
