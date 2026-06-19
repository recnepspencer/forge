#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use forge_server::{
    ForgeServer, ForgeServerCompatibilityPreparedRequest, ForgeServerForgeNativeSession,
    ForgeServerProductAdapterExecutionError, ForgeServerProductApplicationAdapter,
    ForgeServerProductApplicationAdapterRegistration, ForgeServerProductOperationDeclaration,
    ForgeServerProductOperationDenial, ForgeServerProductOperationErrorMaps,
    ForgeServerProductOperationInput, ForgeServerProductOperationPayload,
    ForgeServerProductOperationSuccess, ForgeServerProductOperationSupportSnapshot,
    ForgeServerProductSession, ForgeServerProductSessionCreationRequest,
};
use serde_json::{json, Value};

#[path = "../product_adapter_phase_nine/fixture.rs"]
mod product_adapter_phase_nine_fixture;

#[derive(Clone, Debug)]
pub struct StatefulEditorLikeBackend {
    state: Arc<Mutex<EditorLikeState>>,
}

impl StatefulEditorLikeBackend {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(EditorLikeState::default())),
        }
    }

    pub fn basis_digest(&self) -> String {
        self.state.lock().expect("editor state lock").basis_digest()
    }

    pub fn title(&self) -> String {
        self.state.lock().expect("editor state lock").title.clone()
    }

    pub fn registration(&self) -> ForgeServerProductApplicationAdapterRegistration {
        ForgeServerProductApplicationAdapterRegistration::new(
            "stateful-editor-like",
            Arc::new(self.clone()),
        )
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_read(
                "product_editor.render",
                "product-editor.render.v1",
                forge_server::ForgeServerProductOperationBasisKind::DurableProductDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted("render-ready"),
            ),
        ))
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_read(
                "product_editor.select",
                "product-editor.select.v1",
                forge_server::ForgeServerProductOperationBasisKind::DurableProductDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted("select-ready"),
            ),
        ))
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_read(
                "product_editor.available_actions",
                "product-editor.actions.v1",
                forge_server::ForgeServerProductOperationBasisKind::DurableProductDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted("actions-ready"),
            ),
        ))
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_mutation(
                "product_editor.apply",
                "product-editor.apply.v1",
                forge_server::ForgeServerProductOperationBasisKind::ProductSessionDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted("apply-ready"),
                "draft",
            ),
        ))
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_mutation(
                "product_editor.finalize",
                "product-editor.finalize.v1",
                forge_server::ForgeServerProductOperationBasisKind::ProductSessionDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted("finalize-ready"),
                "draft",
            ),
        ))
    }
}

impl ForgeServerProductApplicationAdapter for StatefulEditorLikeBackend {
    fn execute(
        &self,
        operation: &forge_server::ForgeServerScheduledProductOperation,
    ) -> Result<ForgeServerProductOperationSuccess, ForgeServerProductAdapterExecutionError> {
        let mut state = self.state.lock().expect("editor state lock");
        let plan = operation.plan();
        let body = plan.payload().body();
        let request_basis = plan
            .operation_admission()
            .operation_request()
            .identity()
            .basis_digest()
            .unwrap_or("none");
        match plan.declaration().operation_name() {
            "product_editor.render" => Ok(ForgeServerProductOperationSuccess::new(
                "product_editor.render",
                format!(
                    "render:{request_basis}:{}:{}",
                    state.basis_digest(),
                    state.title
                ),
            )),
            "product_editor.select" => {
                let node = body.get("node").and_then(Value::as_str).unwrap_or("none");
                Ok(ForgeServerProductOperationSuccess::new(
                    "product_editor.select",
                    format!("select:{node}:{}:{}", state.title, state.basis_digest()),
                ))
            }
            "product_editor.available_actions" => {
                let actions = if state.title == "Untitled" {
                    "apply"
                } else {
                    "apply,finalize"
                };
                Ok(ForgeServerProductOperationSuccess::new(
                    "product_editor.available_actions",
                    format!("actions:{actions}:{}", state.basis_digest()),
                ))
            }
            "product_editor.apply" => {
                let title = body
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or("Untitled");
                state.title = title.to_string();
                state.revision += 1;
                Ok(ForgeServerProductOperationSuccess::new(
                    "product_editor.apply",
                    state.basis_digest(),
                ))
            }
            "product_editor.finalize" => {
                if body.get("confirm").and_then(Value::as_bool) != Some(true) {
                    return Err(ForgeServerProductAdapterExecutionError::denied(
                        ForgeServerProductOperationDenial::new(
                            "product.finalize.confirm_required",
                            "finalize requires explicit confirmation",
                        ),
                    ));
                }
                state.revision += 1;
                Ok(ForgeServerProductOperationSuccess::new(
                    "product_editor.finalize",
                    format!("finalized:{}", state.basis_digest()),
                ))
            }
            operation_name => Err(ForgeServerProductAdapterExecutionError::failed(
                "unsupported_operation",
                format!("unsupported editor-like operation `{operation_name}`"),
            )),
        }
    }
}

pub fn build_server(backend: &StatefulEditorLikeBackend) -> ForgeServer {
    product_adapter_phase_nine_fixture::build_server(vec![backend.registration()])
}

pub fn direct_session(server: &ForgeServer) -> ForgeServerForgeNativeSession {
    product_adapter_phase_nine_fixture::direct_session(server)
}

pub fn open_mutation_session(
    session: &ForgeServerForgeNativeSession,
    basis_digest: &str,
    operation_name: &str,
) -> ForgeServerProductSession {
    session
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation(operation_name)
                .with_basis_digest(basis_digest)
                .with_expiry_seconds(300),
        )
        .expect("mutation session should open")
}

pub fn prepared_read_request(
    server: &ForgeServer,
    operation_name: &str,
    basis_digest: Option<&str>,
) -> ForgeServerCompatibilityPreparedRequest {
    product_adapter_phase_nine_fixture::prepared_read_request(server, operation_name, basis_digest)
}

pub fn prepared_mutation_request(
    server: &ForgeServer,
    operation_name: &str,
    basis_digest: Option<&str>,
) -> ForgeServerCompatibilityPreparedRequest {
    product_adapter_phase_nine_fixture::prepared_mutation_request(
        server,
        operation_name,
        basis_digest,
    )
}

pub fn render_payload() -> ForgeServerProductOperationPayload {
    ForgeServerProductOperationPayload::json("product-editor.render.v1", json!({}))
}

pub fn select_payload(node: &str) -> ForgeServerProductOperationPayload {
    ForgeServerProductOperationPayload::json("product-editor.select.v1", json!({ "node": node }))
}

pub fn actions_payload() -> ForgeServerProductOperationPayload {
    ForgeServerProductOperationPayload::json("product-editor.actions.v1", json!({}))
}

pub fn apply_payload(title: &str) -> ForgeServerProductOperationPayload {
    ForgeServerProductOperationPayload::json("product-editor.apply.v1", json!({ "title": title }))
}

pub fn finalize_payload(confirm: bool) -> ForgeServerProductOperationPayload {
    ForgeServerProductOperationPayload::json(
        "product-editor.finalize.v1",
        json!({ "confirm": confirm }),
    )
}

pub fn direct_read(
    session: &ForgeServerForgeNativeSession,
    operation_name: &str,
    payload: ForgeServerProductOperationPayload,
    basis_digest: &str,
) -> forge_server::ForgeServerCompletedProductOperation {
    session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new(operation_name, payload)
                .with_basis_digest(basis_digest),
        )
        .expect("direct editor-like read should complete")
}

pub fn direct_mutation(
    session: &ForgeServerForgeNativeSession,
    operation_name: &str,
    payload: ForgeServerProductOperationPayload,
    basis_digest: &str,
    product_session: &ForgeServerProductSession,
) -> Result<
    forge_server::ForgeServerCompletedProductOperation,
    forge_server::ForgeServerProductOperationSurfaceDenial,
> {
    session.product_operations().execute(
        ForgeServerProductOperationInput::new(operation_name, payload)
            .with_basis_digest(basis_digest)
            .with_product_session_identity(product_session.identity().as_str()),
    )
}

#[derive(Debug)]
struct EditorLikeState {
    title: String,
    revision: usize,
}

impl Default for EditorLikeState {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            revision: 0,
        }
    }
}

impl EditorLikeState {
    fn basis_digest(&self) -> String {
        format!("basis:r{}", self.revision)
    }
}

fn declared(
    declaration: ForgeServerProductOperationDeclaration,
) -> ForgeServerProductOperationDeclaration {
    declaration.with_error_map(ForgeServerProductOperationErrorMaps::passthrough())
}
