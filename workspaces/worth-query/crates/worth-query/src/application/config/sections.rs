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
