use super::ForgeQueryGraphReadCostEstimateStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadCostEvidence {
    status: ForgeQueryGraphReadCostEstimateStatus,
    relation_statistic_count: usize,
}

impl ForgeQueryGraphReadCostEvidence {
    pub fn unknown_conservative() -> Self {
        Self {
            status: ForgeQueryGraphReadCostEstimateStatus::unknown_conservative(),
            relation_statistic_count: 0,
        }
    }

    pub fn status(&self) -> &ForgeQueryGraphReadCostEstimateStatus {
        &self.status
    }

    pub fn relation_statistic_count(&self) -> usize {
        self.relation_statistic_count
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "evidence:{}:relation_statistics:{}",
            self.status.as_str(),
            self.relation_statistic_count
        )
    }
}
