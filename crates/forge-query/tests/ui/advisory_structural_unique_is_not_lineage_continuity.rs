use forge_query::facade::{AdvisoryStructuralUnique, LineageContinuity};

fn expects_lineage(_: LineageContinuity) {}

fn main() {
    let _: fn(AdvisoryStructuralUnique) = expects_lineage;
}
