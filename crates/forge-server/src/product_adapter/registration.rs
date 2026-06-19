use std::sync::Arc;

use super::{ForgeServerProductApplicationAdapter, ForgeServerProductOperationDeclaration};

#[derive(Clone)]
pub struct ForgeServerProductApplicationAdapterRegistration {
    adapter_label: String,
    adapter: Arc<dyn ForgeServerProductApplicationAdapter>,
    declarations: Vec<ForgeServerProductOperationDeclaration>,
}

impl std::fmt::Debug for ForgeServerProductApplicationAdapterRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForgeServerProductApplicationAdapterRegistration")
            .field("adapter_label", &self.adapter_label)
            .field("declarations", &self.declarations)
            .finish()
    }
}

impl ForgeServerProductApplicationAdapterRegistration {
    pub fn new(
        adapter_label: impl Into<String>,
        adapter: Arc<dyn ForgeServerProductApplicationAdapter>,
    ) -> Self {
        Self {
            adapter_label: adapter_label.into(),
            adapter,
            declarations: Vec::new(),
        }
    }

    pub fn with_operation(mut self, declaration: ForgeServerProductOperationDeclaration) -> Self {
        self.declarations.push(declaration);
        self
    }

    pub fn with_operations(
        mut self,
        declarations: impl IntoIterator<Item = ForgeServerProductOperationDeclaration>,
    ) -> Self {
        self.declarations.extend(declarations);
        self
    }

    pub(crate) fn adapter_label(&self) -> &str {
        &self.adapter_label
    }

    pub(crate) fn adapter(&self) -> &Arc<dyn ForgeServerProductApplicationAdapter> {
        &self.adapter
    }

    pub(crate) fn declarations(&self) -> &[ForgeServerProductOperationDeclaration] {
        &self.declarations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductAdapterRegistrationReceipt {
    adapter_label: String,
    operation_names: Vec<String>,
    canonical_digest: String,
}

impl ForgeServerProductAdapterRegistrationReceipt {
    pub(crate) fn new(adapter_label: impl Into<String>, operation_names: Vec<String>) -> Self {
        let adapter_label = adapter_label.into();
        let canonical_digest = format!(
            "forge-server-product-adapter-registration-v1|adapter={adapter_label}|operations={}",
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
