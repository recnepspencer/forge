use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::schema::data::AspectPlanCatalog;
#[cfg(test)]
use crate::schema::data::RelationIntegrityPlanCatalog;
use crate::schema::logic::{lower_aspect_plans, lower_relation_integrity_plans};
use crate::validation::data::InvariantRegistration;
use crate::validation::logic::FrozenCustomInvariantRegistry;

#[derive(Debug, Clone, Default)]
pub(crate) struct AspectSemanticsSubsystem {
    pub(crate) plans: AspectPlanCatalog,
    #[cfg(test)]
    pub(crate) relation_integrity_plans: RelationIntegrityPlanCatalog,
    pub(crate) relation_integrity_registrations: Vec<InvariantRegistration>,
    pub(crate) custom_invariant_registries: FrozenCustomInvariantRegistry,
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
            custom_invariant_registries: FrozenCustomInvariantRegistry::default(),
            #[cfg(test)]
            relation_integrity_plans,
        }
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
