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

#[allow(dead_code)]
pub fn public_verified_relation_profile(operation_family: &str) -> ForgeQueryRuntimeSupportProfile {
    public_graph_support_profile().with_bridge_backed_verification_support(
        operation_family,
        "direct_relation_identity",
        true,
        true,
        None,
    )
}

#[allow(dead_code)]
pub fn public_multi_verified_relation_profile() -> ForgeQueryRuntimeSupportProfile {
    ["update_existing_verified", "delete_existing_verified"]
        .into_iter()
        .fold(
            public_graph_support_profile(),
            |profile, operation_family| {
                profile.with_bridge_backed_verification_support(
                    operation_family,
                    "direct_relation_identity",
                    true,
                    true,
                    None,
                )
            },
        )
}
