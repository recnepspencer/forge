#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NormalizedBasisFamily {
    CurrentHead,
    BranchHead,
    BranchSnapshot,
    RuntimeSnapshot,
    HistoricalSnapshot,
    HistoricalCommit,
    Preview,
    PreviewDerivedHistorical,
}

impl NormalizedBasisFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::BranchHead => "branch_head",
            Self::BranchSnapshot => "branch_snapshot",
            Self::RuntimeSnapshot => "runtime_snapshot",
            Self::HistoricalSnapshot => "historical_snapshot",
            Self::HistoricalCommit => "historical_commit",
            Self::Preview => "preview",
            Self::PreviewDerivedHistorical => "preview_derived_historical",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisAuthorityPosture {
    RuntimeBackedCurrentHead,
    RuntimeBackedBranch,
    RuntimeBackedHistorical,
    PreviewScoped,
}

impl BasisAuthorityPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeBackedCurrentHead => "runtime_backed_current_head",
            Self::RuntimeBackedBranch => "runtime_backed_branch",
            Self::RuntimeBackedHistorical => "runtime_backed_historical",
            Self::PreviewScoped => "preview_scoped",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisTenantSchemaPosture {
    Unscoped,
    TenantScoped,
    PolicyScoped,
    SchemaScoped,
    TenantAndPolicyScoped,
    TenantAndSchemaScoped,
    PolicyAndSchemaScoped,
    TenantPolicyAndSchemaScoped,
}

impl BasisTenantSchemaPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unscoped => "unscoped",
            Self::TenantScoped => "tenant_scoped",
            Self::PolicyScoped => "policy_scoped",
            Self::SchemaScoped => "schema_scoped",
            Self::TenantAndPolicyScoped => "tenant_and_policy_scoped",
            Self::TenantAndSchemaScoped => "tenant_and_schema_scoped",
            Self::PolicyAndSchemaScoped => "policy_and_schema_scoped",
            Self::TenantPolicyAndSchemaScoped => "tenant_policy_and_schema_scoped",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisNormalizationCounters {
    raw_intent_width: usize,
    normalized_family_count: usize,
    source_path_count: usize,
    rejection_width: usize,
}

impl BasisNormalizationCounters {
    pub fn raw_intent_width(&self) -> usize {
        self.raw_intent_width
    }

    pub fn normalized_family_count(&self) -> usize {
        self.normalized_family_count
    }

    pub fn source_path_count(&self) -> usize {
        self.source_path_count
    }

    pub fn rejection_width(&self) -> usize {
        self.rejection_width
    }

    pub(super) fn admitted() -> Self {
        Self {
            raw_intent_width: 1,
            normalized_family_count: 1,
            source_path_count: 1,
            rejection_width: 0,
        }
    }

    pub(super) fn denied() -> Self {
        Self {
            raw_intent_width: 1,
            normalized_family_count: 0,
            source_path_count: 1,
            rejection_width: 1,
        }
    }
}
