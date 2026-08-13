use crate::runtime::{RelationalRuntime, RuntimeSubsystem, SchemaContractRuntimeSubsystem};
use crate::schema::data::{RelationalSchemaRegistry, SchemaRegistryError};

/// Move-only authority to extend an uncommitted runtime's initial schema.
#[derive(Debug)]
pub struct RelationalInitialSchemaInstallation<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalInitialSchemaInstallationDenialKind {
    RuntimeAlreadyCommitted,
    SchemaRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalInitialSchemaInstallationDenial {
    kind: RelationalInitialSchemaInstallationDenialKind,
    detail: String,
}

impl RelationalInitialSchemaInstallationDenial {
    fn new(kind: RelationalInitialSchemaInstallationDenialKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> RelationalInitialSchemaInstallationDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for RelationalInitialSchemaInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "initial Relational schema installation denied: {:?} ({})",
            self.kind, self.detail
        )
    }
}

impl std::error::Error for RelationalInitialSchemaInstallationDenial {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalInitialSchemaInstallationReceipt {
    retained_entity_kind_count: usize,
    retained_relation_kind_count: usize,
}

impl RelationalInitialSchemaInstallationReceipt {
    pub const fn retained_entity_kind_count(&self) -> usize {
        self.retained_entity_kind_count
    }

    pub const fn retained_relation_kind_count(&self) -> usize {
        self.retained_relation_kind_count
    }
}

impl RelationalRuntime {
    pub fn prepare_initial_schema_installation(
        &mut self,
    ) -> Result<RelationalInitialSchemaInstallation<'_>, RelationalInitialSchemaInstallationDenial>
    {
        if self.history().latest_commit().is_some() {
            return Err(RelationalInitialSchemaInstallationDenial::new(
                RelationalInitialSchemaInstallationDenialKind::RuntimeAlreadyCommitted,
                "initial schema authority closes after the first committed mutation",
            ));
        }
        Ok(RelationalInitialSchemaInstallation { runtime: self })
    }
}

impl RelationalInitialSchemaInstallation<'_> {
    pub fn install(
        self,
        additions: RelationalSchemaRegistry,
    ) -> Result<RelationalInitialSchemaInstallationReceipt, RelationalInitialSchemaInstallationDenial>
    {
        let merged = self
            .runtime
            .config()
            .schema
            .registry
            .clone()
            .extend(additions)
            .map_err(schema_denial)?;
        self.runtime.config.schema.registry = merged;
        rebuild_schema_contract_runtime(self.runtime);
        Ok(RelationalInitialSchemaInstallationReceipt {
            retained_entity_kind_count: self.runtime.config().schema.registry.entity_kinds.len(),
            retained_relation_kind_count: self
                .runtime
                .config()
                .schema
                .registry
                .relation_kinds
                .len(),
        })
    }
}

fn rebuild_schema_contract_runtime(runtime: &mut RelationalRuntime) {
    let custom_invariant_registries =
        std::mem::take(&mut runtime.schema_contract_runtime.custom_invariant_registries);
    let mut rebuilt = <SchemaContractRuntimeSubsystem as RuntimeSubsystem>::new(runtime.config());
    rebuilt.custom_invariant_registries = custom_invariant_registries;
    runtime.schema_contract_runtime = rebuilt;
}

fn schema_denial(error: SchemaRegistryError) -> RelationalInitialSchemaInstallationDenial {
    RelationalInitialSchemaInstallationDenial::new(
        RelationalInitialSchemaInstallationDenialKind::SchemaRejected,
        error.detail,
    )
}
