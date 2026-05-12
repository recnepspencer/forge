use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "canonical_values",
        "canonical value carriers and representation-normalized scalar wrappers",
        "aspect contracts, mutation execution, or runtime storage layout",
    )
}
