use crate::durability::data::{DurableCheckpoint, DurableStore};
use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::logic::runtime::RelationalRuntimeConfig;
use crate::replay::data::CanonicalCommitEnvelope;

#[derive(Debug, Clone, Default)]
pub(crate) struct DurabilitySubsystem {
    pub(crate) log: Vec<CanonicalCommitEnvelope>,
    pub(crate) checkpoints: Vec<DurableCheckpoint>,
    pub(crate) store: Option<DurableStore>,
}

impl DurabilitySubsystem {
    fn build_from_config(config: &RelationalRuntimeConfig) -> Self {
        Self {
            log: Vec::new(),
            checkpoints: Vec::new(),
            store: config
                .durability
                .policy
                .store_layout
                .clone()
                .map(|layout| DurableStore {
                    layout,
                    segments: Vec::new(),
                    checkpoints: Vec::new(),
                }),
        }
    }
}

impl RuntimeSubsystem for DurabilitySubsystem {
    type Config = RelationalRuntimeConfig;

    fn new(config: &Self::Config) -> Self {
        Self::build_from_config(config)
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
