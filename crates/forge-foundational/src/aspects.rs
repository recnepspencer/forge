use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "aspect_state_and_patches",
        "aspect contracts, state-map vocabulary, mask law, and patch vocabulary",
        "domain-owned truth mutation or persistence engines",
    )
}
