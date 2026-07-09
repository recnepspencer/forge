use worth_query::facade::runtime::{
    worth_query_domain, BridgePreviewSessionIdentity, WorthQueryIntentDeclaration, WorthQueryIntentInput,
};

fn workflow_common_lane() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );

    let _plan = worth_query_domain("worth.spatial")
        .for_intent(&declaration)
        .plans_preview_mutation(
            "topology.preview_mutation",
            BridgePreviewSessionIdentity::from_stable_name("preview-session:77"),
        )
        .because("promotion-eligible preview can plan a bounded mutation workflow")
        .materialize();
}

fn main() {}
