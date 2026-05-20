use forge_query::facade::{
    ForgeQueryLowerRuntimeCertificationLane, ForgeQueryLowerRuntimeCertificationRow,
};

fn main() {
    let _ = ForgeQueryLowerRuntimeCertificationRow::new(
        ForgeQueryLowerRuntimeCertificationLane::CrossingsSurface,
        "artifact",
        "detail",
        "counters",
        None,
    );
}
