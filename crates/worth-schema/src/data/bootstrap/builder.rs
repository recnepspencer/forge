use forge_relational::facade::schema::{RelationalSchemaRegistry, SchemaRegistryError};

use crate::data::bootstrap::bootstrap_schema_registry;

#[derive(Debug)]
pub enum SchemaBuildError {
    MissingTopologyKinds,
    MissingNamingKinds,
    Registry(SchemaRegistryError),
}

impl std::fmt::Display for SchemaBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTopologyKinds => write!(f, " schema builder requires topology kinds"),
            Self::MissingNamingKinds => write!(f, " schema builder requires naming kinds"),
            Self::Registry(error) => write!(f, " schema registry build failed: {error:?}"),
        }
    }
}

impl std::error::Error for SchemaBuildError {}

impl From<SchemaRegistryError> for SchemaBuildError {
    fn from(value: SchemaRegistryError) -> Self {
        Self::Registry(value)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SchemaBuilder {
    topology_kinds: bool,
    naming_kinds: bool,
}

impl SchemaBuilder {
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

    pub fn build(self) -> Result<RelationalSchemaRegistry, SchemaBuildError> {
        if !self.topology_kinds {
            return Err(SchemaBuildError::MissingTopologyKinds);
        }
        if !self.naming_kinds {
            return Err(SchemaBuildError::MissingNamingKinds);
        }

        Ok(bootstrap_schema_registry()?)
    }
}

#[cfg(test)]
mod tests {
    use super::{SchemaBuildError, SchemaBuilder};
    use crate::facade::{EntityKind, NamingEntityKind, TopologyEntityKind};

    #[test]
    fn builder_requires_topology_and_naming_surfaces() {
        let missing_topology = SchemaBuilder::new()
            .with_naming_kinds()
            .build()
            .expect_err("missing topology kinds should fail");
        assert!(matches!(
            missing_topology,
            SchemaBuildError::MissingTopologyKinds
        ));

        let missing_naming = SchemaBuilder::new()
            .with_topology_kinds()
            .build()
            .expect_err("missing naming kinds should fail");
        assert!(matches!(
            missing_naming,
            SchemaBuildError::MissingNamingKinds
        ));
    }

    #[test]
    fn builder_emits_bootstrap_registry_when_required_surfaces_are_enabled() {
        let registry = SchemaBuilder::new()
            .with_topology_kinds()
            .with_naming_kinds()
            .build()
            .expect("builder should emit bootstrap registry");

        assert!(registry
            .entity_kinds
            .contains_key(&EntityKind::Topology(TopologyEntityKind::Shell).kind_id()));
        assert!(registry
            .entity_kinds
            .contains_key(&EntityKind::Naming(NamingEntityKind::PersistentName).kind_id()));
    }
}
