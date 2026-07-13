use worth_query::facade::runtime::QueryDeliveryError;

fn main() {
    let error: QueryDeliveryError = todo!();
    let _ = error.source_digest();
}
