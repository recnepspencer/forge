use crate::runtime::{
    RelationalRuntime, RelationalRuntimeConfigurationSnapshot, RuntimeSubsystem,
    SchemaContractRuntimeSubsystem,
};
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
    BranchTransitionRejected,
    RetentionCapacityExhausted,
    RetentionIdentityExhausted,
    RetentionOwnerUnavailable,
    RetentionRootSetTooLarge,
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
        let installed = self.runtime.configuration.snapshot();
        let merged = installed
            .config
            .schema
            .registry
            .clone()
            .extend(additions)
            .map_err(schema_denial)?;
        self.runtime
            .history
            .transition_empty_branches_to_initial_schema(&merged)
            .map_err(branch_transition_denial)?;
        let retained_entity_kind_count = merged.entity_kinds.len();
        let retained_relation_kind_count = merged.relation_kinds.len();
        // The registry and the contract runtime lowered from it are installed as
        // one change, so no concurrently bound service can observe the new
        // registry against the old contract runtime.
        let rebuilt = rebuilt_schema_contract_runtime(&installed, merged.clone());
        self.runtime.reconfigure(|configuration| {
            configuration.install_initial_schema(merged, rebuilt);
        });
        Ok(RelationalInitialSchemaInstallationReceipt {
            retained_entity_kind_count,
            retained_relation_kind_count,
        })
    }
}

/// Lower the merged registry into a fresh contract runtime, keeping the custom
/// invariant registries the installed one already carries.
fn rebuilt_schema_contract_runtime(
    installed: &RelationalRuntimeConfigurationSnapshot,
    merged: RelationalSchemaRegistry,
) -> SchemaContractRuntimeSubsystem {
    let mut config = installed.config.as_ref().clone();
    config.schema.registry = merged;
    let mut rebuilt = <SchemaContractRuntimeSubsystem as RuntimeSubsystem>::new(&config);
    rebuilt.custom_invariant_registries.clone_from(
        &installed
            .schema_contract_runtime
            .custom_invariant_registries,
    );
    rebuilt
}

fn schema_denial(error: SchemaRegistryError) -> RelationalInitialSchemaInstallationDenial {
    RelationalInitialSchemaInstallationDenial::new(
        RelationalInitialSchemaInstallationDenialKind::SchemaRejected,
        error.detail,
    )
}

fn branch_transition_denial(
    denial: crate::branch::RelationalBranchCellDenial,
) -> RelationalInitialSchemaInstallationDenial {
    let kind = match denial {
        crate::branch::RelationalBranchCellDenial::RetentionCapacityExhausted => {
            RelationalInitialSchemaInstallationDenialKind::RetentionCapacityExhausted
        }
        crate::branch::RelationalBranchCellDenial::RetentionIdentityExhausted => {
            RelationalInitialSchemaInstallationDenialKind::RetentionIdentityExhausted
        }
        crate::branch::RelationalBranchCellDenial::RetentionOwnerUnavailable => {
            RelationalInitialSchemaInstallationDenialKind::RetentionOwnerUnavailable
        }
        crate::branch::RelationalBranchCellDenial::RetentionRootSetTooLarge => {
            RelationalInitialSchemaInstallationDenialKind::RetentionRootSetTooLarge
        }
        _ => RelationalInitialSchemaInstallationDenialKind::BranchTransitionRejected,
    };
    RelationalInitialSchemaInstallationDenial::new(
        kind,
        format!("empty branch schema transition failed: {denial:?}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_schema_transition_preserves_retention_identity_exhaustion() {
        assert_eq!(
            branch_transition_denial(
                crate::branch::RelationalBranchCellDenial::RetentionIdentityExhausted,
            )
            .kind(),
            RelationalInitialSchemaInstallationDenialKind::RetentionIdentityExhausted,
        );
    }
}
