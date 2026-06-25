#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyMilestoneNineCloseoutDenialKind {
    EmptySelectedObligationProof,
    PhaseEightSeedMismatch,
    SourceFirewallViolation,
    MissingExecutionBackedAdoptionProof,
    UncappedOldAuthority,
    StaleResidueWithoutDeletionLedger,
    SelectionOnlyProof,
}

impl WorthTopologyMilestoneNineCloseoutDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySelectedObligationProof => "empty-selected-obligation-proof",
            Self::PhaseEightSeedMismatch => "phase-eight-seed-mismatch",
            Self::SourceFirewallViolation => "source-firewall-violation",
            Self::MissingExecutionBackedAdoptionProof => "missing-execution-backed-adoption-proof",
            Self::UncappedOldAuthority => "uncapped-old-authority",
            Self::StaleResidueWithoutDeletionLedger => "stale-residue-without-deletion-ledger",
            Self::SelectionOnlyProof => "selection-only-proof",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineCloseoutDenial {
    kind: WorthTopologyMilestoneNineCloseoutDenialKind,
    authority_digest: String,
    message: String,
    denial_digest: String,
}

impl WorthTopologyMilestoneNineCloseoutDenial {
    pub(in crate::validator_invariant_catalog) fn new(
        kind: WorthTopologyMilestoneNineCloseoutDenialKind,
        authority_digest: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let authority_digest = authority_digest.into();
        let message = message.into();
        let denial_digest = [
            "worth-topo-milestone-nine-closeout-denial-v1",
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

    pub const fn kind(&self) -> WorthTopologyMilestoneNineCloseoutDenialKind {
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

impl std::fmt::Display for WorthTopologyMilestoneNineCloseoutDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.kind.as_str(),
            self.message,
            self.authority_digest
        )
    }
}
