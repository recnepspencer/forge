use forge_query::facade::QueryDeliveryError;

fn main() {
    let error: QueryDeliveryError = todo!();
    let _ = error.source_digest();
}
