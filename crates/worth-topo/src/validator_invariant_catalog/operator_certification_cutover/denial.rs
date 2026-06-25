#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyOperatorCertificationCutoverDenialKind {
    EmptyEnforcementReceiptSet,
    SourceFirewallViolation,
    UncappedOldExpectationAuthority,
}

impl WorthTopologyOperatorCertificationCutoverDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyEnforcementReceiptSet => "empty-enforcement-receipt-set",
            Self::SourceFirewallViolation => "source-firewall-violation",
            Self::UncappedOldExpectationAuthority => "uncapped-old-expectation-authority",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyOperatorCertificationCutoverDenial {
    kind: WorthTopologyOperatorCertificationCutoverDenialKind,
    authority_digest: String,
    message: String,
    denial_digest: String,
}

impl WorthTopologyOperatorCertificationCutoverDenial {
    pub(in crate::validator_invariant_catalog) fn new(
        kind: WorthTopologyOperatorCertificationCutoverDenialKind,
        authority_digest: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let authority_digest = authority_digest.into();
        let message = message.into();
        let denial_digest = [
            "worth-topo-operator-certification-cutover-denial-v1",
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

    pub const fn kind(&self) -> WorthTopologyOperatorCertificationCutoverDenialKind {
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

impl std::fmt::Display for WorthTopologyOperatorCertificationCutoverDenial {
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
