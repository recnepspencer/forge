#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyOperatorCertificationOldExpectationResidueStatus {
    ComparisonOnly,
    DeletionProofOnly,
    UncappedAuthority,
}

impl WorthTopologyOperatorCertificationOldExpectationResidueStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComparisonOnly => "comparison-only",
            Self::DeletionProofOnly => "deletion-proof-only",
            Self::UncappedAuthority => "uncapped-authority",
        }
    }

    pub const fn is_capped(self) -> bool {
        matches!(self, Self::ComparisonOnly | Self::DeletionProofOnly)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyOperatorCertificationOldExpectationResidueRow {
    source_path: String,
    residue_kind: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    status: WorthTopologyOperatorCertificationOldExpectationResidueStatus,
    row_digest: String,
}

impl WorthTopologyOperatorCertificationOldExpectationResidueRow {
    pub fn capped_comparison(
        source_path: impl Into<String>,
        residue_kind: impl Into<String>,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self::new(
            source_path,
            residue_kind,
            owner,
            blocker,
            removal_trigger,
            WorthTopologyOperatorCertificationOldExpectationResidueStatus::ComparisonOnly,
        )
    }

    pub fn deletion_proof(
        source_path: impl Into<String>,
        residue_kind: impl Into<String>,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self::new(
            source_path,
            residue_kind,
            owner,
            blocker,
            removal_trigger,
            WorthTopologyOperatorCertificationOldExpectationResidueStatus::DeletionProofOnly,
        )
    }

    pub fn uncapped_authority(
        source_path: impl Into<String>,
        residue_kind: impl Into<String>,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self::new(
            source_path,
            residue_kind,
            owner,
            blocker,
            removal_trigger,
            WorthTopologyOperatorCertificationOldExpectationResidueStatus::UncappedAuthority,
        )
    }

    fn new(
        source_path: impl Into<String>,
        residue_kind: impl Into<String>,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
        status: WorthTopologyOperatorCertificationOldExpectationResidueStatus,
    ) -> Self {
        let source_path = source_path.into();
        let residue_kind = residue_kind.into();
        let owner = owner.into();
        let blocker = blocker.into();
        let removal_trigger = removal_trigger.into();
        let row_digest = [
            "worth-topo-operator-certification-old-expectation-residue-row-v1",
            source_path.as_str(),
            residue_kind.as_str(),
            owner.as_str(),
            blocker.as_str(),
            removal_trigger.as_str(),
            status.as_str(),
        ]
        .join("|");
        Self {
            source_path,
            residue_kind,
            owner,
            blocker,
            removal_trigger,
            status,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn status(&self) -> WorthTopologyOperatorCertificationOldExpectationResidueStatus {
        self.status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyOperatorCertificationOldExpectationResidueReport {
    rows: Vec<WorthTopologyOperatorCertificationOldExpectationResidueRow>,
    report_digest: String,
}

impl WorthTopologyOperatorCertificationOldExpectationResidueReport {
    pub fn from_rows(
        rows: impl IntoIterator<Item = WorthTopologyOperatorCertificationOldExpectationResidueRow>,
    ) -> Self {
        let rows = rows.into_iter().collect::<Vec<_>>();
        let mut digest_parts = vec![
            "worth-topo-operator-certification-old-expectation-residue-report-v1".to_string(),
            format!("row-count:{}", rows.len()),
        ];
        digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        Self {
            rows,
            report_digest: digest_parts.join("|"),
        }
    }

    pub fn empty() -> Self {
        Self::from_rows([])
    }

    pub fn current_capped_migration_residue() -> Self {
        Self::from_rows([
            WorthTopologyOperatorCertificationOldExpectationResidueRow::capped_comparison(
                "certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs",
                "validator-expectation-array",
                "worth-topo",
                "Phase 8 hard deletion must remove old expectation-array authority",
                "operator certification cutover rows become the only closeout proof",
            ),
            WorthTopologyOperatorCertificationOldExpectationResidueRow::capped_comparison(
                "certification/topology_operator_closeout/validation_breadth_row.rs",
                "validation-breadth-row",
                "worth-topo",
                "Phase 8 hard deletion must demote validation breadth to deletion evidence",
                "selected obligation closeout row counts replace validator breadth proof",
            ),
            WorthTopologyOperatorCertificationOldExpectationResidueRow::capped_comparison(
                "topology_operators/edge_split_blueprint/required_phase_1_validator_lanes.rs",
                "blueprint-validator-row-inventory",
                "worth-topo",
                "Phase 8 hard deletion must remove blueprint-local validator row authority",
                "touched graph catalog routing covers edge-split operator obligations",
            ),
            WorthTopologyOperatorCertificationOldExpectationResidueRow::capped_comparison(
                "topology_operators/loop_reconstruction_blueprint/phase_2_inventory/validator_rows.rs",
                "blueprint-validator-row-inventory",
                "worth-topo",
                "Phase 8 hard deletion must remove blueprint-local validator row authority",
                "touched graph catalog routing covers loop-reconstruction operator obligations",
            ),
        ])
    }

    pub fn is_capped(&self) -> bool {
        self.rows.iter().all(|row| row.status().is_capped())
    }

    pub fn uncapped_authority_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| !row.status().is_capped())
            .count()
    }

    pub fn capped_authority_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.status().is_capped())
            .count()
    }

    pub fn rows(&self) -> &[WorthTopologyOperatorCertificationOldExpectationResidueRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
