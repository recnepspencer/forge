use crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictPublicFacade;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthWorkloadCompositionExplainerDisposition {
    MigratedOrdinaryConsumer,
    CappedResidue,
    QueryGap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadCompositionExplainerRow {
    source_path: &'static str,
    surface_name: &'static str,
    owner: &'static str,
    disposition: WorthWorkloadCompositionExplainerDisposition,
    blocker: String,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadCompositionExplainerLedger {
    proof_basis_digests: Vec<String>,
    rows: Vec<WorthWorkloadCompositionExplainerRow>,
}

impl WorthWorkloadCompositionExplainerDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigratedOrdinaryConsumer => "migrated-ordinary-consumer",
            Self::CappedResidue => "capped-residue",
            Self::QueryGap => "query-gap",
        }
    }
}

impl WorthWorkloadCompositionExplainerLedger {
    pub(super) fn current_from_public_facade(
        public_facade: &WorthTouchedGraphConflictPublicFacade,
    ) -> Self {
        let public_proof = public_facade.public_proof();
        let derived_diagnostics = public_facade.derived_diagnostics();

        Self {
            proof_basis_digests: vec![
                public_proof.closeout_digest().to_string(),
                public_proof.proof_chain_digest().to_string(),
                public_proof.milestone_fifteen_seed().seed_digest().to_string(),
                derived_diagnostics.selected_route_identity_digest().to_string(),
                derived_diagnostics.decision_trace_identity_digest().to_string(),
            ],
            rows: vec![WorthWorkloadCompositionExplainerRow {
                source_path: "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/closeout.rs",
                surface_name: "current_worth_workload_ordinary_consumer_sweep_closeout",
                owner: "worth-kernel",
                disposition: WorthWorkloadCompositionExplainerDisposition::MigratedOrdinaryConsumer,
                blocker: "workload-composition route-local status must consume planner-owned public-proof and diagnostic projections instead of reopening route meaning locally".to_string(),
                removal_trigger: format!(
                    "ordinary workload-composition explainers stay bound to selected route {} and decision trace {} from the planner-owned public facade",
                    derived_diagnostics.selected_route_identity_digest(),
                    derived_diagnostics.decision_trace_identity_digest()
                ),
            }],
        }
    }

    pub fn proof_basis_digests(&self) -> &[String] {
        &self.proof_basis_digests
    }

    pub fn rows(&self) -> &[WorthWorkloadCompositionExplainerRow] {
        &self.rows
    }

    pub fn migrated_count(&self) -> usize {
        self.count_rows(WorthWorkloadCompositionExplainerDisposition::MigratedOrdinaryConsumer)
    }

    pub fn capped_residue_count(&self) -> usize {
        self.count_rows(WorthWorkloadCompositionExplainerDisposition::CappedResidue)
    }

    pub fn query_gap_count(&self) -> usize {
        self.count_rows(WorthWorkloadCompositionExplainerDisposition::QueryGap)
    }

    fn count_rows(&self, disposition: WorthWorkloadCompositionExplainerDisposition) -> usize {
        self.rows
            .iter()
            .filter(|row| row.disposition == disposition)
            .count()
    }
}

impl WorthWorkloadCompositionExplainerRow {
    pub fn source_path(&self) -> &str {
        self.source_path
    }

    pub fn surface_name(&self) -> &str {
        self.surface_name
    }

    pub fn owner(&self) -> &str {
        self.owner
    }

    pub const fn disposition(&self) -> WorthWorkloadCompositionExplainerDisposition {
        self.disposition
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}
