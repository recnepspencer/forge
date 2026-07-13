use worth_query::facade::runtime::{WorthQueryLowerRuntimeArtifactStrength, WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeCrossingClassification, WorthQueryLowerRuntimeCrossingRow, WorthQueryLowerRuntimeRouteKind, WorthQueryLowerRuntimeSeamKey};

fn main() {
    let _ = WorthQueryLowerRuntimeCrossingRow::new(
        WorthQueryLowerRuntimeSeamKey::ComposeRead,
        "worthd-capability",
        "worthd-seam",
        WorthQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse,
        WorthQueryLowerRuntimeRouteKind::RoutePlanning,
        WorthQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        WorthQueryLowerRuntimeArtifactStrength::TypedReceipt,
        "Worthd action",
    );
}
