use worth_query::facade::certification::{WorthQueryLowerRuntimeCertificationLane, WorthQueryLowerRuntimeCertificationRow};

fn main() {
    let _ = WorthQueryLowerRuntimeCertificationRow::new(
        WorthQueryLowerRuntimeCertificationLane::CrossingsSurface,
        "artifact",
        "detail",
        "counters",
        None,
    );
}
