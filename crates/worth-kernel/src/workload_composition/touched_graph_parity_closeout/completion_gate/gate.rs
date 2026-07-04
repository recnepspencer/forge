use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;
use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictPublicCloseout;
use crate::workload_composition::{
    LiveCoverageLedger, RepresentativeSelectedRouteParityPath,
    WorthTouchedGraphCrossFamilyCloseoutMatrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RoadmapCompletionFirewallCertification {
    source_firewall_report_digest: String,
    deletion_closeout_digest: String,
    covered_forbidden_surface_count: usize,
    closeout_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphRoadmapCompletionGate {
    closeout_matrix: WorthTouchedGraphCrossFamilyCloseoutMatrix,
    readiness_handoff: TouchedGraphParityReadinessInput,
    representative_path: RepresentativeSelectedRouteParityPath,
    public_closeout: WorthTouchedGraphConflictPublicCloseout,
    source_firewall_certification: RoadmapCompletionFirewallCertification,
    live_coverage_ledger: LiveCoverageLedger,
    covered_family_kinds: Vec<TouchedGraphParityFamilyKind>,
    completion_digest: String,
    is_complete: bool,
}

impl WorthTouchedGraphRoadmapCompletionGate {
    #[allow(clippy::too_many_arguments)]
    fn new_unvalidated(
        closeout_matrix: WorthTouchedGraphCrossFamilyCloseoutMatrix,
        readiness_handoff: TouchedGraphParityReadinessInput,
        representative_path: RepresentativeSelectedRouteParityPath,
        public_closeout: WorthTouchedGraphConflictPublicCloseout,
        source_firewall_certification: RoadmapCompletionFirewallCertification,
        live_coverage_ledger: LiveCoverageLedger,
    ) -> Self {
        let covered_family_kinds = closeout_matrix
            .rows()
            .iter()
            .filter(|row| row.is_covered_family())
            .map(|row| row.family_kind())
            .collect::<Vec<_>>();
        let completion_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:touched-graph-roadmap-completion-gate:v1".to_string(),
                format!("matrix:{}", closeout_matrix.matrix_digest()),
                format!(
                    "claim:{}",
                    closeout_matrix.closeout_architecture_claim_digest()
                ),
                format!(
                    "readiness:{}",
                    readiness_handoff.architecture_claim_digest()
                ),
                format!("residue:{}", readiness_handoff.residue_digest()),
                format!("public-closeout:{}", public_closeout.closeout_digest()),
                format!(
                    "alignment:{}",
                    public_closeout
                        .architecture_alignment_report()
                        .report_digest()
                ),
                format!(
                    "firewall-closeout:{}",
                    source_firewall_certification.closeout_digest()
                ),
                format!("ledger:{}", live_coverage_ledger.ledger_digest()),
            ],
        );
        Self {
            closeout_matrix,
            readiness_handoff,
            representative_path,
            public_closeout,
            source_firewall_certification,
            live_coverage_ledger,
            covered_family_kinds,
            completion_digest,
            is_complete: false,
        }
    }

    pub fn closeout_matrix(&self) -> &WorthTouchedGraphCrossFamilyCloseoutMatrix {
        &self.closeout_matrix
    }

    pub fn readiness_handoff(&self) -> &TouchedGraphParityReadinessInput {
        &self.readiness_handoff
    }

    pub fn representative_path(&self) -> &RepresentativeSelectedRouteParityPath {
        &self.representative_path
    }

    pub fn public_closeout(&self) -> &WorthTouchedGraphConflictPublicCloseout {
        &self.public_closeout
    }

    pub fn source_firewall_report_digest(&self) -> &str {
        self.source_firewall_certification
            .source_firewall_report_digest()
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        self.source_firewall_certification
            .deletion_closeout_digest()
    }

    pub const fn covered_forbidden_surface_count(&self) -> usize {
        self.source_firewall_certification
            .covered_forbidden_surface_count()
    }

    pub fn source_firewall_closeout_digest(&self) -> &str {
        self.source_firewall_certification.closeout_digest()
    }

    pub fn live_coverage_ledger(&self) -> &LiveCoverageLedger {
        &self.live_coverage_ledger
    }

    pub fn covered_family_kinds(&self) -> &[TouchedGraphParityFamilyKind] {
        &self.covered_family_kinds
    }

    pub fn closeout_architecture_claim_digest(&self) -> &str {
        self.closeout_matrix.closeout_architecture_claim_digest()
    }

    pub fn completion_digest(&self) -> &str {
        &self.completion_digest
    }

    pub const fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub(super) fn candidate(
        closeout_matrix: WorthTouchedGraphCrossFamilyCloseoutMatrix,
        readiness_handoff: TouchedGraphParityReadinessInput,
        representative_path: RepresentativeSelectedRouteParityPath,
        public_closeout: WorthTouchedGraphConflictPublicCloseout,
        source_firewall_certification: RoadmapCompletionFirewallCertification,
        live_coverage_ledger: LiveCoverageLedger,
    ) -> Self {
        Self::new_unvalidated(
            closeout_matrix,
            readiness_handoff,
            representative_path,
            public_closeout,
            source_firewall_certification,
            live_coverage_ledger,
        )
    }

    pub(super) fn mark_complete(mut self) -> Self {
        self.is_complete = true;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_closeout_matrix(
        mut self,
        closeout_matrix: WorthTouchedGraphCrossFamilyCloseoutMatrix,
    ) -> Self {
        self.closeout_matrix = closeout_matrix;
        self.covered_family_kinds = self
            .closeout_matrix
            .rows()
            .iter()
            .filter(|row| row.is_covered_family())
            .map(|row| row.family_kind())
            .collect();
        self.is_complete = false;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_live_coverage_ledger(
        mut self,
        live_coverage_ledger: LiveCoverageLedger,
    ) -> Self {
        self.live_coverage_ledger = live_coverage_ledger;
        self.is_complete = false;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_public_closeout(
        mut self,
        public_closeout: WorthTouchedGraphConflictPublicCloseout,
    ) -> Self {
        self.public_closeout = public_closeout;
        self.is_complete = false;
        self
    }
}

impl RoadmapCompletionFirewallCertification {
    pub(crate) fn new(
        source_firewall_report_digest: impl Into<String>,
        deletion_closeout_digest: impl Into<String>,
        covered_forbidden_surface_count: usize,
        closeout_digest: impl Into<String>,
    ) -> Self {
        Self {
            source_firewall_report_digest: source_firewall_report_digest.into(),
            deletion_closeout_digest: deletion_closeout_digest.into(),
            covered_forbidden_surface_count,
            closeout_digest: closeout_digest.into(),
        }
    }

    pub fn source_firewall_report_digest(&self) -> &str {
        &self.source_firewall_report_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub const fn covered_forbidden_surface_count(&self) -> usize {
        self.covered_forbidden_surface_count
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
