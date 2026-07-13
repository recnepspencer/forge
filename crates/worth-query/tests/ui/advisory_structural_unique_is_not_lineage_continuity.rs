use worth_query::facade::foundation::{AdvisoryStructuralUnique, LineageContinuity};

fn expects_lineage(_: LineageContinuity) {}

fn main() {
    let _: fn(AdvisoryStructuralUnique) = expects_lineage;
}
