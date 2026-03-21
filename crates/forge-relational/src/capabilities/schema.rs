use crate::config::data::RelationalRuntimeConfig;
use crate::logic::runtime::RelationalRuntime;
use crate::schema::data::{
    AspectDeclarationTrace, AspectLoweringTrace, AspectPlanCatalog, LoweredAspectPlan,
    LoweredRelationIntegrityPlan, RelationIntegrityPlanCatalog, RelationalSchemaRegistry,
};

pub(crate) trait SchemaSource {
    fn schema_registry(&self) -> &RelationalSchemaRegistry;
}

#[allow(dead_code)]
pub(crate) trait AspectPlanSource {
    fn aspect_plan_catalog(&self) -> &AspectPlanCatalog;
    fn entity_aspect_plan(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<&LoweredAspectPlan>;
    fn relation_aspect_plan(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<&LoweredAspectPlan>;
    fn entity_aspect_plan_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<AspectLoweringTrace>;
    fn relation_aspect_plan_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<AspectLoweringTrace>;
    fn relation_integrity_plan_catalog(&self) -> &RelationIntegrityPlanCatalog;
    fn relation_integrity_plan(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<&LoweredRelationIntegrityPlan>;
}

impl SchemaSource for RelationalRuntime {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        &self.config.schema.registry
    }
}

impl AspectPlanSource for RelationalRuntime {
    fn aspect_plan_catalog(&self) -> &AspectPlanCatalog {
        &self.aspect_semantics.plans
    }

    fn entity_aspect_plan(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<&LoweredAspectPlan> {
        self.aspect_semantics.plans.entity_plans.get(&kind_id)
    }

    fn relation_aspect_plan(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<&LoweredAspectPlan> {
        self.aspect_semantics.plans.relation_plans.get(&kind_id)
    }

    fn entity_aspect_plan_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<AspectLoweringTrace> {
        self.entity_aspect_plan(kind_id)
            .map(LoweredAspectPlan::lowering_trace)
    }

    fn relation_aspect_plan_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<AspectLoweringTrace> {
        self.relation_aspect_plan(kind_id)
            .map(LoweredAspectPlan::lowering_trace)
    }

    fn relation_integrity_plan_catalog(&self) -> &RelationIntegrityPlanCatalog {
        &self.aspect_semantics.relation_integrity_plans
    }

    fn relation_integrity_plan(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<&LoweredRelationIntegrityPlan> {
        self.aspect_semantics.relation_integrity_plans.relation_plans.get(&kind_id)
    }
}

#[allow(dead_code)]
pub(crate) trait AspectDeclarationSource {
    fn entity_aspect_declaration_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Result<AspectDeclarationTrace, crate::schema::data::SchemaRegistryError>;
    fn relation_aspect_declaration_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Result<AspectDeclarationTrace, crate::schema::data::SchemaRegistryError>;
}

impl SchemaSource for RelationalSchemaRegistry {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        self
    }
}

impl AspectDeclarationSource for RelationalSchemaRegistry {
    fn entity_aspect_declaration_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Result<AspectDeclarationTrace, crate::schema::data::SchemaRegistryError> {
        RelationalSchemaRegistry::entity_aspect_declaration_trace(self, kind_id)
    }

    fn relation_aspect_declaration_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Result<AspectDeclarationTrace, crate::schema::data::SchemaRegistryError> {
        RelationalSchemaRegistry::relation_aspect_declaration_trace(self, kind_id)
    }
}

impl SchemaSource for RelationalRuntimeConfig {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        &self.schema.registry
    }
}

impl RelationalRuntime {
    pub fn entity_aspect_declaration_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Result<AspectDeclarationTrace, crate::schema::data::SchemaRegistryError> {
        self.config
            .schema
            .registry
            .entity_aspect_declaration_trace(kind_id)
    }

    pub fn relation_aspect_declaration_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Result<AspectDeclarationTrace, crate::schema::data::SchemaRegistryError> {
        self.config
            .schema
            .registry
            .relation_aspect_declaration_trace(kind_id)
    }

    pub fn entity_aspect_plan_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<AspectLoweringTrace> {
        AspectPlanSource::entity_aspect_plan_trace(self, kind_id)
    }

    pub fn relation_aspect_plan_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<AspectLoweringTrace> {
        AspectPlanSource::relation_aspect_plan_trace(self, kind_id)
    }

    pub fn relation_integrity_plan(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<&LoweredRelationIntegrityPlan> {
        AspectPlanSource::relation_integrity_plan(self, kind_id)
    }
}
