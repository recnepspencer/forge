use crate::identity::hash_parts;

use super::fixtures::{
    control_row_set_lifecycle, grouped_worth_lifecycle, parity_row_set_lifecycle,
};
use super::oracle_comparison_terms::{
    grouped_worth_actual_digest, grouped_worth_expected_digest, row_set_control_actual_digest,
    row_set_control_expected_digest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionOracleLane {
    ControlLane,
    HostileLane,
    ParityLane,
}

impl ProjectionConsumptionOracleLane {
    fn as_str(&self) -> &'static str {
        match self {
            Self::ControlLane => "control_lane",
            Self::HostileLane => "hostile_lane",
            Self::ParityLane => "parity_lane",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionOracleManifestRow {
    lane: ProjectionConsumptionOracleLane,
    lane_name: &'static str,
    oracle_owner_module: &'static str,
    source_artifacts_consulted: Vec<&'static str>,
    forbidden_reused_production_helpers: Vec<&'static str>,
    comparison_digest_fields: Vec<&'static str>,
    row_digest: String,
}

impl ProjectionConsumptionOracleManifestRow {
    pub fn lane(&self) -> ProjectionConsumptionOracleLane {
        self.lane
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionOracleComparisonRow {
    lane: ProjectionConsumptionOracleLane,
    expected_digest: String,
    actual_digest: String,
    row_digest: String,
}

impl ProjectionConsumptionOracleComparisonRow {
    pub fn lane(&self) -> ProjectionConsumptionOracleLane {
        self.lane
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionOracleReport {
    manifest_rows: Vec<ProjectionConsumptionOracleManifestRow>,
    comparison_rows: Vec<ProjectionConsumptionOracleComparisonRow>,
    manifest_digest: String,
    oracle_digest: String,
}

impl ProjectionConsumptionOracleReport {
    pub fn manifest_rows(&self) -> &[ProjectionConsumptionOracleManifestRow] {
        &self.manifest_rows
    }

    pub fn comparison_rows(&self) -> &[ProjectionConsumptionOracleComparisonRow] {
        &self.comparison_rows
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn oracle_digest(&self) -> &str {
        &self.oracle_digest
    }
}

pub fn projection_consumption_oracle_report() -> ProjectionConsumptionOracleReport {
    let control = control_row_set_lifecycle(2);
    let parity = parity_row_set_lifecycle(2);
    let grouped = grouped_worth_lifecycle(3);
    let manifest_rows = vec![
        manifest_row(
            ProjectionConsumptionOracleLane::ControlLane,
            "read_backed_identity_and_display_control",
            "projection_consumption::certification::oracles::row_set_control_oracle",
            vec!["RelationalAuthoritativeRowSetArtifact"],
            vec![
                "extract_from_relational_row_set",
                "ConsumedProjectionFactSet::new",
            ],
            vec![
                "declaration_digest",
                "contract_digest",
                "semantic_fact_digest",
            ],
        ),
        manifest_row(
            ProjectionConsumptionOracleLane::HostileLane,
            "grouped_worth_membership_and_endpoint_hostile",
            "projection_consumption::certification::oracles::grouped_worth_oracle",
            vec!["RelationalGroupedProjectionArtifact"],
            vec![
                "extract_from_relational_grouped_projection",
                "ConsumedProjectionFactSet::new",
            ],
            vec![
                "contract_digest",
                "semantic_fact_digest",
                "counter_snapshot",
            ],
        ),
        manifest_row(
            ProjectionConsumptionOracleLane::ParityLane,
            "equivalent_row_set_replay_parity",
            "projection_consumption::certification::oracles::row_set_parity_oracle",
            vec!["RelationalAuthoritativeRowSetArtifact"],
            vec![
                "extract_from_relational_row_set",
                "ConsumedProjectionFactSet::new",
            ],
            vec![
                "declaration_digest",
                "contract_digest",
                "fact_set_digest",
                "receipt_digest",
            ],
        ),
    ];
    let comparison_rows = vec![
        comparison_row(
            ProjectionConsumptionOracleLane::ControlLane,
            row_set_control_expected_digest(2),
            row_set_control_actual_digest(control.facts()),
        ),
        comparison_row(
            ProjectionConsumptionOracleLane::HostileLane,
            grouped_worth_expected_digest(3),
            grouped_worth_actual_digest(grouped.facts()),
        ),
        comparison_row(
            ProjectionConsumptionOracleLane::ParityLane,
            hash_parts(&[
                control.declaration().declaration_digest().to_string(),
                control.contract().contract_digest().to_string(),
                control.facts().fact_set_digest().to_string(),
                control.receipt().receipt_digest().to_string(),
            ]),
            hash_parts(&[
                parity.declaration().declaration_digest().to_string(),
                parity.contract().contract_digest().to_string(),
                parity.facts().fact_set_digest().to_string(),
                parity.receipt().receipt_digest().to_string(),
            ]),
        ),
    ];
    let manifest_digest = hash_parts(
        &manifest_rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    let oracle_digest = hash_parts(
        &comparison_rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(std::iter::once(format!("manifest:{manifest_digest}")))
            .collect::<Vec<_>>(),
    );
    ProjectionConsumptionOracleReport {
        manifest_rows,
        comparison_rows,
        manifest_digest,
        oracle_digest,
    }
}

fn manifest_row(
    lane: ProjectionConsumptionOracleLane,
    lane_name: &'static str,
    oracle_owner_module: &'static str,
    source_artifacts_consulted: Vec<&'static str>,
    forbidden_reused_production_helpers: Vec<&'static str>,
    comparison_digest_fields: Vec<&'static str>,
) -> ProjectionConsumptionOracleManifestRow {
    let row_digest = hash_parts(&[
        "projection_consumption_oracle_manifest_row_v1".to_string(),
        format!("lane:{}", lane.as_str()),
        format!("lane_name:{lane_name}"),
        format!("owner:{oracle_owner_module}"),
        format!("sources:{}", source_artifacts_consulted.join(",")),
        format!(
            "forbidden_helpers:{}",
            forbidden_reused_production_helpers.join(",")
        ),
        format!("fields:{}", comparison_digest_fields.join(",")),
    ]);
    ProjectionConsumptionOracleManifestRow {
        lane,
        lane_name,
        oracle_owner_module,
        source_artifacts_consulted,
        forbidden_reused_production_helpers,
        comparison_digest_fields,
        row_digest,
    }
}

fn comparison_row(
    lane: ProjectionConsumptionOracleLane,
    expected_digest: String,
    actual_digest: String,
) -> ProjectionConsumptionOracleComparisonRow {
    let row_digest = hash_parts(&[
        "projection_consumption_oracle_comparison_row_v1".to_string(),
        format!("lane:{}", lane.as_str()),
        format!("expected:{expected_digest}"),
        format!("actual:{actual_digest}"),
        format!("match:{}", expected_digest == actual_digest),
    ]);
    ProjectionConsumptionOracleComparisonRow {
        lane,
        expected_digest,
        actual_digest,
        row_digest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_report_binds_control_hostile_and_parity_lanes() {
        let report = projection_consumption_oracle_report();
        assert_eq!(report.manifest_rows().len(), 3);
        assert_eq!(report.comparison_rows().len(), 3);
        assert!(report
            .comparison_rows()
            .iter()
            .all(|row| !row.row_digest().is_empty()));
        assert!(!report.manifest_digest().is_empty());
        assert!(!report.oracle_digest().is_empty());
    }

    #[test]
    fn parity_lane_normalizes_distinct_authoring_path_to_same_proof_outputs() {
        let control = control_row_set_lifecycle(2);
        let parity = parity_row_set_lifecycle(2);
        assert_eq!(
            control.declaration().declaration_digest(),
            parity.declaration().declaration_digest()
        );
        assert_eq!(
            control.contract().contract_digest(),
            parity.contract().contract_digest()
        );
        assert_eq!(
            control.facts().fact_set_digest(),
            parity.facts().fact_set_digest()
        );
        assert_eq!(
            control.receipt().receipt_digest(),
            parity.receipt().receipt_digest()
        );
    }
}
