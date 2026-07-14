use worth_query::facade::runtime::{WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeGapRegistryRow, WorthQueryLowerRuntimeSeamKey};

fn main() {
    let _ = WorthQueryLowerRuntimeGapRegistryRow::new(
        WorthQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
        "worthd-seam",
        "worthd-shape",
        WorthQueryLowerRuntimeAuthorityOwner::Signal,
        "worthd-contract",
        "worthd-closeout",
    );
}
