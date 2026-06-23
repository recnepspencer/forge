#![allow(dead_code)]

use forge_server::{
    ForgeServer, ForgeServerProductOperationInput, ForgeServerProductOperationPayload,
};
use serde_json::json;

#[path = "../product_adapter_phase_nine/fixture.rs"]
mod product_fixture;

pub fn build_server() -> ForgeServer {
    product_fixture::build_server(vec![product_fixture::editor_registration(None, None)])
}

pub fn base_config() -> forge_server::ForgeServerConfig {
    product_fixture::base_config()
}

pub fn editor_registration() -> forge_server::ForgeServerProductApplicationAdapterRegistration {
    product_fixture::editor_registration(None, None)
}

pub fn direct_read(server: &ForgeServer) -> forge_server::ForgeServerCompletedProductOperation {
    product_fixture::completed(
        product_fixture::direct_session(server)
            .product_operations()
            .execute(
                ForgeServerProductOperationInput::new(
                    "product_editor.render",
                    ForgeServerProductOperationPayload::json("product-editor.render.v1", json!({})),
                )
                .with_basis_digest("basis:7"),
            ),
    )
}

pub fn direct_mutation(
    server: &ForgeServer,
) -> (
    forge_server::ForgeServerProductSession,
    forge_server::ForgeServerCompletedProductOperation,
) {
    let session = product_fixture::direct_session(server);
    let product_session =
        product_fixture::open_mutation_product_session(&session, "product_editor.apply", "basis:7");
    let completed = product_fixture::completed(
        session.product_operations().execute(
            ForgeServerProductOperationInput::new(
                "product_editor.apply",
                ForgeServerProductOperationPayload::json(
                    "product-editor.apply.v1",
                    json!({ "title": "Rename" }),
                ),
            )
            .with_basis_digest("basis:7")
            .with_product_session_identity(product_session.identity().as_str()),
        ),
    );
    (product_session, completed)
}
