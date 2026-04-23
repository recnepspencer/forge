use forge_query::facade::SubscriptionLifecycleCertificationContext;

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
