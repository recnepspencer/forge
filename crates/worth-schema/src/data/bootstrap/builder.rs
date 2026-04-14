use forge_relational::facade::schema::{RelationalSchemaRegistry, SchemaRegistryError};

use crate::data::bootstrap::worth_bootstrap_schema_registry;

#[derive(Debug)]
pub enum WorthSchemaBuildError {
    MissingTopologyKinds,
    MissingNamingKinds,
    Registry(SchemaRegistryError),
}

impl std::fmt::Display for WorthSchemaBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTopologyKinds => write!(f, "worth schema builder requires topology kinds"),
            Self::MissingNamingKinds => write!(f, "worth schema builder requires naming kinds"),
            Self::Registry(error) => write!(f, "worth schema registry build failed: {error:?}"),
        }
    }
}

impl std::error::Error for WorthSchemaBuildError {}

impl From<SchemaRegistryError> for WorthSchemaBuildError {
    fn from(value: SchemaRegistryError) -> Self {
        Self::Registry(value)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthSchemaBuilder {
    topology_kinds: bool,
    naming_kinds: bool,
}

impl WorthSchemaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_topology_kinds(mut self) -> Self {
        self.topology_kinds = true;
        self
    }

    pub fn with_naming_kinds(mut self) -> Self {
        self.naming_kinds = true;
        self
    }

    pub fn build(self) -> Result<RelationalSchemaRegistry, WorthSchemaBuildError> {
        if !self.topology_kinds {
            return Err(WorthSchemaBuildError::MissingTopologyKinds);
        }
        if !self.naming_kinds {
            return Err(WorthSchemaBuildError::MissingNamingKinds);
        }

        Ok(worth_bootstrap_schema_registry()?)
    }
}

#[cfg(test)]
mod tests {
    use super::{WorthSchemaBuildError, WorthSchemaBuilder};
    use crate::facade::{WorthEntityKind, WorthNamingEntityKind, WorthTopologyEntityKind};

    #[test]
    fn builder_requires_topology_and_naming_surfaces() {
        let missing_topology = WorthSchemaBuilder::new()
            .with_naming_kinds()
            .build()
            .expect_err("missing topology kinds should fail");
        assert!(matches!(
            missing_topology,
            WorthSchemaBuildError::MissingTopologyKinds
        ));

        let missing_naming = WorthSchemaBuilder::new()
            .with_topology_kinds()
            .build()
            .expect_err("missing naming kinds should fail");
        assert!(matches!(
            missing_naming,
            WorthSchemaBuildError::MissingNamingKinds
        ));
    }

    #[test]
    fn builder_emits_bootstrap_registry_when_required_surfaces_are_enabled() {
        let registry = WorthSchemaBuilder::new()
            .with_topology_kinds()
            .with_naming_kinds()
            .build()
            .expect("builder should emit bootstrap registry");

        assert!(registry
            .entity_kinds
            .contains_key(&WorthEntityKind::Topology(WorthTopologyEntityKind::Shell).kind_id()));
        assert!(registry
            .entity_kinds
            .contains_key(&WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName).kind_id()));
    }
}
