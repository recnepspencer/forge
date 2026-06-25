#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologySelectedGraphObligationEnforcementDenialKind {
    MissingExecutionBackedAdoptionProof,
    MissingQueryExecutionRow,
    ExecutionEnvelopeMismatch,
    UnsupportedQueryStatus,
    SourceFirewallViolation,
}

impl WorthTopologySelectedGraphObligationEnforcementDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingExecutionBackedAdoptionProof => "missing-execution-backed-adoption-proof",
            Self::MissingQueryExecutionRow => "missing-query-execution-row",
            Self::ExecutionEnvelopeMismatch => "execution-envelope-mismatch",
            Self::UnsupportedQueryStatus => "unsupported-query-status",
            Self::SourceFirewallViolation => "source-firewall-violation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologySelectedGraphObligationEnforcementDenial {
    kind: WorthTopologySelectedGraphObligationEnforcementDenialKind,
    authority_digest: String,
    message: String,
    denial_digest: String,
}

impl WorthTopologySelectedGraphObligationEnforcementDenial {
    pub(in crate::validator_invariant_catalog) fn new(
        kind: WorthTopologySelectedGraphObligationEnforcementDenialKind,
        authority_digest: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let authority_digest = authority_digest.into();
        let message = message.into();
        let denial_digest = [
            "worth-topo-selected-graph-obligation-enforcement-denial-v1",
            kind.as_str(),
            authority_digest.as_str(),
            message.as_str(),
        ]
        .join("|");
        Self {
            kind,
            authority_digest,
            message,
            denial_digest,
        }
    }

    pub const fn kind(&self) -> WorthTopologySelectedGraphObligationEnforcementDenialKind {
        self.kind
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for WorthTopologySelectedGraphObligationEnforcementDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at `{}`: {}",
            self.kind.as_str(),
            self.authority_digest,
            self.message
        )
    }
}
