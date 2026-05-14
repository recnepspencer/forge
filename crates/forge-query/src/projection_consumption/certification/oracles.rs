use crate::identity::hash_parts;
use serde_json::Value;

use super::fixtures::{
    certification_grouped_projection, certification_row_set, control_row_set_lifecycle,
    grouped_worth_lifecycle, parity_row_set_lifecycle,
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

fn row_set_control_expected_digest(row_count: usize) -> String {
    let row_set = certification_row_set(row_count);
    hash_parts(
        &row_set
            .rows()
            .iter()
            .flat_map(|row| {
                let entity_identity = row
                    .fields()
                    .iter()
                    .find_map(|(field, value)| {
                        (field.as_str() == "identity.id").then(|| canonical_json(value.value()))
                    })
                    .expect("identity.id should exist");
                let display_name = row
                    .fields()
                    .iter()
                    .find_map(|(field, value)| {
                        (field.as_str() == "profile.display_name")
                            .then(|| canonical_json(value.value()))
                    })
                    .expect("display name should exist");
                [
                    format!(
                        "entity_identity:{}:{}",
                        row.row_identity().as_str(),
                        entity_identity
                    ),
                    format!(
                        "display_field:{}:profile.display_name:{}",
                        row.row_identity().as_str(),
                        display_name
                    ),
                ]
            })
            .collect::<Vec<_>>(),
    )
}

fn row_set_control_actual_digest(
    facts: &crate::projection_consumption::ConsumedProjectionFactSet,
) -> String {
    hash_parts(
        &facts
            .entity_identities()
            .iter()
            .map(|fact| {
                format!(
                    "entity_identity:{}:{}",
                    fact.source_row_identity(),
                    fact.entity_identity()
                )
            })
            .chain(facts.display_fields().iter().map(|fact| {
                format!(
                    "display_field:{}:{}:{}",
                    fact.source_row_identity(),
                    fact.field_key(),
                    canonical_json(fact.value())
                )
            }))
            .collect::<Vec<_>>(),
    )
}

fn grouped_worth_expected_digest(row_count: usize) -> String {
    let grouped = certification_grouped_projection(row_count);
    hash_parts(
        &grouped
            .members()
            .iter()
            .flat_map(|member| {
                [
                    format!(
                        "membership:{}:{}:{}",
                        member.row_identity().as_str(),
                        grouped.contract().grouping_aspect(),
                        canonical_json(member.grouping_value().value())
                    ),
                    format!(
                        "relation_endpoint:{}:{}:{}",
                        member.row_identity().as_str(),
                        grouped.contract().grouping_aspect(),
                        canonical_json(member.grouping_value().value())
                    ),
                    format!(
                        "view_local_identity:{}:{}",
                        member.row_identity().as_str(),
                        member.row_identity().as_str()
                    ),
                ]
            })
            .collect::<Vec<_>>(),
    )
}

fn grouped_worth_actual_digest(
    facts: &crate::projection_consumption::ConsumedProjectionFactSet,
) -> String {
    hash_parts(
        &facts
            .memberships()
            .iter()
            .map(|fact| {
                format!(
                    "membership:{}:{}:{}",
                    fact.source_row_identity(),
                    fact.grouping_aspect(),
                    canonical_json(fact.grouping_value())
                )
            })
            .chain(facts.relation_endpoints().iter().map(|fact| {
                format!(
                    "relation_endpoint:{}:{}:{}",
                    fact.source_row_identity().unwrap_or("none"),
                    fact.grouping_aspect().unwrap_or("none"),
                    canonical_json(fact.grouping_value().unwrap_or(&Value::Null))
                )
            }))
            .chain(facts.view_local_identities().iter().map(|fact| {
                format!(
                    "view_local_identity:{}:{}",
                    fact.source_row_identity(),
                    fact.view_local_identity()
                )
            }))
            .collect::<Vec<_>>(),
    )
}

fn canonical_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<invalid-json>".to_string())
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
