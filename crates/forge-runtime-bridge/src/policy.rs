#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BridgeDiagnosticsTier {
    Minimal,
    #[default]
    Standard,
    Exhaustive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeDiagnosticsRetentionBudget {
    route_record_limit: usize,
    failure_record_limit: usize,
}

impl BridgeDiagnosticsRetentionBudget {
    pub const fn new(route_record_limit: usize, failure_record_limit: usize) -> Self {
        Self {
            route_record_limit,
            failure_record_limit,
        }
    }

    pub fn for_tier(tier: BridgeDiagnosticsTier) -> Self {
        match tier {
            BridgeDiagnosticsTier::Minimal => Self::new(32, 32),
            BridgeDiagnosticsTier::Standard => Self::new(128, 128),
            BridgeDiagnosticsTier::Exhaustive => Self::new(512, 512),
        }
    }

    pub fn route_record_limit(&self) -> usize {
        self.route_record_limit
    }

    pub fn failure_record_limit(&self) -> usize {
        self.failure_record_limit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeRuntimePolicy {
    diagnostics_tier: BridgeDiagnosticsTier,
    retention_budget: BridgeDiagnosticsRetentionBudget,
    record_route_artifacts: bool,
    allow_replay_artifacts: bool,
}

impl BridgeRuntimePolicy {
    pub fn operational() -> Self {
        Self {
            diagnostics_tier: BridgeDiagnosticsTier::Minimal,
            retention_budget: BridgeDiagnosticsRetentionBudget::for_tier(
                BridgeDiagnosticsTier::Minimal,
            ),
            record_route_artifacts: true,
            allow_replay_artifacts: true,
        }
    }

    pub fn development() -> Self {
        Self {
            diagnostics_tier: BridgeDiagnosticsTier::Standard,
            retention_budget: BridgeDiagnosticsRetentionBudget::for_tier(
                BridgeDiagnosticsTier::Standard,
            ),
            record_route_artifacts: true,
            allow_replay_artifacts: true,
        }
    }

    pub fn forensic() -> Self {
        Self {
            diagnostics_tier: BridgeDiagnosticsTier::Exhaustive,
            retention_budget: BridgeDiagnosticsRetentionBudget::for_tier(
                BridgeDiagnosticsTier::Exhaustive,
            ),
            record_route_artifacts: true,
            allow_replay_artifacts: true,
        }
    }

    pub fn diagnostics_tier(&self) -> BridgeDiagnosticsTier {
        self.diagnostics_tier
    }

    pub fn retention_budget(&self) -> BridgeDiagnosticsRetentionBudget {
        self.retention_budget
    }

    pub fn record_route_artifacts(&self) -> bool {
        self.record_route_artifacts
    }

    pub fn allow_replay_artifacts(&self) -> bool {
        self.allow_replay_artifacts
    }

    pub fn with_route_record_limit(mut self, route_record_limit: usize) -> Self {
        self.retention_budget = BridgeDiagnosticsRetentionBudget::new(
            route_record_limit.max(1),
            self.retention_budget.failure_record_limit(),
        );
        self
    }

    pub fn with_failure_record_limit(mut self, failure_record_limit: usize) -> Self {
        self.retention_budget = BridgeDiagnosticsRetentionBudget::new(
            self.retention_budget.route_record_limit(),
            failure_record_limit.max(1),
        );
        self
    }

    pub fn with_replay_artifacts(mut self, allow_replay_artifacts: bool) -> Self {
        self.allow_replay_artifacts = allow_replay_artifacts;
        self
    }
}

impl Default for BridgeRuntimePolicy {
    fn default() -> Self {
        Self::development()
    }
}
