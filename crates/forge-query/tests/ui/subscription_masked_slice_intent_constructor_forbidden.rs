use forge_query::facade::{QuerySubscriptionSliceKind, QuerySubscriptionSlicePart};

fn main() {
    let _masked_slice =
        QuerySubscriptionSlicePart::new(QuerySubscriptionSliceKind::AuthorizedProjection, 0);
}
