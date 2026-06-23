use std::{collections::BTreeMap, sync::Arc};

use super::{
    ForgeServerProductAdapterCertificationCode, ForgeServerProductAdapterCertificationError,
    ForgeServerProductAdapterRegistrationReceipt, ForgeServerProductApplicationAdapter,
    ForgeServerProductApplicationAdapterRegistration, ForgeServerProductOperationDeclaration,
};

#[derive(Clone)]
struct RegisteredProductOperation {
    adapter_label: String,
    adapter: Arc<dyn ForgeServerProductApplicationAdapter>,
    declaration: ForgeServerProductOperationDeclaration,
}

impl std::fmt::Debug for RegisteredProductOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredProductOperation")
            .field("adapter_label", &self.adapter_label)
            .field("declaration", &self.declaration)
            .finish()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ForgeServerProductAdapterRegistry {
    receipts: Vec<ForgeServerProductAdapterRegistrationReceipt>,
    operations_by_name: BTreeMap<String, RegisteredProductOperation>,
}

impl ForgeServerProductAdapterRegistry {
    pub fn build(
        registrations: Vec<ForgeServerProductApplicationAdapterRegistration>,
    ) -> Result<Self, ForgeServerProductAdapterRegistryError> {
        let mut receipts = Vec::new();
        let mut operations_by_name = BTreeMap::new();
        for registration in registrations {
            if registration.adapter_label().trim().is_empty() {
                return Err(
                    ForgeServerProductAdapterRegistryError::InvalidRegistration {
                        adapter_label: registration.adapter_label().to_string(),
                        certification_error: ForgeServerProductAdapterCertificationError::new(
                            ForgeServerProductAdapterCertificationCode::BlankAdapterLabel,
                            "product adapter registrations require a non-blank label",
                        ),
                    },
                );
            }
            if registration.declarations().is_empty() {
                return Err(
                    ForgeServerProductAdapterRegistryError::InvalidRegistration {
                        adapter_label: registration.adapter_label().to_string(),
                        certification_error: ForgeServerProductAdapterCertificationError::new(
                            ForgeServerProductAdapterCertificationCode::MissingDeclarations,
                            "product adapter registrations require at least one declaration",
                        ),
                    },
                );
            }
            let mut operation_names = Vec::new();
            for declaration in registration.declarations() {
                declaration.validate().map_err(|certification_error| {
                    ForgeServerProductAdapterRegistryError::InvalidRegistration {
                        adapter_label: registration.adapter_label().to_string(),
                        certification_error,
                    }
                })?;
                let operation_name = declaration.operation_name().trim().to_ascii_lowercase();
                if operations_by_name.contains_key(&operation_name) {
                    return Err(
                        ForgeServerProductAdapterRegistryError::DuplicateOperationName {
                            operation_name,
                        },
                    );
                }
                operation_names.push(operation_name.clone());
                operations_by_name.insert(
                    operation_name,
                    RegisteredProductOperation {
                        adapter_label: registration.adapter_label().to_string(),
                        adapter: registration.adapter().clone(),
                        declaration: declaration.clone(),
                    },
                );
            }
            receipts.push(ForgeServerProductAdapterRegistrationReceipt::new(
                registration.adapter_label(),
                operation_names,
            ));
        }
        Ok(Self {
            receipts,
            operations_by_name,
        })
    }

    pub fn receipts(&self) -> &[ForgeServerProductAdapterRegistrationReceipt] {
        &self.receipts
    }

    pub(crate) fn declarations(&self) -> Vec<&ForgeServerProductOperationDeclaration> {
        self.operations_by_name
            .values()
            .map(|registered| &registered.declaration)
            .collect()
    }

    pub(crate) fn resolve(
        &self,
        operation_name: &str,
    ) -> Option<(
        &Arc<dyn ForgeServerProductApplicationAdapter>,
        &ForgeServerProductOperationDeclaration,
    )> {
        self.operations_by_name
            .get(&operation_name.trim().to_ascii_lowercase())
            .map(|registered| (&registered.adapter, &registered.declaration))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerProductAdapterRegistryError {
    DuplicateOperationName {
        operation_name: String,
    },
    InvalidRegistration {
        adapter_label: String,
        certification_error: ForgeServerProductAdapterCertificationError,
    },
}

impl ForgeServerProductAdapterRegistryError {
    pub fn certification_code(&self) -> Option<ForgeServerProductAdapterCertificationCode> {
        match self {
            Self::InvalidRegistration {
                certification_error,
                ..
            } => Some(certification_error.code()),
            Self::DuplicateOperationName { .. } => None,
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::DuplicateOperationName { operation_name } => operation_name,
            Self::InvalidRegistration {
                certification_error,
                ..
            } => certification_error.detail(),
        }
    }
}
