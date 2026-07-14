use worth_query::facade::runtime::{QuerySubscriptionSliceKind, QuerySubscriptionSlicePart};

fn main() {
    let _masked_slice =
        QuerySubscriptionSlicePart::new(QuerySubscriptionSliceKind::AuthorizedProjection, 0);
}
