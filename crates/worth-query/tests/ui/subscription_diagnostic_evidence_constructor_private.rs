use worth_query::facade::{
    QuerySubscriptionDiagnosticEvidence, QuerySubscriptionDiagnosticOutcome,
    QuerySubscriptionDiagnosticStage,
};

fn main() {
    let _fabricated = QuerySubscriptionDiagnosticEvidence {
        stage: QuerySubscriptionDiagnosticStage::Declaration,
        outcome: QuerySubscriptionDiagnosticOutcome::Denied,
        reason: String::new(),
        source_digest: String::new(),
        counter_digest: String::new(),
        digest: String::new(),
    };
}
