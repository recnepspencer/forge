use worth_query::facade::{
    WorthQueryLowerRuntimeCertificationLane, WorthQueryLowerRuntimeCertificationRow,
};

fn main() {
    let _ = WorthQueryLowerRuntimeCertificationRow::new(
        WorthQueryLowerRuntimeCertificationLane::CrossingsSurface,
        "artifact",
        "detail",
        "counters",
        None,
    );
}
