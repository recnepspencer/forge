use forge_query::facade::{
    lower_view_shape_plan_to_live, CanonicalQueryBundle, ResolvedSnapshotBasis,
};

fn canonical() -> CanonicalQueryBundle {
    todo!()
}

fn basis() -> ResolvedSnapshotBasis {
    todo!()
}

fn main() {
    let _ = lower_view_shape_plan_to_live(&canonical(), basis(), None);
}
