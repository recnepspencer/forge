use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeArtifactStrength, ForgeQueryLowerRuntimeAuthorityOwner,
    ForgeQueryLowerRuntimeCrossingClassification, ForgeQueryLowerRuntimeCrossingRow,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeSeamKey,
};

fn main() {
    let _ = ForgeQueryLowerRuntimeCrossingRow::new(
        ForgeQueryLowerRuntimeSeamKey::ComposeRead,
        "forged-capability",
        "forged-seam",
        ForgeQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        ForgeQueryLowerRuntimeArtifactStrength::TypedReceipt,
        "forged action",
    );
}
