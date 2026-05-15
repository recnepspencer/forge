use forge_query::facade::runtime::ForgeQueryIntentAdvisoryDecision;

fn main() {
    let _ = ForgeQueryIntentAdvisoryDecision {
        family: todo!(),
        entrypoint: todo!(),
        stage: "stage",
        message: String::new(),
        request_digest: String::new(),
        eligibility_digest: String::new(),
        decision_digest: String::new(),
    };
}
