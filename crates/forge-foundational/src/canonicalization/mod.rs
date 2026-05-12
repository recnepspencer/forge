use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "canonical_ordering_and_equality",
        "stable ordering, equality, and digest-preparation basis vocabulary",
        "final digest algorithms or cryptographic receipt construction",
    )
}
