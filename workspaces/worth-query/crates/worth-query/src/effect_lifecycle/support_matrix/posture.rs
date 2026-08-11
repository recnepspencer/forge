#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectSupportPosture {
    Admitted,
    Advisory,
    Denied,
    RebindRequired,
    Deferred,
    Unsupported,
}

impl EffectSupportPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Advisory => "advisory",
            Self::Denied => "denied",
            Self::RebindRequired => "rebind_required",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectSupportCause {
    Supported,
    AdvisoryOnlyExecution,
    PreviewRebindRequired,
    BranchAuthorityRequired,
    StoreBackedExecutionDeferred,
    DurableReplayDeferred,
    UnsupportedForBasisFamily,
}

impl EffectSupportCause {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::AdvisoryOnlyExecution => "advisory_only_execution",
            Self::PreviewRebindRequired => "preview_rebind_required",
            Self::BranchAuthorityRequired => "branch_authority_required",
            Self::StoreBackedExecutionDeferred => "store_backed_execution_deferred",
            Self::DurableReplayDeferred => "durable_replay_deferred",
            Self::UnsupportedForBasisFamily => "unsupported_for_basis_family",
        }
    }
}
