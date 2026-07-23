use worth_query::facade::installed::collection::WorthQueryCollectionPatch;

fn extract(patch: WorthQueryCollectionPatch) -> u64 {
    let WorthQueryCollectionPatch {
        maintenance_ordinal,
        ..
    } = patch;
    maintenance_ordinal
}

fn main() {}
