use std::sync::Arc;

use super::{WorthServerProductApplicationAdapter, WorthServerProductOperationDeclaration};

#[derive(Clone)]
pub struct WorthServerProductApplicationAdapterRegistration {
    adapter_label: String,
    adapter: Arc<dyn WorthServerProductApplicationAdapter>,
    declarations: Vec<WorthServerProductOperationDeclaration>,
}

impl std::fmt::Debug for WorthServerProductApplicationAdapterRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorthServerProductApplicationAdapterRegistration")
            .field("adapter_label", &self.adapter_label)
            .field("declarations", &self.declarations)
            .finish()
    }
}

impl WorthServerProductApplicationAdapterRegistration {
    pub fn new(
        adapter_label: impl Into<String>,
        adapter: Arc<dyn WorthServerProductApplicationAdapter>,
    ) -> Self {
        Self {
            adapter_label: adapter_label.into(),
            adapter,
            declarations: Vec::new(),
        }
    }

    pub fn with_operation(mut self, declaration: WorthServerProductOperationDeclaration) -> Self {
        self.declarations.push(declaration);
        self
    }

    pub fn with_operations(
        mut self,
        declarations: impl IntoIterator<Item = WorthServerProductOperationDeclaration>,
    ) -> Self {
        self.declarations.extend(declarations);
        self
    }

    pub(crate) fn adapter_label(&self) -> &str {
        &self.adapter_label
    }

    pub(crate) fn adapter(&self) -> &Arc<dyn WorthServerProductApplicationAdapter> {
        &self.adapter
    }

    pub(crate) fn declarations(&self) -> &[WorthServerProductOperationDeclaration] {
        &self.declarations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductAdapterRegistrationReceipt {
    adapter_label: String,
    operation_names: Vec<String>,
    canonical_digest: String,
}

impl WorthServerProductAdapterRegistrationReceipt {
    pub(crate) fn new(adapter_label: impl Into<String>, operation_names: Vec<String>) -> Self {
        let adapter_label = adapter_label.into();
        let canonical_digest = format!(
            "worth-server-product-adapter-registration-v1|adapter={adapter_label}|operations={}",
            operation_names.join(",")
        );
        Self {
            adapter_label,
            operation_names,
            canonical_digest,
        }
    }

    pub fn adapter_label(&self) -> &str {
        &self.adapter_label
    }

    pub fn operation_names(&self) -> &[String] {
        &self.operation_names
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
