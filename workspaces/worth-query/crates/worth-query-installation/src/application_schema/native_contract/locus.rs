use worth_foundational::facade::AspectKey;
use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;

/// Exact installed application locus to which one native aspect contract belongs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledApplicationAspectLocus {
    schema: ApplicationSchemaBindingIdentity,
    entity: String,
    aspect: AspectKey,
}

impl WorthQueryInstalledApplicationAspectLocus {
    pub(crate) fn new(
        schema: ApplicationSchemaBindingIdentity,
        entity: String,
        aspect: AspectKey,
    ) -> Self {
        Self {
            schema,
            entity,
            aspect,
        }
    }

    pub fn schema(&self) -> &ApplicationSchemaBindingIdentity {
        &self.schema
    }

    pub fn entity(&self) -> &str {
        &self.entity
    }

    pub fn aspect(&self) -> &AspectKey {
        &self.aspect
    }
}
