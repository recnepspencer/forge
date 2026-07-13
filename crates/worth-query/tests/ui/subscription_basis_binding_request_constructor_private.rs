use worth_query::facade::runtime::{QuerySubscriptionBasisBindingRequest, QuerySubscriptionBasisBindingRequestKind};

fn main() {
    let _fabricated = QuerySubscriptionBasisBindingRequest {
        request_kind: QuerySubscriptionBasisBindingRequestKind::CurrentHead,
        digest: String::new(),
    };
}
