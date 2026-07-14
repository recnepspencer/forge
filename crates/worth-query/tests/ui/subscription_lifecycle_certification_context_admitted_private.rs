use worth_query::facade::certification::SubscriptionLifecycleCertificationContext;

fn main() {
    let _ = SubscriptionLifecycleCertificationContext::admitted(
        "query",
        "family",
        "equivalence",
        "policy",
        "tenant",
        "relationship",
        "view",
        "basis",
    );
}
