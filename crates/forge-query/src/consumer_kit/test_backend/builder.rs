use crate::domain_capabilities::ForgeQueryInvariantCatalogRegistrationArtifact;
use crate::memory_workspace::ForgeQueryMemoryWorkspace;
use crate::runtime::{ForgeQueryRuntimeBuilder, ForgeQueryWorkspace};
use forge_relational::facade::runtime::{
    CustomInvariantRegistration, CustomInvariantRule, InvariantCatalog,
};

use super::backend::ForgeQueryInMemoryTestBackend;
use super::error::{ForgeQueryTestBackendError, ForgeQueryTestBackendErrorKind};
use super::schema::ForgeQueryTestBackendSchema;

#[derive(Default)]
pub struct ForgeQueryInMemoryTestRuntimeBuilder {
    schema: Option<ForgeQueryTestBackendSchema>,
    invariant_catalog: InvariantCatalog,
    custom_invariants: Vec<CustomInvariantRegistration>,
}

pub fn in_memory_test_runtime() -> ForgeQueryInMemoryTestRuntimeBuilder {
    ForgeQueryInMemoryTestRuntimeBuilder::default()
}

impl ForgeQueryInMemoryTestRuntimeBuilder {
    pub fn with_schema(mut self, schema: ForgeQueryTestBackendSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn invariant_catalog(mut self, invariant_catalog: InvariantCatalog) -> Self {
        self.merge_invariant_catalog(invariant_catalog);
        self
    }

    pub fn invariant_registration_artifact(
        mut self,
        artifact: ForgeQueryInvariantCatalogRegistrationArtifact,
    ) -> Self {
        self.merge_invariant_catalog(artifact.invariant_catalog().clone());
        self
    }

    pub fn custom_invariant(mut self, custom_invariant: CustomInvariantRegistration) -> Self {
        self.custom_invariants.push(custom_invariant);
        self
    }

    pub fn register_invariant<R>(mut self, rule: R) -> Result<Self, ForgeQueryTestBackendError>
    where
        R: CustomInvariantRule + std::panic::UnwindSafe + 'static,
    {
        let registration = CustomInvariantRegistration::new(rule).map_err(|error| {
            ForgeQueryTestBackendError::new(
                ForgeQueryTestBackendErrorKind::InvariantRegistrationFailed,
                format!("failed to register in-memory test backend invariant: {error:?}"),
            )
        })?;
        self.custom_invariants.push(registration);
        Ok(self)
    }

    pub fn workspace(
        self,
        name: impl Into<String>,
    ) -> Result<ForgeQueryWorkspace, ForgeQueryTestBackendError> {
        let schema = self.schema.ok_or_else(|| {
            ForgeQueryTestBackendError::new(
                ForgeQueryTestBackendErrorKind::MissingSchema,
                "in-memory test runtime requires a schema before workspace creation",
            )
        })?;
        let memory_workspace = ForgeQueryMemoryWorkspace::collection_with_invariants(
            schema.collection(),
            schema.memory_aspects()?,
            self.invariant_catalog,
            self.custom_invariants,
        )
        .map_err(|error| {
            ForgeQueryTestBackendError::new(
                ForgeQueryTestBackendErrorKind::WorkspaceBuildFailed,
                format!("failed to build in-memory test backend workspace: {error}"),
            )
        })?;
        let runtime = ForgeQueryRuntimeBuilder::new()
            .backend(ForgeQueryInMemoryTestBackend::new(memory_workspace))
            .build()
            .map_err(|error| {
                ForgeQueryTestBackendError::new(
                    ForgeQueryTestBackendErrorKind::WorkspaceBuildFailed,
                    format!("failed to build in-memory test runtime: {error}"),
                )
            })?;
        ForgeQueryWorkspace::new(name, runtime).map_err(|error| {
            ForgeQueryTestBackendError::new(
                ForgeQueryTestBackendErrorKind::WorkspaceBuildFailed,
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
