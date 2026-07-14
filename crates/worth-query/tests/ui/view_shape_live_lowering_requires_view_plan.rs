use worth_query::facade::foundation::{CanonicalQueryBundle, ResolvedSnapshotBasis};
use worth_query::facade::runtime::lower_view_shape_plan_to_live;

fn canonical() -> CanonicalQueryBundle {
    todo!()
}

fn basis() -> ResolvedSnapshotBasis {
    todo!()
}

fn main() {
    let _ = lower_view_shape_plan_to_live(&canonical(), basis(), None);
}
