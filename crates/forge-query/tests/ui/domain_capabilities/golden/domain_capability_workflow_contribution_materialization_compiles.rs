use forge_query::facade::runtime::{
    forge_query_domain, BridgePreviewSessionIdentity, ForgeQueryIntentDeclaration, ForgeQueryIntentInput,
};

fn workflow_common_lane() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        ForgeQueryIntentInput::object([("edge", ForgeQueryIntentInput::string("e-1"))]),
    );

    let _plan = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .plans_preview_mutation(
            "topology.preview_mutation",
            BridgePreviewSessionIdentity::from_stable_name("preview-session:77"),
        )
        .because("promotion-eligible preview can plan a bounded mutation workflow")
        .materialize();
}

fn main() {}
