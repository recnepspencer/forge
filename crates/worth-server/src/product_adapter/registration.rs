use std::sync::Arc;

use super::{WorthServerProductApplicationAdapter, WorthServerProductOperationDeclaration};

#[derive(Clone)]
pub struct WorthServerProductApplicationAdapterRegistration {
    adapter_label: String,
    adapter: Arc<dyn WorthServerProductApplicationAdapter>,
    durable_mutation_executor: Option<Arc<dyn crate::WorthServerDurableProductMutationExecutor>>,
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
            durable_mutation_executor: None,
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

    pub fn with_durable_mutation_executor(
        mut self,
        executor: Arc<dyn crate::WorthServerDurableProductMutationExecutor>,
    ) -> Self {
        self.durable_mutation_executor = Some(executor);
        self
    }

    pub(crate) fn adapter_label(&self) -> &str {
        &self.adapter_label
    }

    pub(crate) fn adapter(&self) -> &Arc<dyn WorthServerProductApplicationAdapter> {
        &self.adapter
    }

    pub(crate) fn durable_mutation_executor(
        &self,
    ) -> Option<&Arc<dyn crate::WorthServerDurableProductMutationExecutor>> {
        self.durable_mutation_executor.as_ref()
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
    /// Projects the receipt an adapter registration must produce from its declared contract.
    /// This is certification evidence only; it does not register or authorize an adapter.
    pub fn project_expected(
        adapter_label: impl Into<String>,
        declarations: &[WorthServerProductOperationDeclaration],
    ) -> Self {
        Self::new(
            adapter_label,
            declarations
                .iter()
                .map(|declaration| {
                    (
                        declaration.operation_name().trim().to_ascii_lowercase(),
                        declaration.canonical_digest(),
                    )
                })
                .collect(),
        )
    }

    pub(crate) fn new(
        adapter_label: impl Into<String>,
        operation_rows: Vec<(String, String)>,
    ) -> Self {
        let adapter_label = adapter_label.into();
        let operation_names = operation_rows
            .iter()
            .map(|(operation_name, _)| operation_name.clone())
            .collect::<Vec<_>>();
        let mut digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-product-adapter-registration-v3",
        )
        .field("adapter", &adapter_label);
        for (operation_name, declaration_digest) in &operation_rows {
            digest = digest
                .field("operation", operation_name)
                .field("declaration", declaration_digest);
        }
        let canonical_digest = digest.finish();
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
