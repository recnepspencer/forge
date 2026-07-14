use worth_query::facade::foundation::HistoricalMaterializationPathMetadata;
use worth_query::facade::policy::QueryBasisResultBundle;

fn takes_bundle(_: QueryBasisResultBundle) {}

fn main() {
    let metadata: HistoricalMaterializationPathMetadata = unsafe { std::mem::zeroed() };
    takes_bundle(metadata);
}
