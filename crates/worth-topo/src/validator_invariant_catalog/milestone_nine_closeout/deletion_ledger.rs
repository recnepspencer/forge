use crate::validator_invariant_catalog::WorthTopologyOperatorCertificationCutoverCloseout;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyMilestoneNineDeletionDisposition {
    Deleted,
    CappedResidue,
    CertificationOnly,
}

impl WorthTopologyMilestoneNineDeletionDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deleted => "deleted",
            Self::CappedResidue => "capped-residue",
            Self::CertificationOnly => "certification-only",
        }
    }

    pub const fn closes_old_authority(self) -> bool {
        matches!(
            self,
            Self::Deleted | Self::CappedResidue | Self::CertificationOnly
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineDeletionLedgerRow {
    source_path: String,
    old_authority_kind: String,
    owner: String,
    disposition: WorthTopologyMilestoneNineDeletionDisposition,
    blocker: String,
    removal_trigger: String,
    allowed_forbidden_pattern_hits: Vec<(String, usize)>,
    row_digest: String,
}

impl WorthTopologyMilestoneNineDeletionLedgerRow {
    fn new(
        source_path: impl Into<String>,
        old_authority_kind: impl Into<String>,
        owner: impl Into<String>,
        disposition: WorthTopologyMilestoneNineDeletionDisposition,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
        allowed_forbidden_pattern_hits: impl IntoIterator<Item = (impl Into<String>, usize)>,
    ) -> Self {
        let source_path = source_path.into();
        let old_authority_kind = old_authority_kind.into();
        let owner = owner.into();
        let blocker = blocker.into();
        let removal_trigger = removal_trigger.into();
        let allowed_forbidden_pattern_hits = allowed_forbidden_pattern_hits
            .into_iter()
            .map(|(pattern, count)| (pattern.into(), count))
            .collect::<Vec<_>>();
        let mut digest_parts = vec![
            "worth-topo-milestone-nine-deletion-ledger-row-v1",
            source_path.as_str(),
            old_authority_kind.as_str(),
            owner.as_str(),
            disposition.as_str(),
            blocker.as_str(),
            removal_trigger.as_str(),
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
        digest_parts.extend(
            allowed_forbidden_pattern_hits
                .iter()
                .map(|(pattern, count)| format!("allowed-pattern:{pattern}:{count}")),
        );
        let row_digest = digest_parts.join("|");
        Self {
            source_path,
            old_authority_kind,
            owner,
            disposition,
            blocker,
            removal_trigger,
            allowed_forbidden_pattern_hits,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn old_authority_kind(&self) -> &str {
        &self.old_authority_kind
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub const fn disposition(&self) -> WorthTopologyMilestoneNineDeletionDisposition {
        self.disposition
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub fn allowed_forbidden_pattern_hits(&self) -> &[(String, usize)] {
        &self.allowed_forbidden_pattern_hits
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyMilestoneNineDeletionLedgerReport {
    rows: Vec<WorthTopologyMilestoneNineDeletionLedgerRow>,
    report_digest: String,
}

impl WorthTopologyMilestoneNineDeletionLedgerReport {
    pub(in crate::validator_invariant_catalog) fn from_operator_cutover(
        cutover: &WorthTopologyOperatorCertificationCutoverCloseout,
    ) -> Self {
        let mut rows = vec![
            WorthTopologyMilestoneNineDeletionLedgerRow::new(
                "validation/rule_registry.rs",
                "static-derived-topology-rule-specs",
                "worth-topo",
                WorthTopologyMilestoneNineDeletionDisposition::CertificationOnly,
                "Milestone 10 must replace rule arrays with declare-once catalog reads",
                "validator and invariant families are read from catalog descriptors only",
                [("DERIVED_TOPOLOGY_RULE_SPECS", 3)],
            ),
            WorthTopologyMilestoneNineDeletionLedgerRow::new(
                "runtime_support.rs",
                "milestone-one-invariant-registration-pack",
                "worth-topo",
                WorthTopologyMilestoneNineDeletionDisposition::CertificationOnly,
                "Milestone 10 must route runtime support through catalog declarations",
                "runtime support receives only catalog-backed graph read receipts",
                [("milestone_one_invariant_registrations", 2)],
            ),
        ];
        rows.extend(cutover.old_expectation_residue().rows().iter().map(|row| {
            WorthTopologyMilestoneNineDeletionLedgerRow::new(
                row.source_path(),
                "operator-local-validator-expectation",
                "worth-topo",
                WorthTopologyMilestoneNineDeletionDisposition::CappedResidue,
                "Phase 9 keeps this only as deletion evidence from the Phase 7 cutover",
                "operator certification cutover rows are the only executable proof",
                allowed_pattern_hits_for_residue_path(row.source_path()),
            )
        }));
        Self::from_rows(rows)
    }

    pub(in crate::validator_invariant_catalog) fn from_rows(
        rows: impl IntoIterator<Item = WorthTopologyMilestoneNineDeletionLedgerRow>,
    ) -> Self {
        let rows = rows.into_iter().collect::<Vec<_>>();
        let mut digest_parts = vec![
            "worth-topo-milestone-nine-deletion-ledger-report-v1".to_string(),
            format!("row-count:{}", rows.len()),
        ];
        digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        Self {
            rows,
            report_digest: digest_parts.join("|"),
        }
    }

    pub fn rows(&self) -> &[WorthTopologyMilestoneNineDeletionLedgerRow] {
        &self.rows
    }

    pub fn closed_old_authority_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.disposition().closes_old_authority())
            .count()
    }

    pub fn capped_residue_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                row.disposition() == WorthTopologyMilestoneNineDeletionDisposition::CappedResidue
            })
            .count()
    }

    pub fn whole_view_certification_only_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                row.disposition()
                    == WorthTopologyMilestoneNineDeletionDisposition::CertificationOnly
            })
            .count()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn allowed_pattern_hits_for_residue_path(source_path: &str) -> Vec<(&'static str, usize)> {
    match source_path {
        "certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs" => {
            vec![
                ("milestone_three_validator_expectations", 1),
                ("CertificationValidatorExpectation", 3),
                ("validator_expectations", 1),
                ("derived_validation_row_count", 2),
            ]
        }
        "certification/topology_operator_closeout/validation_breadth_row.rs" => vec![
            ("validator_family_count", 1),
            ("validator_name_count", 1),
            ("derived_validation_row_count", 1),
        ],
        "topology_operators/loop_reconstruction_blueprint/phase_2_inventory/validator_rows.rs" => {
            vec![("query_invariant_validator", 21), ("spatial_validator", 25)]
        }
        _ => Vec::new(),
    }
}
