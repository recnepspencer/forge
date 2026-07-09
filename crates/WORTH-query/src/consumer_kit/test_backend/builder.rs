use crate::domain_capabilities::WorthQueryInvariantCatalogRegistrationArtifact;
use crate::memory_workspace::WorthQueryMemoryWorkspace;
use crate::runtime::{WorthQueryRuntimeBuilder, WorthQueryWorkspace};
use worth_relational::facade::runtime::{
    CustomInvariantRegistration, CustomInvariantRule, InvariantCatalog,
};

use super::backend::WorthQueryInMemoryTestBackend;
use super::error::{WorthQueryTestBackendError, WorthQueryTestBackendErrorKind};
use super::schema::WorthQueryTestBackendSchema;

#[derive(Default)]
pub struct WorthQueryInMemoryTestRuntimeBuilder {
    schema: Option<WorthQueryTestBackendSchema>,
    invariant_catalog: InvariantCatalog,
    custom_invariants: Vec<CustomInvariantRegistration>,
}

pub fn in_memory_test_runtime() -> WorthQueryInMemoryTestRuntimeBuilder {
    WorthQueryInMemoryTestRuntimeBuilder::default()
}

impl WorthQueryInMemoryTestRuntimeBuilder {
    pub fn with_schema(mut self, schema: WorthQueryTestBackendSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn invariant_catalog(mut self, invariant_catalog: InvariantCatalog) -> Self {
        self.merge_invariant_catalog(invariant_catalog);
        self
    }

    pub fn invariant_registration_artifact(
        mut self,
        artifact: WorthQueryInvariantCatalogRegistrationArtifact,
    ) -> Self {
        self.merge_invariant_catalog(artifact.invariant_catalog().clone());
        self
    }

    pub fn custom_invariant(mut self, custom_invariant: CustomInvariantRegistration) -> Self {
        self.custom_invariants.push(custom_invariant);
        self
    }

    pub fn register_invariant<R>(mut self, rule: R) -> Result<Self, WorthQueryTestBackendError>
    where
        R: CustomInvariantRule + std::panic::UnwindSafe + 'static,
    {
        let registration = CustomInvariantRegistration::new(rule).map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::InvariantRegistrationFailed,
                format!("failed to register in-memory test backend invariant: {error:?}"),
            )
        })?;
        self.custom_invariants.push(registration);
        Ok(self)
    }

    pub fn workspace(
        self,
        name: impl Into<String>,
    ) -> Result<WorthQueryWorkspace, WorthQueryTestBackendError> {
        let schema = self.schema.ok_or_else(|| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::MissingSchema,
                "in-memory test runtime requires a schema before workspace creation",
            )
        })?;
        let memory_workspace = WorthQueryMemoryWorkspace::collection_with_invariants(
            schema.collection(),
            schema.memory_aspects()?,
            self.invariant_catalog,
            self.custom_invariants,
        )
        .map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::WorkspaceBuildFailed,
                format!("failed to build in-memory test backend workspace: {error}"),
            )
        })?;
        let runtime = WorthQueryRuntimeBuilder::new()
            .backend(WorthQueryInMemoryTestBackend::new(memory_workspace))
            .build()
            .map_err(|error| {
                WorthQueryTestBackendError::new(
                    WorthQueryTestBackendErrorKind::WorkspaceBuildFailed,
                    format!("failed to build in-memory test runtime: {error}"),
                )
            })?;
        WorthQueryWorkspace::new(name, runtime).map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::WorkspaceBuildFailed,
                format!("failed to build in-memory test workspace facade: {error}"),
            )
        })
    }

    fn merge_invariant_catalog(&mut self, invariant_catalog: InvariantCatalog) {
        self.invariant_catalog
            .registrations
            .extend(invariant_catalog.registrations);
        self.invariant_catalog = self.invariant_catalog.clone().canonicalized();
    }
}
