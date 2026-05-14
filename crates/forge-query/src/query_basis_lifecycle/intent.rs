use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisOperationLaneRequest {
    Observation,
    MutationPreparation,
    Replay,
    Inspection,
    Materialization,
    SubscriptionDeclaration,
    SubscriptionActivation,
    PreviewCloseout,
    Certification,
}

impl BasisOperationLaneRequest {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Observation => "observation",
            Self::MutationPreparation => "mutation_preparation",
            Self::Replay => "replay",
            Self::Inspection => "inspection",
            Self::Materialization => "materialization",
            Self::SubscriptionDeclaration => "subscription_declaration",
            Self::SubscriptionActivation => "subscription_activation",
            Self::PreviewCloseout => "preview_closeout",
            Self::Certification => "certification",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawFutureBasisNeighborFamily {
    Temporal,
    AsyncResource,
    StoreBackedParity,
    DurableReload,
    RestartStableEnvelope,
}

impl RawFutureBasisNeighborFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Temporal => "temporal",
            Self::AsyncResource => "async_resource",
            Self::StoreBackedParity => "store_backed_parity",
            Self::DurableReload => "durable_reload",
            Self::RestartStableEnvelope => "restart_stable_envelope",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawBasisSelector {
    CurrentHead,
    BranchHead {
        branch_identity: String,
    },
    BranchSnapshot {
        branch_identity: String,
        snapshot_identity: String,
    },
    RuntimeSnapshot {
        snapshot_identity: String,
    },
    HistoricalSnapshot {
        snapshot_identity: String,
    },
    HistoricalCommit {
        commit_identity: String,
    },
    Preview {
        preview_identity: String,
    },
    PreviewDerivedHistorical {
        preview_identity: String,
    },
    FutureNeighbor {
        family: RawFutureBasisNeighborFamily,
    },
}

impl RawBasisSelector {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::BranchHead { .. } => "branch_head",
            Self::BranchSnapshot { .. } => "branch_snapshot",
            Self::RuntimeSnapshot { .. } => "runtime_snapshot",
            Self::HistoricalSnapshot { .. } => "historical_snapshot",
            Self::HistoricalCommit { .. } => "historical_commit",
            Self::Preview { .. } => "preview",
            Self::PreviewDerivedHistorical { .. } => "preview_derived_historical",
            Self::FutureNeighbor { .. } => "future_neighbor",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RawBasisSourcePath {
    DirectLifecycleConstructor,
    QueryContextCompatibility,
}

impl RawBasisSourcePath {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectLifecycleConstructor => "direct_lifecycle_constructor",
            Self::QueryContextCompatibility => "query_context_compatibility",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawBasisIntent {
    selector: RawBasisSelector,
    tenant_scope: Option<String>,
    policy_scope: Option<String>,
    schema_scope: Option<String>,
    operation_lane: BasisOperationLaneRequest,
    source_path: RawBasisSourcePath,
    raw_digest: String,
}

impl RawBasisIntent {
    pub fn current_head(operation_lane: BasisOperationLaneRequest) -> Self {
        Self::new(
            RawBasisSelector::CurrentHead,
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn branch_head(
        branch_identity: impl Into<String>,
        operation_lane: BasisOperationLaneRequest,
    ) -> Self {
        Self::new(
            RawBasisSelector::BranchHead {
                branch_identity: branch_identity.into(),
            },
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn historical_snapshot(
        snapshot_identity: impl Into<String>,
        operation_lane: BasisOperationLaneRequest,
    ) -> Self {
        Self::new(
            RawBasisSelector::HistoricalSnapshot {
                snapshot_identity: snapshot_identity.into(),
            },
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn historical_commit(
        commit_identity: impl Into<String>,
        operation_lane: BasisOperationLaneRequest,
    ) -> Self {
        Self::new(
            RawBasisSelector::HistoricalCommit {
                commit_identity: commit_identity.into(),
            },
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn preview_derived_historical(
        preview_identity: impl Into<String>,
        operation_lane: BasisOperationLaneRequest,
    ) -> Self {
        Self::new(
            RawBasisSelector::PreviewDerivedHistorical {
                preview_identity: preview_identity.into(),
            },
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn preview(
        preview_identity: impl Into<String>,
        operation_lane: BasisOperationLaneRequest,
    ) -> Self {
        Self::new(
            RawBasisSelector::Preview {
                preview_identity: preview_identity.into(),
            },
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn runtime_snapshot(
        snapshot_identity: impl Into<String>,
        operation_lane: BasisOperationLaneRequest,
    ) -> Self {
        Self::new(
            RawBasisSelector::RuntimeSnapshot {
                snapshot_identity: snapshot_identity.into(),
            },
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn branch_snapshot(
        branch_identity: impl Into<String>,
        snapshot_identity: impl Into<String>,
        operation_lane: BasisOperationLaneRequest,
    ) -> Self {
        Self::new(
            RawBasisSelector::BranchSnapshot {
                branch_identity: branch_identity.into(),
                snapshot_identity: snapshot_identity.into(),
            },
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn future_neighbor(
        family: RawFutureBasisNeighborFamily,
        operation_lane: BasisOperationLaneRequest,
    ) -> Self {
        Self::new(
            RawBasisSelector::FutureNeighbor { family },
            operation_lane,
            RawBasisSourcePath::DirectLifecycleConstructor,
        )
    }

    pub fn with_tenant_scope(mut self, tenant_scope: impl Into<String>) -> Self {
        self.tenant_scope = Some(tenant_scope.into());
        self.raw_digest = self.compute_raw_digest();
        self
    }

    pub fn with_policy_scope(mut self, policy_scope: impl Into<String>) -> Self {
        self.policy_scope = Some(policy_scope.into());
        self.raw_digest = self.compute_raw_digest();
        self
    }

    pub fn with_schema_scope(mut self, schema_scope: impl Into<String>) -> Self {
        self.schema_scope = Some(schema_scope.into());
        self.raw_digest = self.compute_raw_digest();
        self
    }

    pub fn selector(&self) -> &RawBasisSelector {
        &self.selector
    }

    pub fn tenant_scope(&self) -> Option<&str> {
        self.tenant_scope.as_deref()
    }

    pub fn policy_scope(&self) -> Option<&str> {
        self.policy_scope.as_deref()
    }

    pub fn schema_scope(&self) -> Option<&str> {
        self.schema_scope.as_deref()
    }

    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        &self.operation_lane
    }

    pub fn source_path(&self) -> &RawBasisSourcePath {
        &self.source_path
    }

    pub fn raw_digest(&self) -> &str {
        &self.raw_digest
    }

    pub(crate) fn with_source_path(mut self, source_path: RawBasisSourcePath) -> Self {
        self.source_path = source_path;
        self.raw_digest = self.compute_raw_digest();
        self
    }

    fn new(
        selector: RawBasisSelector,
        operation_lane: BasisOperationLaneRequest,
        source_path: RawBasisSourcePath,
    ) -> Self {
        let mut intent = Self {
            selector,
            tenant_scope: None,
            policy_scope: None,
            schema_scope: None,
            operation_lane,
            source_path,
            raw_digest: String::new(),
        };
        intent.raw_digest = intent.compute_raw_digest();
        intent
    }

    fn compute_raw_digest(&self) -> String {
        let mut parts = vec![
            format!("selector:{}", self.selector.as_str()),
            format!("operation_lane:{}", self.operation_lane.as_str()),
            format!("source_path:{}", self.source_path.as_str()),
        ];
        match &self.selector {
            RawBasisSelector::CurrentHead => {}
            RawBasisSelector::BranchHead { branch_identity } => {
                parts.push(format!("branch_identity:{branch_identity}"));
            }
            RawBasisSelector::BranchSnapshot {
                branch_identity,
                snapshot_identity,
            } => {
                parts.push(format!("branch_identity:{branch_identity}"));
                parts.push(format!("snapshot_identity:{snapshot_identity}"));
            }
            RawBasisSelector::RuntimeSnapshot { snapshot_identity }
            | RawBasisSelector::HistoricalSnapshot { snapshot_identity } => {
                parts.push(format!("snapshot_identity:{snapshot_identity}"));
            }
            RawBasisSelector::HistoricalCommit { commit_identity } => {
                parts.push(format!("commit_identity:{commit_identity}"));
            }
            RawBasisSelector::Preview { preview_identity }
            | RawBasisSelector::PreviewDerivedHistorical { preview_identity } => {
                parts.push(format!("preview_identity:{preview_identity}"));
            }
            RawBasisSelector::FutureNeighbor { family } => {
                parts.push(format!("future_neighbor:{}", family.as_str()));
            }
        }
        if let Some(tenant_scope) = &self.tenant_scope {
            parts.push(format!("tenant_scope:{tenant_scope}"));
        }
        if let Some(policy_scope) = &self.policy_scope {
            parts.push(format!("policy_scope:{policy_scope}"));
        }
        if let Some(schema_scope) = &self.schema_scope {
            parts.push(format!("schema_scope:{schema_scope}"));
        }
        hash_parts(&parts)
    }
}
