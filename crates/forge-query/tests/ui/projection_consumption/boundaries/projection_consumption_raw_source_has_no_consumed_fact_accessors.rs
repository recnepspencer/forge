use forge_query::facade::ConsumedProjectionFactSet;
use forge_relational::facade::grouped_truth::RelationalAuthoritativeRowSetArtifact;

fn expect_fact_set(_: &ConsumedProjectionFactSet) {}

fn misuse_raw_row_set(row_set: RelationalAuthoritativeRowSetArtifact) {
    expect_fact_set(&row_set);
    let _receipt = row_set.issue_receipt();
}

fn main() {
    let _ = misuse_raw_row_set;
}
