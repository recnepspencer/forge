use crate::workload_composition::planner_owned_routing::derived_diagnostics::WorthTouchedGraphConflictDerivedDiagnosticProjection;
use crate::workload_composition::planner_owned_routing::public_proof::{
    WorthTouchedGraphConflictArchitectureAlignmentReport,
    WorthTouchedGraphConflictMilestoneFifteenSeed, WorthTouchedGraphConflictResidueChain,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictPublicFacadeErrorKind {
    CurrentPublicProofUnavailable,
    CurrentDerivedDiagnosticsUnavailable,
    MismatchedProjectionAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictPublicFacadeError {
    kind: WorthTouchedGraphConflictPublicFacadeErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictPublicProofInspection {
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    closeout_digest: String,
    proof_chain_digest: String,
    source_firewall_digest: String,
    deletion_closeout_digest: String,
    residue_chain: WorthTouchedGraphConflictResidueChain,
    architecture_alignment_report: WorthTouchedGraphConflictArchitectureAlignmentReport,
    milestone_fifteen_seed: WorthTouchedGraphConflictMilestoneFifteenSeed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictPublicFacade {
    public_proof: WorthTouchedGraphConflictPublicProofInspection,
    derived_diagnostics: WorthTouchedGraphConflictDerivedDiagnosticProjection,
}

impl WorthTouchedGraphConflictPublicProofInspection {
    pub(crate) fn new(
        selected_route_identity_digest: String,
        selected_family_identity: String,
        selected_product_identity_digest: String,
        selected_witness_identity_digest: Option<String>,
        closeout_digest: String,
        proof_chain_digest: String,
        source_firewall_digest: String,
        deletion_closeout_digest: String,
        residue_chain: WorthTouchedGraphConflictResidueChain,
        architecture_alignment_report: WorthTouchedGraphConflictArchitectureAlignmentReport,
        milestone_fifteen_seed: WorthTouchedGraphConflictMilestoneFifteenSeed,
    ) -> Self {
        Self {
            selected_route_identity_digest,
            selected_family_identity,
            selected_product_identity_digest,
            selected_witness_identity_digest,
            closeout_digest,
            proof_chain_digest,
            source_firewall_digest,
            deletion_closeout_digest,
            residue_chain,
            architecture_alignment_report,
            milestone_fifteen_seed,
        }
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn proof_chain_digest(&self) -> &str {
        &self.proof_chain_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub fn residue_chain(&self) -> &WorthTouchedGraphConflictResidueChain {
        &self.residue_chain
    }

    pub fn architecture_alignment_report(
        &self,
    ) -> &WorthTouchedGraphConflictArchitectureAlignmentReport {
        &self.architecture_alignment_report
    }

    pub fn milestone_fifteen_seed(&self) -> &WorthTouchedGraphConflictMilestoneFifteenSeed {
        &self.milestone_fifteen_seed
    }

    #[cfg(test)]
    pub(crate) fn with_test_selected_witness_identity_override(
        mut self,
        digest: Option<&str>,
    ) -> Self {
        self.selected_witness_identity_digest = digest.map(str::to_string);
        self
    }
}

impl WorthTouchedGraphConflictPublicFacade {
    pub(crate) fn new(
        public_proof: WorthTouchedGraphConflictPublicProofInspection,
        derived_diagnostics: WorthTouchedGraphConflictDerivedDiagnosticProjection,
    ) -> Self {
        Self {
            public_proof,
            derived_diagnostics,
        }
    }

    pub fn public_proof(&self) -> &WorthTouchedGraphConflictPublicProofInspection {
        &self.public_proof
    }

    pub fn derived_diagnostics(&self) -> &WorthTouchedGraphConflictDerivedDiagnosticProjection {
        &self.derived_diagnostics
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        self.public_proof.selected_route_identity_digest()
    }

    #[cfg(test)]
    pub(crate) fn with_test_public_proof_witness_identity_override(
        mut self,
        digest: Option<&str>,
    ) -> Self {
        self.public_proof = self
            .public_proof
            .with_test_selected_witness_identity_override(digest);
        self
    }
}

impl WorthTouchedGraphConflictPublicFacadeError {
    pub(crate) fn new(
        kind: WorthTouchedGraphConflictPublicFacadeErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> WorthTouchedGraphConflictPublicFacadeErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
