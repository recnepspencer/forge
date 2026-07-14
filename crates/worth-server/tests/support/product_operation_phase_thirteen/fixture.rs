#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use serde_json::{json, Value};
use worth_server::{
    WorthServer, WorthServerCompatibilityPreparedRequest, WorthServerProductAdapterExecutionError,
    WorthServerProductApplicationAdapter, WorthServerProductApplicationAdapterRegistration,
    WorthServerProductOperationDeclaration, WorthServerProductOperationDenial,
    WorthServerProductOperationErrorMaps, WorthServerProductOperationInput,
    WorthServerProductOperationPayload, WorthServerProductOperationSuccess,
    WorthServerProductOperationSupportSnapshot, WorthServerProductSession,
    WorthServerProductSessionCreationRequest, WorthServerWorthNativeSession,
};

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

    pub fn registration(&self) -> WorthServerProductApplicationAdapterRegistration {
        WorthServerProductApplicationAdapterRegistration::new(
            "stateful-editor-like",
            Arc::new(self.clone()),
        )
        .with_operation(declared(
            WorthServerProductOperationDeclaration::product_read(
                "product_editor.render",
                "product-editor.render.v1",
                worth_server::WorthServerProductOperationBasisKind::DurableProductDerived,
                WorthServerProductOperationSupportSnapshot::production_admitted("render-ready"),
            ),
        ))
        .with_operation(declared(
            WorthServerProductOperationDeclaration::product_read(
                "product_editor.select",
                "product-editor.select.v1",
                worth_server::WorthServerProductOperationBasisKind::DurableProductDerived,
                WorthServerProductOperationSupportSnapshot::production_admitted("select-ready"),
            ),
        ))
        .with_operation(declared(
            WorthServerProductOperationDeclaration::product_read(
                "product_editor.available_actions",
                "product-editor.actions.v1",
                worth_server::WorthServerProductOperationBasisKind::DurableProductDerived,
                WorthServerProductOperationSupportSnapshot::production_admitted("actions-ready"),
            ),
        ))
        .with_operation(declared(
            WorthServerProductOperationDeclaration::product_mutation(
                "product_editor.apply",
                "product-editor.apply.v1",
                worth_server::WorthServerProductOperationBasisKind::ProductSessionDerived,
                WorthServerProductOperationSupportSnapshot::production_admitted("apply-ready"),
                "draft",
            ),
        ))
        .with_operation(declared(
            WorthServerProductOperationDeclaration::product_mutation(
                "product_editor.finalize",
                "product-editor.finalize.v1",
                worth_server::WorthServerProductOperationBasisKind::ProductSessionDerived,
                WorthServerProductOperationSupportSnapshot::production_admitted("finalize-ready"),
                "draft",
            ),
        ))
    }
}

impl WorthServerProductApplicationAdapter for StatefulEditorLikeBackend {
    fn execute(
        &self,
        operation: &worth_server::WorthServerScheduledProductOperation,
    ) -> Result<WorthServerProductOperationSuccess, WorthServerProductAdapterExecutionError> {
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
            "product_editor.render" => Ok(WorthServerProductOperationSuccess::new(
                "product_editor.render",
                format!(
                    "render:{request_basis}:{}:{}",
                    state.basis_digest(),
                    state.title
                ),
            )),
            "product_editor.select" => {
                let node = body.get("node").and_then(Value::as_str).unwrap_or("none");
                Ok(WorthServerProductOperationSuccess::new(
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
                Ok(WorthServerProductOperationSuccess::new(
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
                Ok(WorthServerProductOperationSuccess::new(
                    "product_editor.apply",
                    state.basis_digest(),
                ))
            }
            "product_editor.finalize" => {
                if body.get("confirm").and_then(Value::as_bool) != Some(true) {
                    return Err(WorthServerProductAdapterExecutionError::denied(
                        WorthServerProductOperationDenial::new(
                            "product.finalize.confirm_required",
                            "finalize requires explicit confirmation",
                        ),
                    ));
                }
                state.revision += 1;
                Ok(WorthServerProductOperationSuccess::new(
                    "product_editor.finalize",
                    format!("finalized:{}", state.basis_digest()),
                ))
            }
            operation_name => Err(WorthServerProductAdapterExecutionError::failed(
                "unsupported_operation",
                format!("unsupported editor-like operation `{operation_name}`"),
            )),
        }
    }
}

pub fn build_server(backend: &StatefulEditorLikeBackend) -> WorthServer {
    product_adapter_phase_nine_fixture::build_server(vec![backend.registration()])
}

pub fn direct_session(server: &WorthServer) -> WorthServerWorthNativeSession {
    product_adapter_phase_nine_fixture::direct_session(server)
}

pub fn open_mutation_session(
    session: &WorthServerWorthNativeSession,
    basis_digest: &str,
    operation_name: &str,
) -> WorthServerProductSession {
    session
        .product_sessions()
        .open_mutation(
            WorthServerProductSessionCreationRequest::for_operation(operation_name)
                .with_basis_digest(basis_digest)
                .with_expiry_seconds(300),
        )
        .expect("mutation session should open")
}

pub fn prepared_read_request(
    server: &WorthServer,
    operation_name: &str,
    basis_digest: Option<&str>,
) -> WorthServerCompatibilityPreparedRequest {
    product_adapter_phase_nine_fixture::prepared_read_request(server, operation_name, basis_digest)
}

pub fn prepared_mutation_request(
    server: &WorthServer,
    operation_name: &str,
    basis_digest: Option<&str>,
) -> WorthServerCompatibilityPreparedRequest {
    product_adapter_phase_nine_fixture::prepared_mutation_request(
        server,
        operation_name,
        basis_digest,
    )
}

pub fn render_payload() -> WorthServerProductOperationPayload {
    WorthServerProductOperationPayload::json("product-editor.render.v1", json!({}))
}

pub fn select_payload(node: &str) -> WorthServerProductOperationPayload {
    WorthServerProductOperationPayload::json("product-editor.select.v1", json!({ "node": node }))
}

pub fn actions_payload() -> WorthServerProductOperationPayload {
    WorthServerProductOperationPayload::json("product-editor.actions.v1", json!({}))
}

pub fn apply_payload(title: &str) -> WorthServerProductOperationPayload {
    WorthServerProductOperationPayload::json("product-editor.apply.v1", json!({ "title": title }))
}

pub fn finalize_payload(confirm: bool) -> WorthServerProductOperationPayload {
    WorthServerProductOperationPayload::json(
        "product-editor.finalize.v1",
        json!({ "confirm": confirm }),
    )
}

pub fn direct_read(
    session: &WorthServerWorthNativeSession,
    operation_name: &str,
    payload: WorthServerProductOperationPayload,
    basis_digest: &str,
) -> worth_server::WorthServerCompletedProductOperation {
    session
        .product_operations()
        .execute(
            WorthServerProductOperationInput::new(operation_name, payload)
                .with_basis_digest(basis_digest),
        )
        .expect("direct editor-like read should complete")
}

pub fn direct_mutation(
    session: &WorthServerWorthNativeSession,
    operation_name: &str,
    payload: WorthServerProductOperationPayload,
    basis_digest: &str,
    product_session: &WorthServerProductSession,
) -> Result<
    worth_server::WorthServerCompletedProductOperation,
    worth_server::WorthServerProductOperationSurfaceDenial,
> {
    session.product_operations().execute(
        WorthServerProductOperationInput::new(operation_name, payload)
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
    declaration: WorthServerProductOperationDeclaration,
) -> WorthServerProductOperationDeclaration {
    declaration.with_error_map(WorthServerProductOperationErrorMaps::passthrough())
}
