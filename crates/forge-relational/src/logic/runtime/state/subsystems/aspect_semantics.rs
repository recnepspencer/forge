use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::schema::data::AspectPlanCatalog;
use crate::schema::logic::lower_aspect_plans;

#[derive(Debug, Clone, Default)]
pub(crate) struct AspectSemanticsSubsystem {
    pub(crate) plans: AspectPlanCatalog,
}

impl RuntimeSubsystem for AspectSemanticsSubsystem {
    type Config = crate::config::data::RelationalRuntimeConfig;

    fn new(config: &Self::Config) -> Self {
        Self {
            plans: lower_aspect_plans(&config.schema.registry),
        }
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
