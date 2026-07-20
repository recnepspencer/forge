#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryOperationBindingCounters {
    pub authority_checks: usize,
    pub operation_lookups: usize,
    pub required_domain_lookups: usize,
    pub graph_binding_lookups: usize,
    pub graph_provider_contacts: usize,
    pub conditional_lowering_lookups: usize,
    pub planning_steps: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationBindingDenialKind {
    DomainAuthority,
    OperationNotInstalled,
    RequiredDomainNotInstalled,
    GraphParticipationNotInstalled,
    GraphRoleMismatch,
    GraphAuthorityInsufficient,
    BasisLaneInsufficient,
    BasisExecutionUnsupported,
    ConditionalLoweringNotInstalled,
    ConditionalLoweringDrift,
    IncoherentAuthoritySet,
    CommitAuthorityMismatch,
    CompensationUndeclared,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationBindingDenial {
    kind: WorthQueryOperationBindingDenialKind,
    detail: String,
    counters: WorthQueryOperationBindingCounters,
}

impl WorthQueryOperationBindingDenial {
    pub(super) fn new(
        kind: WorthQueryOperationBindingDenialKind,
        detail: impl Into<String>,
        counters: WorthQueryOperationBindingCounters,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters,
        }
    }

    pub fn kind(&self) -> WorthQueryOperationBindingDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counters(&self) -> WorthQueryOperationBindingCounters {
        self.counters
    }
}
