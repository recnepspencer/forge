use crate::config::data::RelationalRuntimeConfig;
use crate::logic::runtime::RelationalRuntime;
use crate::schema::data::RelationalSchemaRegistry;

pub(crate) trait SchemaSource {
    fn schema_registry(&self) -> &RelationalSchemaRegistry;
}

impl SchemaSource for RelationalRuntime {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        &self.config.schema.registry
    }
}

impl SchemaSource for RelationalSchemaRegistry {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        self
    }
}

impl SchemaSource for RelationalRuntimeConfig {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        &self.schema.registry
    }
}
