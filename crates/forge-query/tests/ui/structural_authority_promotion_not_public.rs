use forge_query::facade::{AdvisoryStructuralUnique, LineageContinuity};

fn requires_lineage(_: LineageContinuity) {}

fn main() {
    let _: fn(AdvisoryStructuralUnique) = requires_lineage;
}
