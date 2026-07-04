use super::architecture_alignment_report::WorthTouchedGraphConflictArchitectureAlignmentReport;
use super::milestone_fifteen_seed::WorthTouchedGraphConflictMilestoneFifteenSeed;
use super::proof_chain::WorthTouchedGraphConflictProofChain;
use super::residue_chain::WorthTouchedGraphConflictResidueChain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictPublicCloseoutErrorKind {
    CurrentProofUnavailable,
    SourceFirewallViolation,
    MismatchedFirewallProof,
    OrdinaryConsumerDependencyStillOpen,
    IncompleteProofChain,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictPublicCloseoutError {
    kind: WorthTouchedGraphConflictPublicCloseoutErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictPublicCloseout {
    pub(crate) proof_chain: WorthTouchedGraphConflictProofChain,
    pub(crate) residue_chain: WorthTouchedGraphConflictResidueChain,
    pub(crate) architecture_alignment_report: WorthTouchedGraphConflictArchitectureAlignmentReport,
    pub(crate) source_firewall_digest: String,
    pub(crate) deletion_closeout_digest: String,
    pub(crate) milestone_fifteen_seed: WorthTouchedGraphConflictMilestoneFifteenSeed,
    pub(crate) closeout_digest: String,
}

impl WorthTouchedGraphConflictPublicCloseout {
    pub fn current() -> Result<Self, WorthTouchedGraphConflictPublicCloseoutError> {
        super::current::current_worth_touched_graph_conflict_public_closeout()
    }

    pub(crate) fn proof_chain(&self) -> &WorthTouchedGraphConflictProofChain {
        &self.proof_chain
    }

    pub fn residue_chain(&self) -> &WorthTouchedGraphConflictResidueChain {
        &self.residue_chain
    }

    pub fn architecture_alignment_report(
        &self,
    ) -> &WorthTouchedGraphConflictArchitectureAlignmentReport {
        &self.architecture_alignment_report
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub fn milestone_fifteen_seed(&self) -> &WorthTouchedGraphConflictMilestoneFifteenSeed {
        &self.milestone_fifteen_seed
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        self.proof_chain.selected_route_identity_digest()
    }

    pub fn selected_family_identity(&self) -> &str {
        self.proof_chain
            .topology_query_selected_equivalence_family_identity()
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        self.milestone_fifteen_seed
            .topology_compiled_product_identity_digest()
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.proof_chain
            .topology_query_reuse_decision_identity_digest()
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    #[cfg(test)]
    pub(crate) fn with_test_architecture_alignment_report(
        mut self,
        architecture_alignment_report: WorthTouchedGraphConflictArchitectureAlignmentReport,
    ) -> Self {
        self.architecture_alignment_report = architecture_alignment_report;
        self
    }
}

impl WorthTouchedGraphConflictPublicCloseoutError {
    pub(crate) fn new(
        kind: WorthTouchedGraphConflictPublicCloseoutErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> WorthTouchedGraphConflictPublicCloseoutErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
