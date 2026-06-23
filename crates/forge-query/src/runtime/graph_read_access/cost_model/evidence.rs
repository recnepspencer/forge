use super::ForgeQueryGraphReadCostEstimateStatus;
use crate::runtime::ForgeQueryGraphReadAccessRequirementSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadCostEvidence {
    status: ForgeQueryGraphReadCostEstimateStatus,
    relation_statistic_count: usize,
    missing_relation_statistic_count: usize,
    requirement_row_count: usize,
}

impl ForgeQueryGraphReadCostEvidence {
    pub fn unknown_conservative() -> Self {
        Self {
            status: ForgeQueryGraphReadCostEstimateStatus::unknown_conservative(),
            relation_statistic_count: 0,
            missing_relation_statistic_count: 0,
            requirement_row_count: 0,
        }
    }

    pub fn status(&self) -> &ForgeQueryGraphReadCostEstimateStatus {
        &self.status
    }

    pub fn relation_statistic_count(&self) -> usize {
        self.relation_statistic_count
    }

    pub fn missing_relation_statistic_count(&self) -> usize {
        self.missing_relation_statistic_count
    }

    pub fn requirement_row_count(&self) -> usize {
        self.requirement_row_count
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "evidence:{}:relation_statistics:{}:missing_relation_statistics:{}:requirement_rows:{}",
            self.status.as_str(),
            self.relation_statistic_count,
            self.missing_relation_statistic_count,
            self.requirement_row_count
        )
    }

    fn derived_unknown_conservative(
        missing_relation_statistic_count: usize,
        requirement_row_count: usize,
    ) -> Self {
        Self {
            status: ForgeQueryGraphReadCostEstimateStatus::unknown_conservative(),
            relation_statistic_count: 0,
            missing_relation_statistic_count,
            requirement_row_count,
        }
    }
}

pub fn derive_graph_read_cost_evidence(
    requirements: &ForgeQueryGraphReadAccessRequirementSet,
) -> ForgeQueryGraphReadCostEvidence {
    let missing_relation_statistic_count = requirements
        .rows()
        .iter()
        .filter(|row| row.relation_name().is_some())
        .count();
    ForgeQueryGraphReadCostEvidence::derived_unknown_conservative(
        missing_relation_statistic_count,
        requirements.rows().len(),
    )
}
