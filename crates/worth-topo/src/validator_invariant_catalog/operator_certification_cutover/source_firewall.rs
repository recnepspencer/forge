use std::collections::BTreeSet;

use crate::validator_invariant_catalog::operator_certification_cutover::WorthTopologyOperatorCertificationOldExpectationResidueReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyOperatorCertificationCutoverSourceFirewallReport {
    scanned_file_count: usize,
    scanned_source_paths: Vec<String>,
    allowed_capped_residue_paths: Vec<String>,
    violations: Vec<String>,
    report_digest: String,
}

impl WorthTopologyOperatorCertificationCutoverSourceFirewallReport {
    pub const CURRENT_SCAN_SOURCE_PATHS: [&'static str; 6] = [
        "certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs",
        "certification/topology_operator_closeout/validation_breadth_row.rs",
        "topology_operators/application/declaration_entry/execution_finalize.rs",
        "topology_operators/declaration_entry/mod.rs",
        "topology_operators/edge_split_blueprint/required_phase_1_validator_lanes.rs",
        "topology_operators/loop_reconstruction_blueprint/phase_2_inventory/validator_rows.rs",
    ];

    pub(in crate::validator_invariant_catalog) fn current_with_capped_residue(
        residue: &WorthTopologyOperatorCertificationOldExpectationResidueReport,
    ) -> Self {
        Self::from_source_pairs_with_capped_residue(
            [
                (
                    "certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs",
                    include_str!("../../certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs"),
                ),
                (
                    "certification/topology_operator_closeout/validation_breadth_row.rs",
                    include_str!("../../certification/topology_operator_closeout/validation_breadth_row.rs"),
                ),
                (
                    "topology_operators/application/declaration_entry/execution_finalize.rs",
                    include_str!("../../topology_operators/application/declaration_entry/execution_finalize.rs"),
                ),
                (
                    "topology_operators/declaration_entry/mod.rs",
                    include_str!("../../topology_operators/declaration_entry/mod.rs"),
                ),
                (
                    "topology_operators/edge_split_blueprint/required_phase_1_validator_lanes.rs",
                    include_str!("../../topology_operators/edge_split_blueprint/required_phase_1_validator_lanes.rs"),
                ),
                (
                    "topology_operators/loop_reconstruction_blueprint/phase_2_inventory/validator_rows.rs",
                    include_str!("../../topology_operators/loop_reconstruction_blueprint/phase_2_inventory/validator_rows.rs"),
                ),
            ],
            residue,
        )
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn from_source_pairs(
        sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        Self::from_source_pairs_and_allowed_residue_paths(sources, &BTreeSet::new())
    }

    fn from_source_pairs_with_capped_residue(
        sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
        residue: &WorthTopologyOperatorCertificationOldExpectationResidueReport,
    ) -> Self {
        let allowed_residue_paths = residue
            .rows()
            .iter()
            .filter(|row| row.status().is_capped())
            .map(|row| row.source_path())
            .collect::<BTreeSet<_>>();
        Self::from_source_pairs_and_allowed_residue_paths(sources, &allowed_residue_paths)
    }

    fn from_source_pairs_and_allowed_residue_paths(
        sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
        allowed_residue_paths: &BTreeSet<&str>,
    ) -> Self {
        let mut scanned_file_count = 0;
        let mut scanned_source_paths = Vec::new();
        let mut violations = Vec::new();
        for (path, source) in sources {
            scanned_file_count += 1;
            let path = path.into();
            let source = source.into();
            scanned_source_paths.push(path.clone());
            for pattern in FORBIDDEN_AUTHORITY_PATTERNS {
                if source_contains_forbidden_pattern(&source, pattern)
                    && !allowed_residue_paths.contains(path.as_str())
                {
                    violations.push(format!("{path}::{pattern}"));
                }
            }
        }
        scanned_source_paths.sort();
        scanned_source_paths.dedup();
        let allowed_capped_residue_paths = allowed_residue_paths
            .iter()
            .map(|path| (*path).to_string())
            .collect::<Vec<_>>();
        let mut digest_parts = vec![
            "worth-topo-operator-certification-cutover-source-firewall-v1".to_string(),
            format!("scanned-file-count:{scanned_file_count}"),
            format!("violation-count:{}", violations.len()),
        ];
        digest_parts.extend(
            scanned_source_paths
                .iter()
                .map(|path| format!("scanned:{path}")),
        );
        digest_parts.extend(
            allowed_capped_residue_paths
                .iter()
                .map(|path| format!("capped-residue:{path}")),
        );
        digest_parts.extend(
            violations
                .iter()
                .map(|violation| format!("violation:{violation}")),
        );
        Self {
            scanned_file_count,
            scanned_source_paths,
            allowed_capped_residue_paths,
            violations,
            report_digest: digest_parts.join("|"),
        }
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn clean_for_cutover() -> Self {
        Self::from_source_pairs([(
            "operator_certification_cutover/closeout.rs",
            "selected_obligation_closeout_rows support_posture_rows old_expectation_residue",
        )])
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn scanned_source_paths(&self) -> &[String] {
        &self.scanned_source_paths
    }

    pub fn allowed_capped_residue_paths(&self) -> &[String] {
        &self.allowed_capped_residue_paths
    }

    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn violations(&self) -> &[String] {
        &self.violations
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub const fn forbidden_authority_patterns() -> &'static [&'static str] {
        &FORBIDDEN_AUTHORITY_PATTERNS
    }
}

fn source_contains_forbidden_pattern(source: &str, pattern: &str) -> bool {
    source.match_indices(pattern).any(|(start, _)| {
        let end = start + pattern.len();
        let previous = source[..start].chars().next_back();
        let next = source[end..].chars().next();
        !is_identifier_part(previous) && !is_identifier_part(next)
    })
}

fn is_identifier_part(character: Option<char>) -> bool {
    character.is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
}

const FORBIDDEN_AUTHORITY_PATTERNS: [&str; 12] = [
    "milestone_three_validator_expectations",
    "CertificationValidatorExpectation",
    "validator_expectations",
    "validator_family_count",
    "validator_name_count",
    "derived_validation_row_count",
    "query_invariant_validator",
    "spatial_validator",
    "required_phase_1_validator_rows",
    "required_phase_2_validator_rows",
    "operator_local_invariant_hook",
    "local_validator_array",
];
