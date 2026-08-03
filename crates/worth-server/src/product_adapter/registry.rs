use std::{collections::BTreeMap, sync::Arc};

use super::{
    WorthServerProductAdapterCertificationCode, WorthServerProductAdapterCertificationError,
    WorthServerProductAdapterRegistrationReceipt, WorthServerProductApplicationAdapter,
    WorthServerProductApplicationAdapterRegistration, WorthServerProductOperationDeclaration,
};

#[derive(Clone)]
struct RegisteredProductOperation {
    adapter_label: String,
    adapter: Arc<dyn WorthServerProductApplicationAdapter>,
    durable_mutation_executor: Option<Arc<dyn crate::WorthServerDurableProductMutationExecutor>>,
    declaration: WorthServerProductOperationDeclaration,
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
pub struct WorthServerProductAdapterRegistry {
    receipts: Vec<WorthServerProductAdapterRegistrationReceipt>,
    operations_by_name: BTreeMap<String, RegisteredProductOperation>,
    mutation_lane_coordinator: super::lane_coordination::WorthServerProductMutationLaneCoordinator,
    operation_authorizer: Option<Arc<dyn super::WorthServerProductOperationAuthorizer>>,
}

impl WorthServerProductAdapterRegistry {
    pub fn build(
        registrations: Vec<WorthServerProductApplicationAdapterRegistration>,
    ) -> Result<Self, WorthServerProductAdapterRegistryError> {
        let mut receipts = Vec::new();
        let mut operations_by_name = BTreeMap::new();
        for registration in registrations {
            if registration.adapter_label().trim().is_empty() {
                return Err(
                    WorthServerProductAdapterRegistryError::InvalidRegistration {
                        adapter_label: registration.adapter_label().to_string(),
                        certification_error: WorthServerProductAdapterCertificationError::new(
                            WorthServerProductAdapterCertificationCode::BlankAdapterLabel,
                            "product adapter registrations require a non-blank label",
                        ),
                    },
                );
            }
            if registration.declarations().is_empty() {
                return Err(
                    WorthServerProductAdapterRegistryError::InvalidRegistration {
                        adapter_label: registration.adapter_label().to_string(),
                        certification_error: WorthServerProductAdapterCertificationError::new(
                            WorthServerProductAdapterCertificationCode::MissingDeclarations,
                            "product adapter registrations require at least one declaration",
                        ),
                    },
                );
            }
            for declaration in registration.declarations() {
                declaration.validate().map_err(|certification_error| {
                    WorthServerProductAdapterRegistryError::InvalidRegistration {
                        adapter_label: registration.adapter_label().to_string(),
                        certification_error,
                    }
                })?;
                validate_durable_executor(&registration, declaration).map_err(
                    |certification_error| {
                        WorthServerProductAdapterRegistryError::InvalidRegistration {
                            adapter_label: registration.adapter_label().to_string(),
                            certification_error,
                        }
                    },
                )?;
                let operation_name = declaration.operation_name().trim().to_ascii_lowercase();
                if operations_by_name.contains_key(&operation_name) {
                    return Err(
                        WorthServerProductAdapterRegistryError::DuplicateOperationName {
                            operation_name,
                        },
                    );
                }
                operations_by_name.insert(
                    operation_name,
                    RegisteredProductOperation {
                        adapter_label: registration.adapter_label().to_string(),
                        adapter: registration.adapter().clone(),
                        durable_mutation_executor: registration
                            .durable_mutation_executor()
                            .cloned(),
                        declaration: declaration.clone(),
                    },
                );
            }
            receipts.push(
                WorthServerProductAdapterRegistrationReceipt::project_expected(
                    registration.adapter_label(),
                    registration.declarations(),
                ),
            );
        }
        Ok(Self {
            receipts,
            operations_by_name,
            mutation_lane_coordinator: Default::default(),
            operation_authorizer: None,
        })
    }

    pub(crate) fn with_operation_authorizer(
        mut self,
        authorizer: Option<Arc<dyn super::WorthServerProductOperationAuthorizer>>,
    ) -> Self {
        self.operation_authorizer = authorizer;
        self
    }

    pub(crate) fn operation_authorizer(
        &self,
    ) -> Option<&Arc<dyn super::WorthServerProductOperationAuthorizer>> {
        self.operation_authorizer.as_ref()
    }

    pub fn receipts(&self) -> &[WorthServerProductAdapterRegistrationReceipt] {
        &self.receipts
    }

    pub(crate) fn declarations(&self) -> Vec<&WorthServerProductOperationDeclaration> {
        self.operations_by_name
            .values()
            .map(|registered| &registered.declaration)
            .collect()
    }

    pub(crate) fn resolve(
        &self,
        operation_name: &str,
    ) -> Option<(
        &Arc<dyn WorthServerProductApplicationAdapter>,
        &WorthServerProductOperationDeclaration,
    )> {
        self.operations_by_name
            .get(&operation_name.trim().to_ascii_lowercase())
            .map(|registered| (&registered.adapter, &registered.declaration))
    }

    pub(crate) fn resolve_durable_executor(
        &self,
        operation_name: &str,
    ) -> Option<&Arc<dyn crate::WorthServerDurableProductMutationExecutor>> {
        self.operations_by_name
            .get(&operation_name.trim().to_ascii_lowercase())
            .and_then(|registered| registered.durable_mutation_executor.as_ref())
    }

    pub(crate) fn coordinate_mutation_lane<T>(
        &self,
        lane_identity: &str,
        operation: impl FnOnce() -> T,
    ) -> T {
        self.mutation_lane_coordinator
            .coordinate(lane_identity, operation)
    }
}

fn validate_durable_executor(
    registration: &WorthServerProductApplicationAdapterRegistration,
    declaration: &WorthServerProductOperationDeclaration,
) -> Result<(), WorthServerProductAdapterCertificationError> {
    let Some(contract) = declaration.durable_mutation_contract() else {
        return Ok(());
    };
    let executor = registration.durable_mutation_executor().ok_or_else(|| {
        WorthServerProductAdapterCertificationError::new(
            WorthServerProductAdapterCertificationCode::MissingDurableMutationExecutor,
            format!(
                "durable product operation `{}` requires an atomic mutation executor",
                declaration.operation_name()
            ),
        )
    })?;
    if !executor
        .capability()
        .satisfies(contract.required_capability())
    {
        return Err(WorthServerProductAdapterCertificationError::new(
            WorthServerProductAdapterCertificationCode::IncompatibleDurableMutationCapability,
            format!(
                "durable product operation `{}` requires capability `{}`",
                declaration.operation_name(),
                contract.required_capability().as_str(),
            ),
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductAdapterRegistryError {
    DuplicateOperationName {
        operation_name: String,
    },
    InvalidRegistration {
        adapter_label: String,
        certification_error: WorthServerProductAdapterCertificationError,
    },
}

impl WorthServerProductAdapterRegistryError {
    pub fn certification_code(&self) -> Option<WorthServerProductAdapterCertificationCode> {
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
