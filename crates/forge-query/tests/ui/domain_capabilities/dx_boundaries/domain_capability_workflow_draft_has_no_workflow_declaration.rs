use forge_query::facade::runtime::{
    forge_query_domain, BridgePreviewSessionIdentity, ForgeQueryIntentDeclaration, ForgeQueryIntentInput,
};

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        ForgeQueryIntentInput::object([("edge", ForgeQueryIntentInput::string("e-1"))]),
    );

    let _ = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .plans_preview_mutation(
            "topology.preview_mutation",
            BridgePreviewSessionIdentity::from_stable_name("preview-session:77"),
        )
        .workflow_declaration();
}
