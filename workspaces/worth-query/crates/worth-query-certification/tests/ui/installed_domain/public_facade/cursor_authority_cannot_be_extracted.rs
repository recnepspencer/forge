use worth_query::facade::installed::collection::WorthQueryCollectionCursor;

fn extract(cursor: WorthQueryCollectionCursor) -> u64 {
    let WorthQueryCollectionCursor {
        capability_identity,
        ..
    } = cursor;
    capability_identity
}

fn main() {}
