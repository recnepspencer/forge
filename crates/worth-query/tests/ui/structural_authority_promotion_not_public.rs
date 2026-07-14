use worth_query::facade::foundation::{AdvisoryStructuralUnique, LineageContinuity};

fn requires_lineage(_: LineageContinuity) {}

fn main() {
    let _: fn(AdvisoryStructuralUnique) = requires_lineage;
}
