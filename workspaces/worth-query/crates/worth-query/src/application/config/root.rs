use crate::identity::hash_parts;

use super::sections::{
    WorthQueryQueryConfig, WorthQueryRelationalConfig, WorthQueryRuntimeBridgeConfig,
    WorthQuerySignalConfig, WorthQueryStoreConfig,
};
use super::validation::{ConfigurationAdmissionError, ValidatedWorthQueryConfig};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConfig {
    query: WorthQueryQueryConfig,
    relational: WorthQueryRelationalConfig,
    signal: WorthQuerySignalConfig,
    runtime_bridge: WorthQueryRuntimeBridgeConfig,
    store: WorthQueryStoreConfig,
}

impl WorthQueryConfig {
    pub fn runtime_backed_default() -> Self {
        Self {
            query: WorthQueryQueryConfig::enabled(),
            relational: WorthQueryRelationalConfig::enabled(),
            signal: WorthQuerySignalConfig::enabled(),
            runtime_bridge: WorthQueryRuntimeBridgeConfig::enabled(),
            store: WorthQueryStoreConfig::disabled(),
        }
    }

    pub fn with_query(mut self, query: WorthQueryQueryConfig) -> Self {
        self.query = query;
        self
    }

    pub fn with_relational(mut self, relational: WorthQueryRelationalConfig) -> Self {
        self.relational = relational;
        self
    }

    pub fn with_signal(mut self, signal: WorthQuerySignalConfig) -> Self {
        self.signal = signal;
        self
    }

    pub fn with_runtime_bridge(mut self, runtime_bridge: WorthQueryRuntimeBridgeConfig) -> Self {
        self.runtime_bridge = runtime_bridge;
        self
    }

    pub fn with_store(mut self, store: WorthQueryStoreConfig) -> Self {
        self.store = store;
        self
    }

    pub fn query(&self) -> &WorthQueryQueryConfig {
        &self.query
    }

    pub fn relational(&self) -> &WorthQueryRelationalConfig {
        &self.relational
    }

    pub fn signal(&self) -> &WorthQuerySignalConfig {
        &self.signal
    }

    pub fn runtime_bridge(&self) -> &WorthQueryRuntimeBridgeConfig {
        &self.runtime_bridge
    }

    pub fn store(&self) -> &WorthQueryStoreConfig {
        &self.store
    }

    pub(crate) fn digest(&self) -> String {
        hash_parts(&[
            format!(
                "query:runtime_backed_reads:{}",
                self.query.runtime_backed_reads_enabled()
            ),
            format!(
                "relational:workflow_orchestration:{}",
                self.relational.workflow_orchestration_enabled()
            ),
            format!(
                "relational:historical_evaluation:{}",
                self.relational.historical_evaluation_enabled()
            ),
            format!(
                "signal:live_promotion:{}",
                self.signal.live_promotion_enabled()
            ),
            format!(
                "runtime_bridge:preview_session:{}",
                self.runtime_bridge.preview_session_enabled()
            ),
            format!(
                "store:durable_artifacts:{}",
                self.store.durable_artifacts_enabled()
            ),
        ])
    }

    pub fn validate(self) -> Result<ValidatedWorthQueryConfig, ConfigurationAdmissionError> {
        ValidatedWorthQueryConfig::new(self)
    }
}
