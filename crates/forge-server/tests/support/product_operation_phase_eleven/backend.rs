use std::sync::{Arc, Mutex};

use forge_server::{
    ForgeServerProductAdapterExecutionError, ForgeServerProductApplicationAdapter,
    ForgeServerProductApplicationAdapterRegistration, ForgeServerProductOperationBasisKind,
    ForgeServerProductOperationDeclaration, ForgeServerProductOperationErrorMaps,
    ForgeServerProductOperationPayload, ForgeServerProductOperationSuccess,
    ForgeServerProductOperationSupportSnapshot,
};
use serde_json::json;

#[derive(Clone, Debug)]
pub struct StatefulProductEditorBackend {
    state: Arc<Mutex<ProductEditorState>>,
}

impl StatefulProductEditorBackend {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ProductEditorState::default())),
        }
    }

    pub fn basis_digest(&self) -> String {
        self.state
            .lock()
            .expect("stateful editor backend should not poison")
            .basis_digest()
    }

    pub fn title(&self) -> String {
        self.state
            .lock()
            .expect("stateful editor backend should not poison")
            .title
            .clone()
    }

    pub fn revision(&self) -> usize {
        self.state
            .lock()
            .expect("stateful editor backend should not poison")
            .revision
    }
}

pub fn stateful_editor_registration(
    backend: StatefulProductEditorBackend,
) -> ForgeServerProductApplicationAdapterRegistration {
    ForgeServerProductApplicationAdapterRegistration::new("stateful-editor", Arc::new(backend))
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_read(
                "product_editor.render_preview",
                "product-editor.render-preview.v1",
                ForgeServerProductOperationBasisKind::ProductSessionDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted("preview-session"),
            ),
        ))
        .with_operation(declared(
            ForgeServerProductOperationDeclaration::product_mutation(
                "product_editor.apply",
                "product-editor.apply.v1",
                ForgeServerProductOperationBasisKind::ProductSessionDerived,
                ForgeServerProductOperationSupportSnapshot::production_admitted("apply-session"),
                "draft",
            ),
        ))
}

pub fn controlled_apply_payload(title: &str, fail: bool) -> ForgeServerProductOperationPayload {
    ForgeServerProductOperationPayload::json(
        "product-editor.apply.v1",
        json!({ "title": title, "fail": fail }),
    )
}

impl ForgeServerProductApplicationAdapter for StatefulProductEditorBackend {
    fn execute(
        &self,
        operation: &forge_server::ForgeServerScheduledProductOperation,
    ) -> Result<ForgeServerProductOperationSuccess, ForgeServerProductAdapterExecutionError> {
        let mut state = self
            .state
            .lock()
            .expect("stateful editor backend should not poison");
        match operation.plan().declaration().operation_name() {
            "product_editor.render_preview" => Ok(ForgeServerProductOperationSuccess::new(
                "product_editor.render_preview",
                format!("preview:{}:{}", state.basis_digest(), state.title),
            )),
            "product_editor.apply" => {
                let body = operation.plan().payload().body();
                if body.get("fail").and_then(|value| value.as_bool()) == Some(true) {
                    return Err(ForgeServerProductAdapterExecutionError::failed(
                        "adapter_failed",
                        format!("controlled failure at {}", state.basis_digest()),
                    ));
                }
                let title = body
                    .get("title")
                    .and_then(|value| value.as_str())
                    .unwrap_or("Untitled");
                state.title = title.to_string();
                state.revision += 1;
                Ok(ForgeServerProductOperationSuccess::new(
                    "product_editor.apply",
                    format!("{}:{}", state.basis_digest(), state.title),
                ))
            }
            operation_name => Err(ForgeServerProductAdapterExecutionError::failed(
                "unsupported_operation",
                format!("stateful editor backend does not support `{operation_name}`"),
            )),
        }
    }
}

#[derive(Debug)]
struct ProductEditorState {
    title: String,
    revision: usize,
}

impl Default for ProductEditorState {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            revision: 0,
        }
    }
}

impl ProductEditorState {
    fn basis_digest(&self) -> String {
        format!("basis:r{}", self.revision)
    }
}

fn declared(
    declaration: ForgeServerProductOperationDeclaration,
) -> ForgeServerProductOperationDeclaration {
    declaration.with_error_map(ForgeServerProductOperationErrorMaps::passthrough())
}
