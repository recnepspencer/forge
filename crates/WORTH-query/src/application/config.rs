use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryConfigSectionFamily {
    Query,
    Relational,
    Signal,
    RuntimeBridge,
    Store,
}

impl WorthQueryConfigSectionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Relational => "relational",
            Self::Signal => "signal",
            Self::RuntimeBridge => "runtime_bridge",
            Self::Store => "store",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQuerySubsystemOwner {
    Query,
    Relational,
    Signal,
    RuntimeBridge,
    Store,
}

impl WorthQuerySubsystemOwner {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Relational => "relational",
            Self::Signal => "signal",
            Self::RuntimeBridge => "runtime_bridge",
            Self::Store => "store",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationAdmissionFailureClass {
    MissingRequiredSection,
    ContradictorySectionPosture,
    DeferredStoreBackedSection,
}

impl ConfigurationAdmissionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingRequiredSection => "missing_required_section",
            Self::ContradictorySectionPosture => "contradictory_section_posture",
            Self::DeferredStoreBackedSection => "deferred_store_backed_section",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryConfigCounters {
    config_validation_count: usize,
    config_section_resolution_count: usize,
    config_validation_denial_count: usize,
}

impl WorthQueryConfigCounters {
    pub fn config_validation_count(&self) -> usize {
        self.config_validation_count
    }

    pub fn config_section_resolution_count(&self) -> usize {
        self.config_section_resolution_count
    }

    pub fn config_validation_denial_count(&self) -> usize {
        self.config_validation_denial_count
    }

    fn successful_validation() -> Self {
        Self {
            config_validation_count: 1,
            config_section_resolution_count: 5,
            config_validation_denial_count: 0,
        }
    }

    fn denied_validation() -> Self {
        Self {
            config_validation_count: 1,
            config_section_resolution_count: 5,
            config_validation_denial_count: 1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationAdmissionError {
    failure_class: ConfigurationAdmissionFailureClass,
    section: Option<WorthQueryConfigSectionFamily>,
    counters: WorthQueryConfigCounters,
    reason: &'static str,
}

impl ConfigurationAdmissionError {
    fn new(
        failure_class: ConfigurationAdmissionFailureClass,
        section: Option<WorthQueryConfigSectionFamily>,
        reason: &'static str,
    ) -> Self {
        Self {
            failure_class,
            section,
            counters: WorthQueryConfigCounters::denied_validation(),
            reason,
        }
    }

    pub fn failure_class(&self) -> ConfigurationAdmissionFailureClass {
        self.failure_class
    }

    pub fn section(&self) -> Option<WorthQueryConfigSectionFamily> {
        self.section
    }

    pub fn counters(&self) -> &WorthQueryConfigCounters {
        &self.counters
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryQueryConfig {
    runtime_backed_reads_enabled: bool,
}

impl WorthQueryQueryConfig {
    pub fn enabled() -> Self {
        Self {
            runtime_backed_reads_enabled: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            runtime_backed_reads_enabled: false,
        }
    }

    pub fn runtime_backed_reads_enabled(&self) -> bool {
        self.runtime_backed_reads_enabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRelationalConfig {
    workflow_orchestration_enabled: bool,
    historical_evaluation_enabled: bool,
}

impl WorthQueryRelationalConfig {
    pub fn enabled() -> Self {
        Self {
            workflow_orchestration_enabled: true,
            historical_evaluation_enabled: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            workflow_orchestration_enabled: false,
            historical_evaluation_enabled: false,
        }
    }

    pub fn with_workflow_orchestration(mut self, enabled: bool) -> Self {
        self.workflow_orchestration_enabled = enabled;
        self
    }

    pub fn with_historical_evaluation(mut self, enabled: bool) -> Self {
        self.historical_evaluation_enabled = enabled;
        self
    }

    pub fn workflow_orchestration_enabled(&self) -> bool {
        self.workflow_orchestration_enabled
    }

    pub fn historical_evaluation_enabled(&self) -> bool {
        self.historical_evaluation_enabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySignalConfig {
    live_promotion_enabled: bool,
}

impl WorthQuerySignalConfig {
    pub fn enabled() -> Self {
        Self {
            live_promotion_enabled: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            live_promotion_enabled: false,
        }
    }

    pub fn live_promotion_enabled(&self) -> bool {
        self.live_promotion_enabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRuntimeBridgeConfig {
    preview_session_enabled: bool,
}

impl WorthQueryRuntimeBridgeConfig {
    pub fn enabled() -> Self {
        Self {
            preview_session_enabled: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            preview_session_enabled: false,
        }
    }

    pub fn preview_session_enabled(&self) -> bool {
        self.preview_session_enabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryStoreConfig {
    durable_artifacts_enabled: bool,
}

impl WorthQueryStoreConfig {
    pub fn disabled() -> Self {
        Self {
            durable_artifacts_enabled: false,
        }
    }

    pub fn enabled() -> Self {
        Self {
            durable_artifacts_enabled: true,
        }
    }

    pub fn durable_artifacts_enabled(&self) -> bool {
        self.durable_artifacts_enabled
    }
}

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConfigSectionResolution {
    section: WorthQueryConfigSectionFamily,
    owner: WorthQuerySubsystemOwner,
    enabled: bool,
    config_digest: String,
}

impl WorthQueryConfigSectionResolution {
    fn new(
        section: WorthQueryConfigSectionFamily,
        owner: WorthQuerySubsystemOwner,
        enabled: bool,
        config_digest: String,
    ) -> Self {
        Self {
            section,
            owner,
            enabled,
            config_digest,
        }
    }

    pub fn section(&self) -> WorthQueryConfigSectionFamily {
        self.section
    }

    pub fn owner(&self) -> WorthQuerySubsystemOwner {
        self.owner
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn config_digest(&self) -> &str {
        &self.config_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedWorthQueryConfig {
    raw: WorthQueryConfig,
    query_resolution: WorthQueryConfigSectionResolution,
    relational_resolution: WorthQueryConfigSectionResolution,
    signal_resolution: WorthQueryConfigSectionResolution,
    runtime_bridge_resolution: WorthQueryConfigSectionResolution,
    store_resolution: WorthQueryConfigSectionResolution,
    counters: WorthQueryConfigCounters,
    validated_digest: String,
}

impl ValidatedWorthQueryConfig {
    fn new(raw: WorthQueryConfig) -> Result<Self, ConfigurationAdmissionError> {
        let config_digest = raw.digest();
        let query_resolution = WorthQueryConfigSectionResolution::new(
            WorthQueryConfigSectionFamily::Query,
            WorthQuerySubsystemOwner::Query,
            raw.query().runtime_backed_reads_enabled(),
            config_digest.clone(),
        );
        let relational_resolution = WorthQueryConfigSectionResolution::new(
            WorthQueryConfigSectionFamily::Relational,
            WorthQuerySubsystemOwner::Relational,
            raw.relational().workflow_orchestration_enabled()
                || raw.relational().historical_evaluation_enabled(),
            config_digest.clone(),
        );
        let signal_resolution = WorthQueryConfigSectionResolution::new(
            WorthQueryConfigSectionFamily::Signal,
            WorthQuerySubsystemOwner::Signal,
            raw.signal().live_promotion_enabled(),
            config_digest.clone(),
        );
        let runtime_bridge_resolution = WorthQueryConfigSectionResolution::new(
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQuerySubsystemOwner::RuntimeBridge,
            raw.runtime_bridge().preview_session_enabled(),
            config_digest.clone(),
        );
        let store_resolution = WorthQueryConfigSectionResolution::new(
            WorthQueryConfigSectionFamily::Store,
            WorthQuerySubsystemOwner::Store,
            raw.store().durable_artifacts_enabled(),
            config_digest.clone(),
        );

        if !raw.query().runtime_backed_reads_enabled()
            && (raw.signal().live_promotion_enabled()
                || raw.runtime_bridge().preview_session_enabled()
                || raw.relational().workflow_orchestration_enabled()
                || raw.relational().historical_evaluation_enabled())
        {
            return Err(ConfigurationAdmissionError::new(
                ConfigurationAdmissionFailureClass::MissingRequiredSection,
                Some(WorthQueryConfigSectionFamily::Query),
                "query section must remain enabled when signal, runtime bridge, or relational runtime-backed sections are enabled",
            ));
        }

        if raw.store().durable_artifacts_enabled()
            && !raw.relational().historical_evaluation_enabled()
        {
            return Err(ConfigurationAdmissionError::new(
                ConfigurationAdmissionFailureClass::ContradictorySectionPosture,
                Some(WorthQueryConfigSectionFamily::Store),
                "store-backed durability cannot be enabled before relational historical evaluation is enabled",
            ));
        }

        if raw.store().durable_artifacts_enabled() {
            return Err(ConfigurationAdmissionError::new(
                ConfigurationAdmissionFailureClass::DeferredStoreBackedSection,
                Some(WorthQueryConfigSectionFamily::Store),
                "store-backed durable artifacts remain deferred debt until later milestones close",
            ));
        }

        let validated_digest = hash_parts(&[
            format!("config:{config_digest}"),
            format!("query:{}", query_resolution.enabled()),
            format!("relational:{}", relational_resolution.enabled()),
            format!("signal:{}", signal_resolution.enabled()),
            format!("runtime_bridge:{}", runtime_bridge_resolution.enabled()),
            format!("store:{}", store_resolution.enabled()),
        ]);

        Ok(Self {
            raw,
            query_resolution,
            relational_resolution,
            signal_resolution,
            runtime_bridge_resolution,
            store_resolution,
            counters: WorthQueryConfigCounters::successful_validation(),
            validated_digest,
        })
    }

    pub fn raw(&self) -> &WorthQueryConfig {
        &self.raw
    }

    pub fn query(&self) -> &WorthQueryQueryConfig {
        self.raw.query()
    }

    pub fn relational(&self) -> &WorthQueryRelationalConfig {
        self.raw.relational()
    }

    pub fn signal(&self) -> &WorthQuerySignalConfig {
        self.raw.signal()
    }

    pub fn runtime_bridge(&self) -> &WorthQueryRuntimeBridgeConfig {
        self.raw.runtime_bridge()
    }

    pub fn store(&self) -> &WorthQueryStoreConfig {
        self.raw.store()
    }

    pub fn resolve_section(
        &self,
        section: WorthQueryConfigSectionFamily,
    ) -> WorthQueryConfigSectionResolution {
        match section {
            WorthQueryConfigSectionFamily::Query => self.query_resolution.clone(),
            WorthQueryConfigSectionFamily::Relational => self.relational_resolution.clone(),
            WorthQueryConfigSectionFamily::Signal => self.signal_resolution.clone(),
            WorthQueryConfigSectionFamily::RuntimeBridge => self.runtime_bridge_resolution.clone(),
            WorthQueryConfigSectionFamily::Store => self.store_resolution.clone(),
        }
    }

    pub fn counters(&self) -> &WorthQueryConfigCounters {
        &self.counters
    }

    pub fn validated_digest(&self) -> &str {
        &self.validated_digest
    }
}
