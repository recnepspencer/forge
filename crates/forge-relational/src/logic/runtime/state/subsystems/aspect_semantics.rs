use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::schema::data::{AspectPlanCatalog, RelationIntegrityPlanCatalog};
use crate::schema::logic::{lower_aspect_plans, lower_relation_integrity_plans};
use crate::validation::data::InvariantRegistration;

#[derive(Debug, Clone, Default)]
pub(crate) struct AspectSemanticsSubsystem {
    pub(crate) plans: AspectPlanCatalog,
    pub(crate) relation_integrity_plans: RelationIntegrityPlanCatalog,
    pub(crate) relation_integrity_registrations: Vec<InvariantRegistration>,
}

impl RuntimeSubsystem for AspectSemanticsSubsystem {
    type Config = crate::config::data::RelationalRuntimeConfig;

    fn new(config: &Self::Config) -> Self {
        let relation_integrity_plans = lower_relation_integrity_plans(&config.schema.registry);
        Self {
            plans: lower_aspect_plans(&config.schema.registry),
            relation_integrity_registrations: relation_integrity_plans
                .relation_plans
                .values()
                .flat_map(crate::validation::data::relation_integrity_registrations_for_plan)
                .collect(),
            relation_integrity_plans,
        }
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
