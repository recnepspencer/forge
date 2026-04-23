use forge_query::facade::{
    QuerySubscriptionSignalStrategyRequest, QuerySubscriptionSignalStrategyRequestKind,
};

fn main() {
    let _fabricated = QuerySubscriptionSignalStrategyRequest {
        request_kind: QuerySubscriptionSignalStrategyRequestKind::ExactDetailSignals,
        digest: String::new(),
    };
}
