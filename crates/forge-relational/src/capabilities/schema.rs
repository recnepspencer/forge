use crate::config::data::RelationalRuntimeConfig;
use crate::logic::runtime::RelationalRuntime;
#[cfg(test)]
use crate::schema::data::{AspectLoweringTrace, LoweredRelationIntegrityPlan};
use crate::schema::data::{AspectPlanCatalog, LoweredAspectPlan, RelationalSchemaRegistry};

pub(crate) trait SchemaSource {
    fn schema_registry(&self) -> &RelationalSchemaRegistry;
}

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

impl RelationalRuntime {
    #[cfg(test)]
    pub(crate) fn entity_aspect_plan_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<AspectLoweringTrace> {
        self.aspect_semantics
            .plans
            .entity_plans
            .get(&kind_id)
            .map(LoweredAspectPlan::lowering_trace)
    }

    #[cfg(test)]
    pub(crate) fn relation_aspect_plan_trace(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<AspectLoweringTrace> {
        self.aspect_semantics
            .plans
            .relation_plans
            .get(&kind_id)
            .map(LoweredAspectPlan::lowering_trace)
    }

    #[cfg(test)]
    pub(crate) fn relation_integrity_plan(
        &self,
        kind_id: crate::identity::data::KindId,
    ) -> Option<&LoweredRelationIntegrityPlan> {
        self.aspect_semantics
            .relation_integrity_plans
            .relation_plans
            .get(&kind_id)
    }
}
