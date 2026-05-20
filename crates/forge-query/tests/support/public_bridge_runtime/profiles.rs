use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeSupportProfile,
};

pub fn public_graph_support_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::bridge_backed(
        "public-graph-subscription-activation",
        "public-graph-preview-basis",
        "public-graph-inspector-evidence",
    )
    .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Write,
        [ForgeQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["public-graph-write-authority"],
    ))
}
