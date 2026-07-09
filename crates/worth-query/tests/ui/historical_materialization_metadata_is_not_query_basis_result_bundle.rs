use worth_query::facade::{HistoricalMaterializationPathMetadata, QueryBasisResultBundle};

fn takes_bundle(_: QueryBasisResultBundle) {}

fn main() {
    let metadata: HistoricalMaterializationPathMetadata = unsafe { std::mem::zeroed() };
    takes_bundle(metadata);
}
